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

## Setup

You need **Node.js 18+** with `npx` on your `PATH` — the extension shells out to
`npx mcp-remote` to bridge stdio to SigNoz Cloud's HTTP MCP endpoint.

Add this to your Zed `settings.json`:

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

Valid regions: `us`, `us2`, `eu`, `eu2`, `in`, `in2`. Match the region of your
SigNoz Cloud account (Settings → Ingestion in the SigNoz UI).

On first tool call, `mcp-remote` opens a browser tab for the SigNoz OAuth flow.
You'll need a SigNoz Service Account API key — create one under **Settings →
Service Accounts** in SigNoz.

### Self-hosted SigNoz

Point at any HTTP MCP endpoint with `url`:

```json
{
  "context_servers": {
    "mcp-server-signoz": {
      "settings": {
        "url": "http://localhost:8000/mcp"
      }
    }
  }
}
```

### Non-OAuth (headless) setup

Pass the API key directly:

```json
{
  "context_servers": {
    "mcp-server-signoz": {
      "settings": {
        "region": "us",
        "api_key": "YOUR_SIGNOZ_API_KEY"
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
rustup target add wasm32-wasip1

# Build
cargo build --release --target wasm32-wasip1

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
