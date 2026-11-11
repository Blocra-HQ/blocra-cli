//! BlocRA API client.

use std::time::Duration;

use serde_json::Value;
use url::Url;

const TIMEOUT: Duration = Duration::from_secs(60);
const USER_AGENT: &str = concat!("blocra-cli/", env!("CARGO_PKG_VERSION"));

/// `scheme://host[:port]` of a URL, for messages.
pub fn origin(url: &str) -> String {
    Url::parse(url).map_or_else(|_| url.to_string(), |u| u.origin().ascii_serialization())
}

/// `host[:port]` of the API, for messages.
pub fn host(api: &str) -> Result<String, String> {
    let u = Url::parse(api).map_err(|_| format!("Invalid API URL: {api}"))?;
    let host = u.host_str().unwrap_or_default();
    Ok(match u.port() {
        Some(p) => format!("{host}:{p}"),
        None => host.to_string(),
    })
}

fn get_json(url: &str) -> Result<Value, String> {
    let agent = ureq::AgentBuilder::new()
        .timeout(TIMEOUT)
        .user_agent(USER_AGENT)
        .build();
    let (status, response) = match agent.get(url).call() {
        Ok(r) => (r.status(), r),
        Err(ureq::Error::Status(code, r)) => (code, r),
        Err(ureq::Error::Transport(t)) => {
            // The full error repeats the URL; keep only the cause.
            let mut reason = t.message().map_or_else(|| t.kind().to_string(), String::from);
            if let Some(cause) = std::error::Error::source(&t) {
                reason = format!("{reason}: {cause}");
            }
            let timed_out = t.to_string().to_lowercase().contains("timed out");
            return Err(if timed_out {
                format!("Timed out reaching {}", origin(url))
            } else {
                format!("Could not reach {}: {reason}", origin(url))
            });
        }
    };
    let body: Option<Value> = response.into_string().ok().and_then(|t| serde_json::from_str(&t).ok());
    let failed = body.as_ref().and_then(|b| b.get("success")) == Some(&Value::Bool(false));
    if !(200..300).contains(&status) || failed {
        let message = body.as_ref().and_then(|b| b.get("message")).and_then(Value::as_str);
        return Err(message.map_or_else(|| format!("HTTP {status} from {}", origin(url)), String::from));
    }
    Ok(body.unwrap_or(Value::Null))
}

fn base(api: &str) -> String {
    format!("{}/api", api.trim_end_matches('/'))
}

/// The key the server signs with: `{ signer, algorithm, method_version, chains }`.
pub fn fetch_signing_key(api: &str) -> Result<Value, String> {
    get_json(&format!("{}/attestation/key", base(api)))
}

/// A signed traction report envelope: `{ report_json, attestation }`.
pub fn fetch_signed_report(
    api: &str,
    subject: &str,
    days: u32,
    chain: &str,
    relayers: &[String],
) -> Result<Value, String> {
    let mut url =
        Url::parse(&format!("{}/reports/traction", base(api))).map_err(|_| format!("Invalid API URL: {api}"))?;
    {
        let mut q = url.query_pairs_mut();
        q.append_pair("subject", subject)
            .append_pair("days", &days.to_string())
            .append_pair("chain", chain)
            .append_pair("signed", "true");
        if !relayers.is_empty() {
            q.append_pair("relayers", &relayers.join(","));
        }
    }
    let body = get_json(url.as_str())?;
    Ok(body.get("signed").cloned().unwrap_or(Value::Null))
}
