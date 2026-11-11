//! Verification of BlocRA-signed reports.
//!
//! A signed report is an envelope `{ report_json, attestation }`. The
//! signature is Ed25519 over the exact UTF-8 bytes of `report_json`, so
//! nothing is re-serialised: check the bytes, then parse them.

use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::js::{get, str_of, to_str, truthy};
use crate::strkey::decode_account_id;

pub struct Check {
    /// True only if every check passed.
    pub ok: bool,
    /// The parsed report, when `report_json` is valid JSON.
    pub report: Option<Value>,
    /// Hex sha256 of `report_json` (empty if it is missing).
    pub sha256: String,
    pub reasons: Vec<String>,
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes).iter().map(|b| format!("{b:02x}")).collect()
}

/// Lenient hex like Node's `Buffer.from(s, "hex")`: stops at the first
/// invalid pair.
fn hex_bytes(s: &str) -> Vec<u8> {
    s.as_bytes()
        .chunks_exact(2)
        .map_while(|p| std::str::from_utf8(p).ok().and_then(|p| u8::from_str_radix(p, 16).ok()))
        .collect()
}

fn signature_valid(text: &str, signer: Option<&Value>, signature: Option<&Value>) -> Result<bool, String> {
    let key = decode_account_id(str_of(signer).unwrap_or_default())?;
    let sig = str_of(signature).ok_or("the signature is missing")?;
    let key = VerifyingKey::from_bytes(&key).map_err(|_| "the signer is not a valid Ed25519 key".to_string())?;
    let Ok(sig) = <[u8; 64]>::try_from(hex_bytes(sig)) else {
        return Ok(false);
    };
    Ok(key.verify(text.as_bytes(), &Signature::from_bytes(&sig)).is_ok())
}

/// Checks an envelope against the key(s) you trust.
pub fn verify_envelope(envelope: &Value, trusted: &[String]) -> Check {
    let trusted: Vec<&String> = trusted.iter().filter(|k| !k.is_empty()).collect();
    let env = Some(envelope);
    let att = get(env, "attestation");
    let text = match str_of(get(env, "report_json")) {
        Some(t) if truthy(att) => t,
        _ => {
            return Check {
                ok: false,
                report: None,
                sha256: String::new(),
                reasons: vec!["Not a BlocRA signed report (missing report_json or attestation)".into()],
            }
        }
    };
    let mut reasons = Vec::new();
    if str_of(get(att, "algorithm")) != Some("ed25519") {
        reasons.push(format!(
            "Unsupported signature algorithm \"{}\"",
            to_str(get(att, "algorithm"))
        ));
    }
    let signer = get(att, "signer");
    if !trusted.is_empty() && !str_of(signer).is_some_and(|s| trusted.iter().any(|k| k.as_str() == s)) {
        let list = trusted.iter().map(|k| k.as_str()).collect::<Vec<_>>().join(", ");
        let plural = if trusted.len() > 1 { "s" } else { "" };
        reasons.push(format!(
            "Signed by {}, not by the trusted BlocRA key{plural} {list}",
            to_str(signer)
        ));
    }
    let sha = sha256_hex(text.as_bytes());
    let claimed = get(att, "payload_sha256");
    if truthy(claimed) && str_of(claimed) != Some(sha.as_str()) {
        reasons.push("payload_sha256 does not match the report".into());
    }
    let signature_ok = match signature_valid(text, signer, get(att, "signature")) {
        Ok(ok) => ok,
        Err(e) => {
            reasons.push(format!("Signature check failed: {e}"));
            false
        }
    };
    if !signature_ok && !reasons.iter().any(|r| r.starts_with("Signature")) {
        reasons.push("Signature is not valid: the report was changed or not signed by this key".into());
    }
    let report: Option<Value> = serde_json::from_str(text).ok();
    if report.is_none() {
        reasons.push("report_json is not valid JSON".into());
    }
    let (att_method, report_method) = (get(att, "method_version"), get(report.as_ref(), "method_version"));
    if truthy(report.as_ref()) && truthy(att_method) && truthy(report_method) && !strict_eq(att_method, report_method) {
        reasons.push("Method version in the report and the attestation differ".into());
    }
    Check {
        ok: reasons.is_empty() && signature_ok,
        report,
        sha256: sha,
        reasons,
    }
}

/// `a === b` for JSON values (objects and arrays are never equal).
fn strict_eq(a: Option<&Value>, b: Option<&Value>) -> bool {
    match (a, b) {
        (Some(Value::String(x)), Some(Value::String(y))) => x == y,
        (Some(Value::Number(x)), Some(Value::Number(y))) => x.as_f64() == y.as_f64(),
        (Some(Value::Bool(x)), Some(Value::Bool(y))) => x == y,
        _ => false,
    }
}
