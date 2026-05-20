use serde::Deserialize;
use std::collections::HashMap;
use zed_extension_api::{
    self as zed,
    serde_json,
    settings::ContextServerSettings,
    ContextServerId, Project, Result,
};

const PACKAGE_NAME: &str = "mcp-remote";
const SERVER_ID: &str = "mcp-server-signoz";

/// User-tunable settings, read from Zed's `context_servers.mcp-server-signoz.settings`.
///
/// All fields are optional. The most common setup is just `region = "us"`, which
/// resolves to https://mcp.us.signoz.cloud/mcp and lets the user complete OAuth in
/// the browser the first time they invoke the server.
#[derive(Debug, Default, Deserialize)]
struct SigNozSettings {
    /// SigNoz Cloud region. One of: us, us2, eu, eu2, in, in2.
    /// Ignored if `url` is set.
    #[serde(default)]
    region: Option<String>,

    /// Full MCP endpoint URL. Overrides `region` if set. Use this for a
    /// self-hosted SigNoz instance running the MCP server in HTTP transport mode.
    /// Example: "https://mcp.example.com/mcp" or "http://localhost:8000/mcp".
    #[serde(default)]
    url: Option<String>,

    /// Optional bearer token for clients that can't do OAuth (e.g. headless setups).
    /// When set, mcp-remote forwards it as `Authorization: Bearer <token>`.
    /// For SigNoz Cloud this should be a Service Account API key.
    #[serde(default)]
    api_key: Option<String>,

    /// Extra `--header "Name:Value"` pairs to forward through mcp-remote.
    /// Useful for tenant slugs, region overrides, etc.
    #[serde(default)]
    headers: HashMap<String, String>,
}

struct SigNozMcpExtension;

impl zed::Extension for SigNozMcpExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<zed::Command> {
        // Defensive: this extension only declares one server, but match by id anyway.
        let id: &str = context_server_id.as_ref();
        if id != SERVER_ID {
            return Err(format!("unknown context server id: {id}"));
        }

        // Pull user settings out of Zed's settings.json. If the user hasn't
        // configured anything, defaults give us the SigNoz US Cloud endpoint.
        let raw = ContextServerSettings::for_project(SERVER_ID, project)?;
        let settings: SigNozSettings = match raw.settings {
            Some(value) => serde_json::from_value(value)
                .map_err(|e| format!("invalid SigNoz MCP settings: {e}"))?,
            None => SigNozSettings::default(),
        };

        let endpoint = resolve_endpoint(&settings)?;

        // Build the argv for `npx -y mcp-remote <url> [--header ...] [--header ...]`.
        // mcp-remote is a tiny stdio<->HTTP shim maintained by the MCP community.
        // It handles the OAuth dance for hosted SigNoz Cloud automatically (popping
        // open a browser tab on first use and caching tokens locally under ~/.mcp-auth).
        let mut args = vec!["-y".to_string(), PACKAGE_NAME.to_string(), endpoint];

        if let Some(token) = settings.api_key.as_deref().filter(|s| !s.is_empty()) {
            args.push("--header".to_string());
            args.push(format!("Authorization:Bearer {token}"));
        }

        for (name, value) in &settings.headers {
            if name.is_empty() {
                continue;
            }
            args.push("--header".to_string());
            args.push(format!("{name}:{value}"));
        }

        Ok(zed::Command {
            command: "npx".to_string(),
            args,
            env: vec![],
        })
    }
}

/// Pick the MCP endpoint based on user settings, with sensible defaults.
fn resolve_endpoint(settings: &SigNozSettings) -> Result<String> {
    if let Some(url) = settings.url.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        return Ok(url.to_string());
    }

    let region = settings
        .region
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("us");

    // Validate against the published SigNoz Cloud regions to give a clear error
    // rather than letting it fail later inside mcp-remote with a 404.
    match region {
        "us" | "us2" | "eu" | "eu2" | "in" | "in2" => {
            Ok(format!("https://mcp.{region}.signoz.cloud/mcp"))
        }
        other => Err(format!(
            "unknown SigNoz Cloud region {other:?}; expected one of: us, us2, eu, eu2, in, in2. \
             For a self-hosted SigNoz, set `url` to your MCP endpoint instead."
        )),
    }
}

zed::register_extension!(SigNozMcpExtension);
