# SigNoz MCP Server

This extension connects Zed's Agent Panel to your SigNoz observability data via the
[SigNoz MCP Server](https://signoz.io/docs/ai/signoz-mcp-server/). It uses the
hosted SigNoz Cloud endpoint by default — no binary to install.

## Requirements

- **Zed 1.8+** can connect to HTTP MCP servers directly. This is the
  recommended path for SigNoz Cloud and self-hosted SigNoz.
- **Node.js 18+** with `npx` on your `PATH` is only required if you use the
  extension bridge mode, where the extension launches `npx mcp-remote`.
- A **SigNoz Cloud** account, or a self-hosted SigNoz instance running the MCP
  server in HTTP transport mode.

## Configuration

Open your Zed `settings.json` (`zed: open settings`) and add one of the
following configs.

### Recommended: Zed 1.8+ native HTTP MCP

For SigNoz Cloud:

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

Replace `us` with the region where your SigNoz Cloud account lives. Find it
under **Settings → Ingestion** in the SigNoz UI.

| Region | Endpoint                              |
| ------ | ------------------------------------- |
| `us`   | `https://mcp.us.signoz.cloud/mcp`     |
| `us2`  | `https://mcp.us2.signoz.cloud/mcp`    |
| `eu`   | `https://mcp.eu.signoz.cloud/mcp`     |
| `eu2`  | `https://mcp.eu2.signoz.cloud/mcp`    |
| `in`   | `https://mcp.in.signoz.cloud/mcp`     |
| `in2`  | `https://mcp.in2.signoz.cloud/mcp`    |

Using the wrong region will fail authentication.

For self-hosted HTTP mode:

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

Do not use Zed's **Add Server** button while testing local self-hosted mode; it
may create an empty manual entry. Edit `settings.json` directly.

### First-run authentication for Cloud

When prompted, complete the SigNoz Cloud authentication flow. You'll be asked
for your SigNoz instance URL and a Service Account API key (create one under
**Settings → Service Accounts**).

### Extension bridge mode

The extension can also launch `npx mcp-remote` for clients or Zed versions that
expect stdio MCP servers.

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

If you can't use OAuth, pass SigNoz Cloud headers directly:

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

For arbitrary extra headers in extension bridge mode:

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
settings view. Green = server is active. If it stays red, run `zed: open log`.
For native HTTP mode, check that the `url` is reachable and the region is
correct. For extension bridge mode, also look for `mcp-remote` output; common
failures are `npx` not on `PATH`, a wrong `region`, or an expired SigNoz API
key.

## What you can ask

Once it's running, prompts like these work in the Agent Panel:

- *"List all firing alerts in the last hour"*
- *"Show me the top 5 slowest operations in the `checkout` service"*
- *"Search logs for errors from `payments-api` in the last 30 minutes"*
- *"What's the p95 latency on `/api/orders`?"*
- *"Create a dashboard for the checkout service"*

See the [SigNoz MCP use cases guide](https://signoz.io/docs/ai/use-cases/) for
more.
