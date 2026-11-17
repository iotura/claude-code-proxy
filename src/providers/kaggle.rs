use async_trait::async_trait;
use axum::{
    body::Body,
    http::{HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
};
use std::time::Duration;

use crate::{
    anthropic::{
        json_error,
        schema::{CountTokensResponse, MessagesRequest},
    },
    provider::{CliHandlers, Provider, RequestContext},
    providers::kimi::count_tokens,
};

const DEFAULT_MODELS: &[&str] = &[
    "anthropic/claude-sonnet-5@default",
];

pub const MODEL_PREFIX: &str = "kaggle/";

pub fn advertised_models() -> Vec<String> {
    std::env::var("CCP_KAGGLE_MODELS")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|model| !model.is_empty())
                .map(|model| format!("{MODEL_PREFIX}{model}"))
                .collect()
        })
        .unwrap_or_else(|| {
            DEFAULT_MODELS
                .iter()
                .map(|model| format!("{MODEL_PREFIX}{model}"))
                .collect()
        })
}

pub struct KaggleProvider {
    client: reqwest::Client,
    base_url: String,
    api_key: Option<String>,
}

impl KaggleProvider {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .connect_timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        let base_url = std::env::var("MODEL_PROXY_URL")
            .unwrap_or_else(|_| "https://mp-staging.kaggle.net/models".to_string());
        let api_key = std::env::var("MODEL_PROXY_API_KEY")
            .ok()
            .filter(|value| !value.trim().is_empty());
        Self {
            client,
            base_url,
            api_key,
        }
    }

    fn endpoint(&self) -> String {
        format!(
            "{}/anthropic/v1/messages",
            self.base_url.trim_end_matches('/')
        )
    }

    async fn forward(&self, request: &MessagesRequest) -> Response {
        let Some(api_key) = self.api_key.as_deref() else {
            return json_error(
                StatusCode::UNAUTHORIZED,
                "authentication_error",
                "Kaggle Model Proxy credentials are missing; set MODEL_PROXY_API_KEY",
            );
        };
        let api_key = match HeaderValue::from_str(api_key) {
            Ok(value) => value,
            Err(_) => {
                return json_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "api_error",
                    "Invalid Kaggle API key header",
                );
            }
        };
        let upstream = match self
            .client
            .post(self.endpoint())
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header(header::CONTENT_TYPE, "application/json")
            .json(request)
            .send()
            .await
        {
            Ok(response) => response,
            Err(error) => {
                return json_error(
                    StatusCode::BAD_GATEWAY,
                    "api_error",
                    format!("Kaggle Model Proxy request failed: {error}"),
                );
            }
        };
        let status = upstream.status();
        let mut response = Response::builder().status(status);
        for name in [
            header::CONTENT_TYPE,
            header::RETRY_AFTER,
            header::HeaderName::from_static("x-request-id"),
        ] {
            if let Some(value) = upstream.headers().get(&name).cloned() {
                response = response.header(name, value);
            }
        }
        match response.body(Body::from_stream(upstream.bytes_stream())) {
            Ok(response) => response,
            Err(_) => json_error(StatusCode::BAD_GATEWAY, "api_error", "Failed to relay Kaggle response"),
        }
    }
}

impl Default for KaggleProvider {
    fn default() -> Self {
        Self::new()
    }
}

struct KaggleCli;
impl CliHandlers for KaggleCli {
    fn login(&self) -> anyhow::Result<()> {
        anyhow::bail!("Use `kaggle benchmarks auth` to refresh Model Proxy credentials")
    }
    fn device(&self) -> anyhow::Result<()> {
        self.login()
    }
    fn status(&self) -> anyhow::Result<()> {
        if std::env::var("MODEL_PROXY_API_KEY")
            .is_ok_and(|value| !value.trim().is_empty())
        {
            println!("Kaggle Model Proxy credential is configured");
        } else {
            println!("Kaggle Model Proxy credential is missing");
        }
        Ok(())
    }
    fn logout(&self) -> anyhow::Result<()> {
        println!("Remove MODEL_PROXY_API_KEY from the environment to disconnect Kaggle");
        Ok(())
    }
}
static KAGGLE_CLI: KaggleCli = KaggleCli;

#[async_trait]
impl Provider for KaggleProvider {
    fn name(&self) -> &'static str {
        "kaggle"
    }
    fn supported_models(&self) -> Vec<String> {
        advertised_models()
    }
    fn cli(&self) -> &'static dyn CliHandlers {
        &KAGGLE_CLI
    }

    async fn handle_messages(&self, mut body: MessagesRequest, _ctx: RequestContext) -> Response {
        if let Some(model) = body.model.as_deref().and_then(|model| model.strip_prefix(MODEL_PREFIX)) {
            body.model = Some(model.to_string());
        }
        self.forward(&body).await
    }

    async fn handle_count_tokens(&self, body: MessagesRequest, _ctx: RequestContext) -> Response {
        (
            StatusCode::OK,
            axum::Json(CountTokensResponse {
                input_tokens: count_tokens::count_tokens(&body),
            }),
        )
            .into_response()
    }
}
