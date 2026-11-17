---
title: Kaggle Model Proxy
description: Use Kaggle's Anthropic-compatible Model Proxy endpoint from Claude Code.
---

The Kaggle provider forwards Claude Code Messages API requests to Kaggle's
Anthropic-compatible endpoint. Kaggle credentials are short-lived. The
`scripts/claude-kaggle` launcher refreshes the key using your existing Kaggle
CLI login when it needs to start a local proxy, and reads the resulting
environment file without adding credentials to the repository.

From your project directory, launch or resume Claude Code:

```sh
scripts/claude-kaggle --continue
```

If no proxy server is running, this starts a temporary local server and stops
it when Claude Code exits. An already-running server is reused and left up.
For a long-running server shared across sessions, run `scripts/serve-kaggle`
separately and stop it with Ctrl+C when done.

The default and only model advertised by default is Kaggle Claude Sonnet 5:
`kaggle/anthropic/claude-sonnet-5@default`. Other models can be added to the
`CCP_KAGGLE_MODELS` allowlist after checking that their tool and reasoning
formats are compatible with Claude Code. The model name follows the Kaggle
Model Proxy catalog associated with your credential. A `kaggle/` prefix selects
the Kaggle provider and is removed before the request is sent.

To return to your normal Claude Code account or API configuration, run
`scripts/claude-official`. That command removes the local proxy URL and model
overrides for that process.

The Kaggle inference credential expires and consumes the account's Model Proxy
quota. Kaggle's short-lived key is refreshed whenever `scripts/serve-kaggle`
starts. The proxy does not print or persist the key in the repository.
