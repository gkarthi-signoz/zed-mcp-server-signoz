# SigNoz MCP Server

This extension connects Zed's Agent Panel to your SigNoz observability data via the
[SigNoz MCP Server](https://signoz.io/docs/ai/signoz-mcp-server/). It uses the
hosted SigNoz Cloud endpoint by default — no binary to install.

## Requirements

- **Node.js 18+** with `npx` on your `PATH`. The extension launches `npx mcp-remote`
  as a stdio↔HTTP bridge to SigNoz Cloud.
- A **SigNoz Cloud** account, or a self-hosted SigNoz instance running the MCP
  server in HTTP transport mode.

## Configuration

Open your Zed `settings.json` (`zed: open settings`) and add:

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

### Picking your region

Set `region` to match where your SigNoz Cloud account lives. Find it under
**Settings → Ingestion** in the SigNoz UI.

| Region | Endpoint                              |
| ------ | ------------------------------------- |
| `us`   | `https://mcp.us.signoz.cloud/mcp`     |
| `eu`   | `https://mcp.eu.signoz.cloud/mcp`     |
| `in`   | `https://mcp.in.signoz.cloud/mcp`     |

Using the wrong region will fail authentication.

### First-run authentication

The first time the agent calls a SigNoz tool, `mcp-remote` opens a browser tab to
complete the OAuth flow. You'll be asked for your SigNoz instance URL and a
Service Account API key (create one under **Settings → Service Accounts**).
Tokens are cached locally in `~/.mcp-auth/` and refreshed automatically.

### Self-hosted SigNoz

Point at any HTTP `/mcp` endpoint by setting `url` instead of `region`:

```json
{
  "context_servers": {
    "mcp-server-signoz": {
      "source": "extension",
      "settings": {
        "url": "http://localhost:8000/mcp"
      }
    }
  }
}
```

### Headless / non-OAuth setups

If you can't use the OAuth flow, pass a SigNoz Service Account API key directly
and it'll be sent as `Authorization: Bearer …`:

```json
{
  "context_servers": {
    "mcp-server-signoz": {
      "source": "extension",
      "settings": {
        "region": "us",
        "api_key": "YOUR_SIGNOZ_API_KEY"
      }
    }
  }
}
```

For arbitrary extra headers (tenant slugs, custom proxies):

```json
{
  "settings": {
    "url": "https://mcp.example.com/mcp",
    "headers": {
      "X-Tenant": "team-acme"
    }
  }
}
```

## Verifying it works

Open the Agent Panel and check the indicator next to **SigNoz MCP Server** in the
settings view. Green = server is active. If it stays red, run `zed: open log` and
look for `mcp-remote` output. The most common failures are: `npx` not on `PATH`,
a wrong `region`, or an expired SigNoz API key.

## What you can ask

Once it's running, prompts like these work in the Agent Panel:

- *"List all firing alerts in the last hour"*
- *"Show me the top 5 slowest operations in the `checkout` service"*
- *"Search logs for errors from `payments-api` in the last 30 minutes"*
- *"What's the p95 latency on `/api/orders`?"*
- *"Create a dashboard for the checkout service"*

See the [SigNoz MCP use cases guide](https://signoz.io/docs/ai/use-cases/) for
more.
