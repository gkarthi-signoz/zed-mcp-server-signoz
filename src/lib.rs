use serde::Deserialize;
use std::collections::HashMap;
use zed_extension_api::{
    self as zed, serde_json, settings::ContextServerSettings, ContextServerId, Os, Project, Result,
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

    /// Optional SigNoz API key for clients that can't do OAuth.
    /// When set, mcp-remote forwards it as `SIGNOZ-API-KEY`.
    #[serde(default)]
    api_key: Option<String>,

    /// Optional SigNoz instance URL used with `api_key`.
    /// When set, mcp-remote forwards it as `X-SigNoz-URL`.
    #[serde(default)]
    signoz_url: Option<String>,

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

        let mut args = vec![
            "-y".to_string(),
            PACKAGE_NAME.to_string(),
            endpoint.to_string(),
            "--transport".to_string(),
            "http-only".to_string(),
            "--header".to_string(),
            "Accept:application/json, text/event-stream".to_string(),
        ];

        if endpoint.starts_with("http://") {
            args.push("--allow-http".to_string());
        }

        if let Some(api_key) = settings
            .api_key
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            args.extend(["--header".to_string(), format!("SIGNOZ-API-KEY:{api_key}")]);
        }

        if let Some(signoz_url) = settings
            .signoz_url
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            args.extend(["--header".to_string(), format!("X-SigNoz-URL:{signoz_url}")]);
        }

        for (name, value) in &settings.headers {
            if name.is_empty() {
                continue;
            }
            args.extend(["--header".to_string(), format!("{name}:{value}")]);
        }

        // Zed passes the user's login-shell environment (including a Node version
        // manager's `PATH`) to the *child* process, but it resolves the top-level
        // `command` name against its own restricted launch `PATH`. A bare `npx`
        // therefore can't be found and the server hangs on "Connecting..." with
        // no error. Launch `/bin/sh` instead (an absolute path Zed can always
        // find), then let the shell resolve `npx` using the rich environment it
        // inherits, and `exec` so stdio is handed cleanly to the bridge.
        // On Windows, GUI apps inherit `PATH` from the registry, so `npx.cmd`
        // works directly.
        match zed::current_platform().0 {
            Os::Windows => Ok(zed::Command {
                command: "npx.cmd".to_string(),
                args,
                env: vec![],
            }),
            Os::Mac | Os::Linux => {
                let mut invocation = String::from("exec npx");
                for arg in &args {
                    invocation.push(' ');
                    invocation.push_str(&shell_quote(arg));
                }
                Ok(zed::Command {
                    command: "/bin/sh".to_string(),
                    args: vec!["-c".to_string(), invocation],
                    env: vec![],
                })
            }
        }
    }

    fn context_server_configuration(
        &mut self,
        context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<zed::ContextServerConfiguration>> {
        let id: &str = context_server_id.as_ref();
        if id != SERVER_ID {
            return Err(format!("unknown context server id: {id}"));
        }

        Ok(Some(zed::ContextServerConfiguration {
            installation_instructions: INSTALLATION_INSTRUCTIONS.to_string(),
            settings_schema: SETTINGS_SCHEMA.to_string(),
            default_settings: DEFAULT_SETTINGS.to_string(),
        }))
    }
}

const INSTALLATION_INSTRUCTIONS: &str = r#"Configure SigNoz MCP Server.

This extension starts the packaged `mcp-remote` bridge with `npx`.

Requirements:
- Node.js 18+ with `npx` on your PATH.
- SigNoz Cloud account, or a self-hosted SigNoz MCP HTTP endpoint.

For SigNoz Cloud, set `region` to one of: `us`, `us2`, `eu`, `eu2`, `in`, `in2`.
If omitted, `region` defaults to `us`.

For self-hosted SigNoz, set `url` to your MCP endpoint, for example:
`http://localhost:8000/mcp`.

For Cloud OAuth, leave `api_key` empty and complete the browser auth flow.
On the first connection a browser tab opens for SigNoz login. Complete it
promptly: if it takes too long the server may report a startup timeout. Just
start the server again, the cached token makes the next connection instant.
For header-based Cloud auth, set both `api_key` and `signoz_url`.
"#;

const SETTINGS_SCHEMA: &str = r#"{
  "type": "object",
  "properties": {
    "region": {
      "type": "string",
      "enum": ["us", "us2", "eu", "eu2", "in", "in2"],
      "description": "SigNoz Cloud region. Ignored when url is set."
    },
    "url": {
      "type": "string",
      "description": "Full MCP endpoint URL for self-hosted or custom SigNoz MCP servers."
    },
    "api_key": {
      "type": "string",
      "description": "Optional SigNoz API key for header-based auth."
    },
    "signoz_url": {
      "type": "string",
      "description": "Optional SigNoz instance URL used with api_key."
    },
    "headers": {
      "type": "object",
      "additionalProperties": {
        "type": "string"
      },
      "description": "Additional headers forwarded to the remote MCP endpoint."
    }
  }
}"#;

const DEFAULT_SETTINGS: &str = r#"{
  "region": "us"
}"#;

/// Pick the MCP endpoint based on user settings, with sensible defaults.
fn resolve_endpoint(settings: &SigNozSettings) -> Result<String> {
    if let Some(url) = settings
        .url
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        validate_http_endpoint(url)?;
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
            let endpoint = format!("https://mcp.{region}.signoz.cloud/mcp");
            validate_http_endpoint(&endpoint)?;
            Ok(endpoint)
        }
        other => Err(format!(
            "unknown SigNoz Cloud region {other:?}; expected one of: us, us2, eu, eu2, in, in2. \
             For a self-hosted SigNoz, set `url` to your MCP endpoint instead."
        )),
    }
}

/// Single-quote a string for safe inclusion in a POSIX shell command line.
/// Wraps the value in single quotes and escapes any embedded single quote as
/// the standard `'\''` sequence, so user-supplied headers, URLs, and API keys
/// can't break out of quoting or inject shell syntax.
fn shell_quote(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('\'');
    for ch in s.chars() {
        if ch == '\'' {
            out.push_str("'\\''");
        } else {
            out.push(ch);
        }
    }
    out.push('\'');
    out
}

fn validate_http_endpoint(url: &str) -> Result<()> {
    if url.starts_with("https://") || url.starts_with("http://") {
        Ok(())
    } else {
        Err("SigNoz MCP endpoint must start with http:// or https://".to_string())
    }
}

zed::register_extension!(SigNozMcpExtension);
