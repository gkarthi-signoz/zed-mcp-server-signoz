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

For Zed 1.8+ native HTTP MCP, installing this extension is optional. Use the
extension when you want the packaged `mcp-remote` bridge or region-based Cloud
defaults.

## Setup

### Recommended: Zed 1.8+ native HTTP MCP

Recent Zed versions can connect to HTTP MCP servers directly. This is the
recommended setup for both SigNoz Cloud and self-hosted SigNoz because it avoids
an extra stdio bridge process.

For SigNoz Cloud, add this to your Zed `settings.json`:

```json
{
  "context_servers": {
    "signoz": {
      "enabled": true,
      "url": "https://mcp.us.signoz.cloud/mcp"
    }
  }
}
```

Replace `us` with your SigNoz Cloud region: `us`, `us2`, `eu`, `eu2`, `in`, or
`in2`. Match the region shown under **Settings → Ingestion** in SigNoz.

When prompted, complete the SigNoz Cloud authentication flow. You will need your
SigNoz instance URL and a Service Account API key from **Settings → Service
Accounts**.

### Self-hosted SigNoz

Start the SigNoz MCP server in HTTP mode, then point Zed at its `/mcp` endpoint:

```json
{
  "context_servers": {
    "signoz": {
      "enabled": true,
      "url": "http://localhost:8000/mcp",
      "headers": {
        "Accept": "application/json, text/event-stream"
      },
      "timeout": 60000
    }
  }
}
```

Do not use Zed's **Add Server** button for this extension while testing local
self-hosted mode; it may create a manual empty entry. Edit `settings.json`
directly.

### Extension bridge mode

The extension can also launch [`mcp-remote`](https://www.npmjs.com/package/mcp-remote)
for clients or Zed versions that expect stdio MCP servers. This path requires
**Node.js 18+** with `npx` on your `PATH`.

For SigNoz Cloud via the extension:

```json
{
  "context_servers": {
    "mcp-server-signoz": {
      "source": "extension",
      "enabled": true,
      "settings": {
        "region": "us"
      }
    }
  }
}
```

For a custom or self-hosted HTTP MCP endpoint via the extension:

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
    "signoz": {
      "enabled": true,
      "url": "https://mcp.us.signoz.cloud/mcp",
      "headers": {
        "SIGNOZ-API-KEY": "YOUR_SIGNOZ_API_KEY",
        "X-SigNoz-URL": "https://your-instance.signoz.cloud"
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

Apache-2.0. See [LICENSE](./LICENSE).

This is the official SigNoz extension for Zed.
