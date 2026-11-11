//! The renderer must match the golden files byte for byte (they are what the
//! BlocRA website's renderer produces), and verification must accept genuine
//! reports and reject every kind of tampering.

use std::fs;

use ed25519_dalek::{Signer, SigningKey};
use serde_json::Value;

use blocra::markdown::{body_matches_report, parse_markdown, render_body, render_markdown};
use blocra::strkey::decode_account_id;
use blocra::verify::{sha256_hex, verify_envelope};

const OTHER_KEY: &str = "GDQNY3PBOJOKYZSRMK2S7LHHGWZIUISD4QORETLMXEWXBI7KFZZMKTL3";

fn read(path: &str) -> String {
    fs::read_to_string(format!("{}/test/fixtures/{path}", env!("CARGO_MANIFEST_DIR"))).unwrap()
}
fn fixture(name: &str) -> Value {
    serde_json::from_str(&read(&format!("{name}.json"))).unwrap()
}
fn signer() -> String {
    fixture("signed-asset")["attestation"]["signer"]
        .as_str()
        .unwrap()
        .to_string()
}
fn report_json(env: &Value) -> String {
    env["report_json"].as_str().unwrap().to_string()
}

#[test]
fn signed_fixtures_render_like_the_golden_files() {
    for name in ["signed-asset", "signed-contract", "signed-relayed"] {
        assert_eq!(
            render_markdown(&fixture(name)).unwrap(),
            read(&format!("{name}.md")),
            "{name}"
        );
    }
}

#[test]
fn edge_cases_render_like_the_golden_files() {
    for name in ["full-asset", "edge-numbers", "relayed-unsigned", "empty"] {
        let case = fixture(&format!("render/{name}"));
        let att = case.get("attestation").filter(|a| !a.is_null());
        assert_eq!(
            render_body(&case["report"], att),
            read(&format!("render/{name}.md")),
            "{name}"
        );
    }
}

#[test]
fn decodes_stellar_account_ids_and_rejects_typos() {
    let s = signer();
    assert_eq!(decode_account_id(&s).unwrap().len(), 32);
    let swap = if &s[10..11] == "A" { "B" } else { "A" };
    let typo = format!("{}{swap}{}", &s[..10], &s[11..]);
    assert!(decode_account_id(&typo).unwrap_err().contains("checksum"));
    assert!(decode_account_id("SABC").unwrap_err().contains("account id"));
}

#[test]
fn accepts_genuine_signed_reports() {
    for name in ["signed-asset", "signed-contract", "signed-relayed"] {
        let env = fixture(name);
        let r = verify_envelope(&env, &[signer()]);
        assert!(r.ok, "{name}: {}", r.reasons.join("; "));
        assert_eq!(r.report.unwrap()["method_version"], "traction/1");
        assert_eq!(r.sha256, env["attestation"]["payload_sha256"].as_str().unwrap());
    }
}

#[test]
fn rejects_an_edited_report() {
    let mut env = fixture("signed-asset");
    let text = report_json(&env).replacen(
        "\"active_accounts\":{\"change_pct\":null,\"previous\":null,\"value\":",
        "\"active_accounts\":{\"change_pct\":null,\"previous\":null,\"value\":9",
        1,
    );
    assert_ne!(text, report_json(&env));
    env["report_json"] = text.into();
    assert!(!verify_envelope(&env, &[signer()]).ok);
}

#[test]
fn rejects_a_report_resigned_by_another_key_even_with_a_consistent_hash() {
    let mut env = fixture("signed-asset");
    let key = SigningKey::from_bytes(&[7u8; 32]);
    let text = report_json(&env).replacen("\"findings\":[", "\"findings\":[\"All good.\",", 1);
    let sig: String = key
        .sign(text.as_bytes())
        .to_bytes()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    env["attestation"]["signature"] = sig.into();
    env["attestation"]["payload_sha256"] = sha256_hex(text.as_bytes()).into();
    env["report_json"] = text.into();
    assert!(!verify_envelope(&env, &[signer()]).ok);
}

#[test]
fn rejects_a_genuine_report_when_a_different_key_is_trusted() {
    let r = verify_envelope(&fixture("signed-asset"), &[OTHER_KEY.into()]);
    assert!(!r.ok);
    assert!(r.reasons.join(" ").contains("not by the trusted BlocRA key"));
}

#[test]
fn trusts_any_key_in_a_list_of_official_keys() {
    let env = fixture("signed-asset");
    assert!(verify_envelope(&env, &[OTHER_KEY.into(), signer()]).ok);
    assert!(!verify_envelope(&env, &[OTHER_KEY.into()]).ok);
}

#[test]
fn rejects_envelopes_that_are_not_signed_reports() {
    for bad in [
        serde_json::json!({}),
        serde_json::json!({"report_json": "{}"}),
        serde_json::json!(5),
    ] {
        let r = verify_envelope(&bad, &[signer()]);
        assert!(!r.ok);
        assert!(r.reasons[0].contains("Not a BlocRA signed report"));
    }
}

#[test]
fn markdown_round_trips_and_embeds_the_envelope() {
    for name in ["signed-asset", "signed-contract", "signed-relayed"] {
        let env = fixture(name);
        let md = render_markdown(&env).unwrap();
        for bad in ["NaN", "undefined", "[object"] {
            assert!(!md.contains(bad), "{name} contains {bad}");
        }
        let parsed = parse_markdown(&md);
        assert_eq!(parsed.envelope.as_ref(), Some(&env));
        let report: Value = serde_json::from_str(&report_json(&env)).unwrap();
        assert!(body_matches_report(&parsed.body, &report, env.get("attestation")));
        let edited = parsed.body.replacen("| Active accounts | ", "| Active accounts | 9", 1);
        assert!(!body_matches_report(&edited, &report, env.get("attestation")));
    }
}

#[test]
fn relayed_contract_reports_show_relayer_figures() {
    let md = render_markdown(&fixture("signed-relayed")).unwrap();
    assert!(md.contains("Calls submitted by a relayer or sponsor"));
    assert!(md.contains("Declared relayers (excluded from account counts)"));
}
