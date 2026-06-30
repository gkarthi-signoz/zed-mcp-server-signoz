# SigNoz MCP Server for Zed

A [Zed](https://zed.dev) extension that connects the Agent Panel to your
[SigNoz](https://signoz.io) observability data via the
[SigNoz MCP Server](https://signoz.io/docs/ai/signoz-mcp-server/).

Query metrics, traces, logs, alerts, dashboards, and services using natural
language, directly from your editor.

## What is SigNoz?

SigNoz is an open-source observability platform — metrics, traces, logs, alerts,
and dashboards in one place. The SigNoz MCP Server exposes that data over the
Model Context Protocol so AI assistants can answer questions like:

- *"Why is `/api/checkout` slow right now?"*
- *"Show me errors in `payments-api` in the last 30 minutes."*
- *"List all firing alerts and rank by severity."*
- *"Compare p95 latency on `orders` before and after yesterday's deploy."*

## Install

Open the Extension Gallery (`cmd-shift-x` / `ctrl-shift-x`) in Zed, search for
**SigNoz**, and click Install.

This extension launches the packaged [`mcp-remote`](https://www.npmjs.com/package/mcp-remote)
bridge with `npx`, so Zed can talk to SigNoz's HTTP MCP endpoint through stdio.

## Setup

### Requirements

- Node.js 18+ with `npx` on your `PATH`.
- A SigNoz Cloud account, or a self-hosted SigNoz MCP server running over HTTP.

### SigNoz Cloud

After installing the extension, enable **SigNoz MCP Server** from the Agent
Panel settings. The default Cloud region is `us`.

For another Cloud region, configure the extension server with a `region`:

```json
{
  "context_servers": {
    "mcp-server-signoz": {
      "source": "extension",
      "enabled": true,
      "settings": {
        "region": "in2"
      }
    }
  }
}
```

Supported regions: `us`, `us2`, `eu`, `eu2`, `in`, `in2`.

On the first connection a browser tab opens for SigNoz Cloud login. Complete it
promptly — if authentication takes too long, Zed may report a startup timeout.
If that happens, just enable the server again: the token is cached, so the next
connection is instant.

### Self-hosted SigNoz

Start the SigNoz MCP server in HTTP mode, then set `url` to its `/mcp` endpoint:

```json
{
  "context_servers": {
    "mcp-server-signoz": {
      "source": "extension",
      "enabled": true,
      "settings": {
        "url": "http://localhost:8000/mcp"
      }
    }
  }
}
```

`url` overrides `region`. The extension passes `--transport http-only`,
`Accept: application/json, text/event-stream`, and `--allow-http` for local HTTP
URLs when launching `mcp-remote`.

### Header-based authentication

For clients or environments that cannot complete OAuth, pass SigNoz Cloud
headers directly:

```json
{
  "context_servers": {
    "mcp-server-signoz": {
      "source": "extension",
      "enabled": true,
      "settings": {
        "region": "us",
        "api_key": "YOUR_SIGNOZ_API_KEY",
        "signoz_url": "https://your-instance.signoz.cloud"
      }
    }
  }
}
```

## Available tools

The SigNoz MCP server exposes ~30 tools covering metrics, logs, traces, alerts,
dashboards, notification channels, and saved views. See the
[full tool reference on GitHub](https://github.com/SigNoz/signoz-mcp-server) for
parameter details.

Mentioning "SigNoz" in your prompt helps the model pick from the right toolset.

## Development

This is a standard Zed Rust/WASM extension.

```bash
# One-time: install Rust via rustup (NOT homebrew — Zed dev extensions require rustup)
rustup target add wasm32-wasip2

# Build
cargo build --release --target wasm32-wasip2

# Install as a dev extension
# In Zed: Extensions → Install Dev Extension → select this folder
```

If you already have the published extension installed, the dev extension will
override it.

## Resources

- [SigNoz MCP Server docs](https://signoz.io/docs/ai/signoz-mcp-server/)
- [SigNoz MCP Server source](https://github.com/SigNoz/signoz-mcp-server)
- [Zed MCP extensions docs](https://zed.dev/docs/extensions/mcp-extensions)
- [mcp-remote](https://www.npmjs.com/package/mcp-remote) — the stdio↔HTTP bridge

## License

MIT. See [LICENSE](./LICENSE).

This is the official SigNoz extension for Zed.
