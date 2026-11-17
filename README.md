# claude-code-proxy

Claude Code, powered by **OpenAI Codex**, **Kimi**, **Grok**, **OpenCode Go**,
**Kaggle Model Proxy**, or **Cursor Agent**.

Docs: <https://claude-code-proxy.raine.dev>

LLM docs: <https://claude-code-proxy.raine.dev/llms.txt>

<img src="meta/claude-code-screenshot-2026-07.webp" alt="Claude Code running through claude-code-proxy" />

<a href="https://aventasks.dev/">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://aventasks.dev/readme/aven-banner-dark.webp">
    <img src="https://aventasks.dev/readme/aven-banner-light.webp" alt="aven: one task queue for you and your coding agents" width="100%">
  </picture>
</a>

<sub>Also from the author of claude-code-proxy: <a href="https://aventasks.dev/"><b>aven</b></a>, one
queue for every project, with a keyboard-driven TUI for you and a CLI for your
agents.</sub>

## Why?

Claude Code remains an excellent coding harness, with strong tools, skills,
hooks, subagents, and editor integrations. claude-code-proxy keeps that client
experience while translating its Anthropic API traffic for subscription-backed
provider services.

One local process handles provider authentication, model-based routing,
protocol translation, streaming responses, and diagnostics. The built-in
monitor shows sessions, active and recent requests, errors, token usage, and
throughput.

## Quick start with Codex

Install on macOS or Linux:

```sh
brew install raine/claude-code-proxy/claude-code-proxy
```

Or use the release installer:

```sh
curl -fsSL https://raw.githubusercontent.com/raine/claude-code-proxy/main/scripts/install.sh | bash
```

