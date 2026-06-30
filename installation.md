# SigNoz MCP Server

This extension connects Zed's Agent Panel to your SigNoz observability data via the
[SigNoz MCP Server](https://signoz.io/docs/ai/signoz-mcp-server/). It uses the
packaged `mcp-remote` bridge to connect Zed's stdio MCP support to SigNoz's
HTTP MCP endpoint.

## Requirements

- **Node.js 18+** with `npx` on your `PATH`. The extension launches
  `npx mcp-remote`.
- A **SigNoz Cloud** account, or a self-hosted SigNoz instance running the MCP
  server in HTTP transport mode.

## Configuration

After installing the extension, enable **SigNoz MCP Server** from the Agent
Panel settings. The default Cloud region is `us`.

### SigNoz Cloud

For a different SigNoz Cloud region, set `region`:

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

Replace `in2` with the region where your SigNoz Cloud account lives. Find it
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

### Self-hosted HTTP mode

For self-hosted SigNoz, set `url` to your MCP endpoint:

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

The extension passes `--transport http-only`, `Accept: application/json,
text/event-stream`, and `--allow-http` for local HTTP URLs when launching
`mcp-remote`.

### First-run authentication for Cloud

On the first connection, `mcp-remote` opens a browser tab for SigNoz Cloud
login. Complete it promptly — if it takes too long, Zed cancels the connection
with a startup timeout. If that happens, just enable the server again: the token
is cached under `~/.mcp-auth`, so the next connection completes instantly.

To skip the browser flow entirely, use header-based authentication below.

`url` overrides `region`.

### Header-based authentication

If you can't use OAuth, pass SigNoz Cloud headers directly:

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

For arbitrary extra headers:

```json
{
  "context_servers": {
    "mcp-server-signoz": {
      "source": "extension",
      "enabled": true,
      "settings": {
        "url": "https://mcp.example.com/mcp",
        "headers": {
          "X-Tenant": "team-acme"
        }
      }
    }
  }
}
```

## Verifying it works

Open the Agent Panel and check the indicator next to **SigNoz MCP Server** in the
settings view. Green = server is active. If it stays red, run `zed: open log`.
Check that the `url` is reachable and the region is correct. Also look for
`mcp-remote` output; common failures are `npx` not on `PATH`, a wrong `region`,
or an expired SigNoz API key.

## What you can ask

Once it's running, prompts like these work in the Agent Panel:

- *"List all firing alerts in the last hour"*
- *"Show me the top 5 slowest operations in the `checkout` service"*
- *"Search logs for errors from `payments-api` in the last 30 minutes"*
- *"What's the p95 latency on `/api/orders`?"*
- *"Create a dashboard for the checkout service"*

See the [SigNoz MCP use cases guide](https://signoz.io/docs/ai/use-cases/) for
more.
