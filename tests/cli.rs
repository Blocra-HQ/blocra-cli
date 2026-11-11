//! End-to-end runs of the `blocra` binary (no network).

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn fixture(name: &str) -> String {
    fs::read_to_string(format!("{}/test/fixtures/{name}", env!("CARGO_MANIFEST_DIR"))).unwrap()
}

fn signer() -> String {
    let env: serde_json::Value = serde_json::from_str(&fixture("signed-asset.json")).unwrap();
    env["attestation"]["signer"].as_str().unwrap().to_string()
}

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("blocra-test-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn blocra(args: &[&str], dir: &PathBuf) -> (i32, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_blocra"))
        .args(args)
        .current_dir(dir)
        .env("NO_COLOR", "1")
        .env_remove("BLOCRA_API")
        .env_remove("BLOCRA_TRUSTED_KEY")
        .output()
        .unwrap();
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    (out.status.code().unwrap_or(-1), text)
}

#[test]
fn verify_passes_a_genuine_markdown_report_and_fails_an_edited_one() {
    let dir = temp_dir("md");
    let md = fixture("signed-asset.md");
    fs::write(dir.join("good.md"), &md).unwrap();
    let (code, out) = blocra(&["verify", "good.md", "--key", &signer()], &dir);
    assert_eq!(code, 0, "{out}");
    assert!(out.contains("✓ VALID good.md"), "{out}");
    assert!(out.contains("sha256 f3c59e0ccf034c68"), "{out}");

    let start = md.find("| Active accounts | ").unwrap() + "| Active accounts | ".len();
    let end = start + md[start..].find(' ').unwrap();
    fs::write(dir.join("bad.md"), format!("{}999,999{}", &md[..start], &md[end..])).unwrap();
    let (code, out) = blocra(&["verify", "bad.md", "--key", &signer()], &dir);
    assert_eq!(code, 1, "{out}");
    assert!(out.contains("edited after signing"), "{out}");
}

#[test]
fn verify_passes_json_envelopes() {
    let dir = temp_dir("json");
    for name in ["signed-asset.json", "signed-contract.json", "signed-relayed.json"] {
        fs::write(dir.join(name), fixture(name)).unwrap();
    }
    let (code, out) = blocra(
        &[
            "verify",
            "signed-asset.json",
            "signed-contract.json",
            "signed-relayed.json",
            "--key",
            &signer(),
        ],
        &dir,
    );
    assert_eq!(code, 0, "{out}");
    assert_eq!(out.matches("✓ VALID").count(), 3, "{out}");
}

#[test]
fn verify_fails_files_without_a_signature() {
    let dir = temp_dir("plain");
    fs::write(dir.join("plain.md"), "# Our traction\n\n1,000,000 users\n").unwrap();
    let (code, out) = blocra(&["verify", "plain.md", "--key", &signer()], &dir);
    assert_eq!(code, 1);
    assert!(out.contains("no BlocRA signature"), "{out}");
}

#[test]
fn verify_uses_the_key_pinned_in_blocra_json() {
    let dir = temp_dir("pinned");
    fs::write(dir.join("r.md"), fixture("signed-asset.md")).unwrap();
    fs::write(
        dir.join("blocra.json"),
        r#"{"trusted_key": "GDQNY3PBOJOKYZSRMK2S7LHHGWZIUISD4QORETLMXEWXBI7KFZZMKTL3"}"#,
    )
    .unwrap();
    let (code, out) = blocra(&["verify", "r.md"], &dir);
    assert_eq!(code, 1, "{out}");
    assert!(out.contains("not by the trusted BlocRA key"), "{out}");
}

#[test]
fn help_version_and_unknown_commands() {
    let dir = temp_dir("help");
    let (code, out) = blocra(&["--help"], &dir);
    assert_eq!(code, 0);
    assert!(out.contains("blocra verify <file.md|file.json>"));
    let (code, out) = blocra(&["--version"], &dir);
    assert_eq!((code, out.trim()), (0, env!("CARGO_PKG_VERSION")));
    let (code, out) = blocra(&["frobnicate"], &dir);
    assert_eq!(code, 1);
    assert!(out.contains("Unknown command"));
    let (code, out) = blocra(&["report", "X"], &dir);
    assert_eq!(code, 1);
    assert!(out.contains("No BlocRA API set"), "{out}");
}