Windows and other prebuilt artifacts are available from
[GitHub Releases](https://github.com/raine/claude-code-proxy/releases).

Sign in with a **ChatGPT Plus or Pro account**, not an OpenAI API account:

```sh
claude-code-proxy codex auth login
```

Start the proxy in one terminal:

```sh
claude-code-proxy serve
```

For a background service, use `claude-code-proxy serve --no-monitor` and attach
from another terminal with `claude-code-proxy monitor`. Closing an attached
dashboard leaves the proxy running.

For a session-scoped Codex setup, authenticate once with
`claude-code-proxy codex auth login`, then install the launcher from the repo
root:

```sh
mkdir -p "$HOME/.local/bin"
ln -sfn "$(pwd -P)/scripts/codex-code" "$HOME/.local/bin/codex-code"
```

From any project directory, `codex-code` starts the proxy if needed and stops
the server when Claude Code exits. It reuses an already-running proxy. Resume
with `codex-code --resume <session-id>`.

Start Claude Code in another:

```sh
ANTHROPIC_BASE_URL=http://127.0.0.1:18765 \
ANTHROPIC_AUTH_TOKEN=unused \
ANTHROPIC_MODEL=gpt-6-sol[1m] \
ANTHROPIC_SMALL_FAST_MODEL=gpt-6-luna[1m] \
CLAUDE_CODE_AUTO_COMPACT_WINDOW=272000 \
CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1 \
CLAUDE_CODE_DISABLE_NONSTREAMING_FALLBACK=1 \
  claude
```

See [Getting started](https://claude-code-proxy.raine.dev/getting-started/)
for the complete first session.

Optional Codex image generation and editing can reuse the same ChatGPT login:

```sh
CCP_CODEX_IMAGES_API=1 claude-code-proxy serve
curl http://127.0.0.1:18765/v1/images/generations \
  -H 'Content-Type: application/json' \
  -d '{"prompt":"A paper-cut fox","model":"gpt-image-2"}'
```

The opt-in Images API returns base64 image data and consumes the signed-in account's image quota. Image prompts and payloads are excluded from traffic captures. See the [HTTP API](https://claude-code-proxy.raine.dev/reference/http-api/) for generation and edit schemas.

## Providers

| Provider     | Account                        | Model selection                                 |
| ------------ | ------------------------------ | ----------------------------------------------- |
| Codex        | ChatGPT Plus or Pro            | Registered `gpt-*` models, `-fast`/`-ultrafast` |
| Kimi         | kimi.com with Kimi Code access | `kimi-for-coding` and aliases                   |
| Grok         | grok.com                       | Registered Grok models                          |
| OpenCode Go  | OpenCode Go subscription       | Non-conflicting IDs and `opencode-go/<model-id>` |
| Kaggle       | Kaggle account with Benchmarks access | `kaggle/<Kaggle model slug>` |
| Cursor Agent | Cursor account                 | Cursor aliases and `cursor:<model-id>` prefixes |

### Switch between Kaggle and Anthropic

The Kaggle backend uses Kaggle's Anthropic-compatible Messages endpoint. It
needs Claude Code, the Kaggle CLI, Python 3.11+, and Rust/Cargo (the launcher
builds this proxy from source the first time it starts).

Install Rust/Cargo with [rustup](https://rustup.rs) if they are not already
installed. Install Claude Code using
[Anthropic's instructions](https://code.claude.com/docs/en/getting-started).
Install the official Kaggle CLI in an isolated Python environment with either
`pipx` or `uv`:

```sh
pipx install kaggle
# or: uv tool install kaggle
```

Authenticate the Kaggle CLI in one of these ways:

```sh
kaggle auth login
```

This opens Kaggle's browser login flow. Alternatively, create an API token in
[Kaggle account settings](https://www.kaggle.com/settings), save the token
string in `~/.kaggle/access_token`, and restrict the file permissions:

```sh
mkdir -p "$HOME/.kaggle"
chmod 700 "$HOME/.kaggle"
${EDITOR:-nano} "$HOME/.kaggle/access_token"
chmod 600 "$HOME/.kaggle/access_token"
```

Paste only the token string into that file. Never put it in the repository.
The Kaggle CLI uses this account credential to mint the short-lived Model Proxy
key used by the local server.

Install the launchers on your `PATH` from the repository root:

```sh
mkdir -p "$HOME/.local/bin"
ln -sfn "$(pwd -P)/scripts/claude-kaggle" "$HOME/.local/bin/kaggle-code"
ln -sfn "$(pwd -P)/scripts/claude-official" "$HOME/.local/bin/claude-official"
```

Make sure `$HOME/.local/bin` is on your `PATH`, then configure the Kaggle CLI
path once from this checkout:

```sh
scripts/configure-kaggle "$(command -v kaggle)"
```

This saves the Kaggle CLI path and credential-file path under
`~/.config/claude-code-proxy/`. You can rerun it if you move the repository or
Kaggle CLI. It does not copy your Kaggle account token.

From your project directory, run the installed `kaggle-code` command:

```sh
kaggle-code --continue
```

`kaggle-code` checks for a local proxy. If none is running, it refreshes the
short-lived Kaggle key through your Kaggle CLI login, starts a temporary server,
and stops that server when Claude Code exits. If a server is already running,
the command reuses it and leaves it running afterward. The key is stored in
`~/.config/claude-code-proxy/kaggle.env` with mode 0600.

For a long-running server shared by multiple Claude Code sessions, start
`scripts/serve-kaggle` from this checkout in a separate terminal. Stop it with
Ctrl+C when you are done. A `kaggle-code` process reuses this server and does
not stop it.

Each server start refreshes Kaggle's local-development settings. By default,
`kaggle-code` advertises only Kaggle Claude Sonnet 5 in the `/model` picker.
Keep this conservative default because other Kaggle models may reject Claude
Code's tool schemas or request options. You can explicitly allow additional
models with `CCP_KAGGLE_MODELS` after confirming compatibility. To view
Kaggle's full live benchmark catalog, run `scripts/kaggle-models`; that catalog
is distinct from the local Model Proxy model list.

Resume a particular session with `kaggle-code --resume <session-id>`. The
default model remains Kaggle Claude Sonnet 5; see the allowlist note above
before enabling another model.

To use your normal Anthropic setup, run
`"$CCP_PROXY_DIR/scripts/claude-official" --continue` from the same project
directory. The launch scripts keep Claude Code's global settings unchanged.
The previous command name `claude-kaggle` remains available as an alias.

Run `claude-code-proxy models` for the current catalog or
`claude-code-proxy models --full` for every dynamic Cursor alias.

> [!WARNING]
> The proxy accepts local requests without client authentication. It binds to
> `127.0.0.1` by default. Protect any non-loopback listener with a firewall or
> authenticating reverse proxy. Provider subscriptions, model access, terms,
> and account enforcement remain under each provider's control. Unofficial
> clients may carry account risk.

## Documentation

- [What is claude-code-proxy?](https://claude-code-proxy.raine.dev/)
- [Choosing a provider](https://claude-code-proxy.raine.dev/providers/choosing-a-provider/)
- [Configure Claude Code](https://claude-code-proxy.raine.dev/using/configure-claude-code/)
- [Models and routing](https://claude-code-proxy.raine.dev/using/models-and-routing/)
- [Monitor TUI](https://claude-code-proxy.raine.dev/using/monitor-tui/)
- [Troubleshooting](https://claude-code-proxy.raine.dev/using/troubleshooting/)
- [Command reference](https://claude-code-proxy.raine.dev/reference/command-reference/)
- [Configuration](https://claude-code-proxy.raine.dev/reference/configuration/)
- [HTTP API](https://claude-code-proxy.raine.dev/reference/http-api/)
- [Compatibility and limitations](https://claude-code-proxy.raine.dev/reference/compatibility-and-limitations/)
- [For coding agents](https://claude-code-proxy.raine.dev/using/for-coding-agents/)

## Related projects

- [aven](https://github.com/raine/aven): local-first task management for power
  users and agents
- [claude-history](https://github.com/raine/claude-history): search Claude Code
  conversation history from the terminal
- [git-surgeon](https://github.com/raine/git-surgeon): non-interactive
  hunk-level git staging for coding agents
- [workmux](https://github.com/raine/workmux): parallel coding tasks in git
  worktrees and tmux
- [consult-llm](https://github.com/raine/consult-llm): consult other AI models
  from an agent workflow

## License

[MIT](LICENSE)
