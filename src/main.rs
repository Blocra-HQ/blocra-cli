//! blocra: signed, verifiable traction reports from your terminal or CI.

use std::collections::HashMap;
use std::fs;
use std::io::IsTerminal;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use serde_json::{json, Value};

use blocra::api::{self, fetch_signed_report, fetch_signing_key};
use blocra::config::{self, CONFIG_FILE};
use blocra::defaults::{default_api, official_keys};
use blocra::js::{get, items, nullish, num, or, str_of, stringify, to_str, truthy};
use blocra::markdown::{body_matches_report, fmt_int, fmt_pct, parse_markdown, render_markdown};
use blocra::verify::verify_envelope;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn help() -> String {
    format!(
        "blocra {VERSION}: signed, verifiable traction reports for on-chain projects

Usage
  blocra init --subject <id> [--subject <id>...] [--relayer <account>...] [--chain stellar] [--api <url>] [--days 30]
  blocra report [<id>...] [--days 7|30] [--out <dir>] [--format md,json]
  blocra verify <file.md|file.json>... [--key <G...>]
  blocra key

Subjects
  A smart contract id, or an asset (e.g. XLM or CODE:ISSUER on Stellar).

Options
  --api <url>     BlocRA API (or BLOCRA_API, or \"api\" in {CONFIG_FILE})
  --chain <name>  Chain of the subjects (default stellar)
  --key <G...>    Signing key to trust when verifying (or BLOCRA_TRUSTED_KEY,
                  or \"trusted_key\" in {CONFIG_FILE}, pinned by init)
  --relayer <id>  An account your project uses to submit transactions for users
                  (repeatable). Users who sign their own authorization are
                  found automatically; this only matters for relayers that
                  sign everything themselves.
  --days <n>      Report period in days (default 7)
  --out <dir>     Where report files are written (default .)
  --format <f>    md, json or md,json (default md,json)

Reports are computed and signed by BlocRA from on-chain data; this tool only
fetches and verifies them. Exit code 1 on any failure, so it fits CI."
    )
}

#[derive(Default)]
struct Args {
    positional: Vec<String>,
    subject: Vec<String>,
    relayer: Vec<String>,
    opts: HashMap<String, String>,
    help: bool,
    version: bool,
}

impl Args {
    fn opt(&self, k: &str) -> Option<String> {
        self.opts.get(k).cloned()
    }
}

fn parse_args(argv: Vec<String>) -> Result<Args, String> {
    let mut args = Args::default();
    let mut it = argv.into_iter();
    while let Some(a) = it.next() {
        if a == "-h" || a == "--help" {
            args.help = true;
        } else if a == "-v" || a == "--version" {
            args.version = true;
        } else if let Some(flag) = a.strip_prefix("--") {
            let (k, v) = match flag.split_once('=') {
                Some((k, v)) => (k.to_string(), v.to_string()),
                None => {
                    let v = it.next().ok_or_else(|| format!("--{flag} needs a value"))?;
                    (flag.to_string(), v)
                }
            };
            match k.as_str() {
                "subject" => args.subject.push(v),
                "relayer" => args.relayer.push(v),
                _ => {
                    args.opts.insert(k, v);
                }
            }
        } else {
            args.positional.push(a);
        }
    }
    Ok(args)
}

fn color() -> bool {
    std::io::stdout().is_terminal() && std::env::var_os("NO_COLOR").is_none()
}
fn paint(code: &str, s: &str) -> String {
    if color() {
        format!("\x1b[{code}m{s}\x1b[0m")
    } else {
        s.to_string()
    }
}
fn bold(s: &str) -> String {
    paint("1", s)
}
fn green(s: &str) -> String {
    paint("32", s)
}
fn red(s: &str) -> String {
    paint("31", s)
}
fn dim(s: &str) -> String {
    paint("2", s)
}

fn env(k: &str) -> Option<String> {
    std::env::var(k).ok()
}

/// A string setting from the config file.
fn cfg_str(config: Option<&Value>, k: &str) -> Option<String> {
    str_of(get(config, k)).map(String::from)
}

/// A list setting from the config file.
fn cfg_list(config: Option<&Value>, k: &str) -> Vec<String> {
    items(get(config, k))
        .iter()
        .filter_map(Value::as_str)
        .map(String::from)
        .collect()
}

fn api_url(args: &Args, config: Option<&Value>) -> Result<String, String> {
    args.opt("api")
        .or_else(|| env("BLOCRA_API"))
        .or_else(|| cfg_str(config, "api"))
        .or_else(default_api)
        .ok_or_else(|| {
            "No BlocRA API set. Pass --api <url>, set BLOCRA_API, or run \"blocra init --api <url>\".".into()
        })
}

fn parse_days(flag: Option<String>, config: Option<&Value>, default: u32) -> Result<u32, String> {
    let days = match flag {
        Some(s) => s.trim().parse::<f64>().ok(),
        None => match get(config, "days") {
            None | Some(Value::Null) => Some(f64::from(default)),
            v => num(v).or_else(|| str_of(v).and_then(|s| s.trim().parse().ok())),
        },
    };
    match days {
        Some(d) if d >= 1.0 && d.fract() == 0.0 && d <= f64::from(u32::MAX) => Ok(d as u32),
        _ => Err("--days must be a whole number of days (1 or more).".into()),
    }
}

fn pinned_key(args: &Args, config: Option<&Value>) -> Option<String> {
    args.opt("key")
        .or_else(|| env("BLOCRA_TRUSTED_KEY"))
        .or_else(|| cfg_str(config, "trusted_key"))
}

/// The signer(s) to trust: an explicit flag/env/config key, else BlocRA's
/// official keys built into this binary, else the server's key (with a notice).
fn trusted_signers(args: &Args, config: Option<&Value>, api: &str) -> Result<Vec<String>, String> {
    if let Some(k) = pinned_key(args, config) {
        return Ok(vec![k]);
    }
    let official = official_keys();
    if !official.is_empty() {
        return Ok(official);
    }
    let key = fetch_signing_key(api)?;
    let signer = to_str(get(Some(&key), "signer"));
    eprintln!(
        "{}",
        dim(&format!(
            "No pinned key; trusting the key served by {}: {signer}",
            api::host(api)?
        ))
    );
    Ok(vec![signer])
}

fn cmd_init(args: &Args) -> Result<bool, String> {
    let existing = config::load()?;
    let existing = existing.as_ref();
    let api = args
        .opt("api")
        .or_else(|| env("BLOCRA_API"))
        .or_else(|| cfg_str(existing, "api"))
        .or_else(default_api)
        .ok_or("Pass --api <url> (the BlocRA API to fetch reports from).")?;
    let subjects = if args.subject.is_empty() {
        cfg_list(existing, "subjects")
    } else {
        args.subject.clone()
    };
    if subjects.is_empty() {
        return Err("Pass at least one --subject <contract id or asset>.".into());
    }
    let key = fetch_signing_key(&api)?;
    let key = Some(&key);
    let chain = args
        .opt("chain")
        .or_else(|| cfg_str(existing, "chain"))
        .unwrap_or("stellar".into())
        .to_lowercase();
    if let Some(Value::Array(chains)) = get(key, "chains") {
        if !chains.iter().any(|c| c.as_str() == Some(chain.as_str())) {
            let list = chains.iter().map(|c| to_str(Some(c))).collect::<Vec<_>>().join(", ");
            eprintln!(
                "{}",
                dim(&format!(
                    "Note: {} reports on {list} today; {chain} is not supported yet.",
                    api::host(&api)?
                ))
            );
        }
    }
    let days = parse_days(args.opt("days"), existing, 30)?;
    let signer = get(key, "signer").cloned().unwrap_or(Value::Null);
    let config = json!({
        "api": api,
        "chain": chain,
        "subjects": subjects,
        "relayers": if args.relayer.is_empty() { cfg_list(existing, "relayers") } else { args.relayer.clone() },
        "days": days,
        "trusted_key": signer,
        "out": args.opt("out").or_else(|| cfg_str(existing, "out")).unwrap_or("reports".into()),
    });
    let path = config::save(&config)?;
    let name = path
        .file_name()
        .map_or(CONFIG_FILE.into(), |n| n.to_string_lossy().into_owned());
    println!(
        "{} Wrote {name}: {} subject(s) on {chain}, {days}-day reports",
        green("✓"),
        subjects.len()
    );
    println!(
        "  Pinned BlocRA signing key {} ({})",
        to_str(Some(&signer)),
        to_str(get(key, "method_version"))
    );
    println!("  Next: {}", bold("blocra report"));
    Ok(true)
}

fn print_summary(report: &Value) {
    let r = Some(report);
    let k = get(r, "kpis");
    let subject = get(r, "subject");
    let noun = if str_of(get(subject, "kind")) == Some("contract") {
        "calls"
    } else {
        "payments"
    };
    let chain = or(get(r, "chain"), None).map_or("stellar".into(), |c| to_str(Some(c)));
    let period = get(r, "period");
    let eco = get(r, "ecosystem");
    let or_dash = |v| if nullish(v) { "—".to_string() } else { to_str(v) };
    println!(
        "\n{} {}",
        bold(&to_str(or(get(subject, "label"), get(subject, "id")))),
        dim(&format!(
            "{chain} · {} – {}",
            to_str(get(period, "from")),
            to_str(get(period, "to"))
        ))
    );
    println!(
        "  {} active accounts · {} {noun} · {} failed · rank #{} of {}",
        fmt_int(get(get(k, "active_accounts"), "value")),
        fmt_int(get(get(k, "calls"), "value")),
        fmt_pct(get(get(k, "failed_pct"), "value")),
        or_dash(get(eco, "rank")),
        or_dash(get(eco, "of")),
    );
    let q = get(r, "quality");
    if truthy(get(q, "relayed_calls")) {
        println!(
            "{}",
            dim(&format!(
                "  {} of calls came through relayers; counted for the users who signed them",
                fmt_pct(get(q, "relayed_share_pct"))
            ))
        );
    }
    for f in items(get(r, "findings")) {
        println!("  - {}", to_str(Some(f)));
    }
}

/// `blocra-<chain>-<subject>-<to>-<days>d`.
fn report_name(chain: &str, report: &Value, days: u32) -> Result<String, String> {
    let r = Some(report);
    let id = get(get(r, "subject"), "id").ok_or("The report has no subject id")?;
    let to = get(get(r, "period"), "to").ok_or("The report has no period")?;
    let id = to_str(Some(id));
    let id: String = id.split(':').next().unwrap_or_default().chars().take(12).collect();
    Ok(format!(
        "blocra-{chain}-{}-{}-{days}d",
        id.to_lowercase(),
        to_str(Some(to))
    ))
}

fn write_file(path: &Path, text: &str) -> Result<(), String> {
    fs::write(path, text).map_err(|e| format!("Could not write {}: {e}", path.display()))
}

fn cmd_report(args: &Args) -> Result<bool, String> {
    let config = config::load()?;
    let config = config.as_ref();
    let api = api_url(args, config)?;
    let subjects = if args.positional.is_empty() {
        cfg_list(config, "subjects")
    } else {
        args.positional.clone()
    };
    if subjects.is_empty() {
        return Err("Name a subject (blocra report <id>) or run \"blocra init\" first.".into());
    }
    let days = parse_days(args.opt("days"), config, 7)?;
    let chain = args
        .opt("chain")
        .or_else(|| cfg_str(config, "chain"))
        .unwrap_or("stellar".into())
        .to_lowercase();
    let format = args.opt("format").unwrap_or("md,json".into());
    let formats: Vec<&str> = format.split(',').map(str::trim).collect();
    let out_dir = PathBuf::from(args.opt("out").or_else(|| cfg_str(config, "out")).unwrap_or(".".into()));
    let relayers = if args.relayer.is_empty() {
        cfg_list(config, "relayers")
    } else {
        args.relayer.clone()
    };
    let signers = trusted_signers(args, config, &api)?;
    fs::create_dir_all(&out_dir).map_err(|e| format!("Could not create {}: {e}", out_dir.display()))?;

    let mut all_ok = true;
    for subject in &subjects {
        let result = (|| -> Result<(), String> {
            let envelope = fetch_signed_report(&api, subject, days, &chain, &relayers)?;
            let check = verify_envelope(&envelope, &signers);
            if !check.ok {
                return Err(format!(
                    "Received report failed verification: {}",
                    check.reasons.join("; ")
                ));
            }
            let report = check.report.as_ref().ok_or("The report is empty")?;
            let name = report_name(&chain, report, days)?;
            let mut written = Vec::new();
            if formats.contains(&"json") {
                let p = out_dir.join(format!("{name}.json"));
                write_file(&p, &(stringify(&envelope, true) + "\n"))?;
                written.push(p);
            }
            if formats.contains(&"md") {
                let p = out_dir.join(format!("{name}.md"));
                write_file(&p, &render_markdown(&envelope)?)?;
                written.push(p);
            }
            print_summary(report);
            let signer = to_str(get(get(Some(&envelope), "attestation"), "signer"));
            let short_signer: String = signer.chars().take(8).collect();
            println!(
                "  {} signed by {short_signer}… · sha256 {}…",
                green("✓"),
                &check.sha256[..16]
            );
            for p in written {
                println!("  → {}", p.display());
            }
            Ok(())
        })();
        if let Err(e) = result {
            all_ok = false;
            eprintln!("{} {subject}: {e}", red("✗"));
        }
    }
    Ok(all_ok)
}

fn cmd_verify(args: &Args) -> Result<bool, String> {
    if args.positional.is_empty() {
        return Err("Name the report file(s) to verify.".into());
    }
    let config = config::load()?;
    let config = config.as_ref();
    let mut signers: Option<Vec<String>> = pinned_key(args, config).map(|k| vec![k]).or_else(|| {
        let official = official_keys();
        (!official.is_empty()).then_some(official)
    });
    let mut all_ok = true;
    for file in &args.positional {
        let text = fs::read_to_string(file).map_err(|e| format!("Could not read {file}: {e}"))?;
        let (envelope, body) = if file.ends_with(".md") {
            let parsed = parse_markdown(&text);
            let body = parsed.envelope.as_ref().map(|_| parsed.body.clone());
            (parsed.envelope, body)
        } else {
            (serde_json::from_str::<Value>(&text).ok(), None)
        };
        let Some(envelope) = envelope.filter(|e| truthy(Some(e))) else {
            all_ok = false;
            println!("{} {file}: no BlocRA signature found", red("✗ FAIL"));
            continue;
        };
        if signers.is_none() {
            signers = Some(trusted_signers(args, config, &api_url(args, config)?)?);
        }
        let mut check = verify_envelope(&envelope, signers.as_deref().unwrap_or_default());
        let attestation = get(Some(&envelope), "attestation");
        if let (true, Some(body), Some(report)) = (check.ok, &body, &check.report) {
            if !body_matches_report(body, report, attestation) {
                check.ok = false;
                check
                    .reasons
                    .push("The Markdown text was edited after signing (it no longer matches the signed report)".into());
            }
        }
        if check.ok {
            let r = check.report.as_ref();
            let chain = get(r, "chain").map_or("stellar".into(), |c| {
                if c.is_null() {
                    "stellar".into()
                } else {
                    to_str(Some(c))
                }
            });
            let period = get(r, "period");
            println!("{} {file}", green("✓ VALID"));
            println!(
                "  {} · {chain} · {} – {} · {}",
                to_str(get(get(r, "subject"), "label")),
                to_str(get(period, "from")),
                to_str(get(period, "to")),
                to_str(get(r, "method_version"))
            );
            println!(
                "  Signed by {} at {}",
                to_str(get(attestation, "signer")),
                to_str(get(attestation, "signed_at"))
            );
            println!("  sha256 {}", check.sha256);
        } else {
            all_ok = false;
            println!("{} {file}", red("✗ FAIL"));
            for reason in &check.reasons {
                println!("  - {reason}");
            }
        }
    }
    Ok(all_ok)
}

fn cmd_key(args: &Args) -> Result<bool, String> {
    let config = config::load()?;
    let key = fetch_signing_key(&api_url(args, config.as_ref())?)?;
    let key = Some(&key);
    let chains = items(get(key, "chains"))
        .iter()
        .map(|c| to_str(Some(c)))
        .collect::<Vec<_>>()
        .join(", ");
    println!(
        "{}  {}",
        to_str(get(key, "signer")),
        dim(&format!(
            "{} · {} · chains: {chains}",
            to_str(get(key, "algorithm")),
            to_str(get(key, "method_version"))
        ))
    );
    Ok(true)
}

fn run() -> Result<bool, String> {
    let mut args = parse_args(std::env::args().skip(1).collect())?;
    if args.version {
        println!("{VERSION}");
        return Ok(true);
    }
    if args.positional.is_empty() || args.help || args.positional[0] == "help" {
        println!("{}", help());
        return Ok(true);
    }
    let command = args.positional.remove(0);
    match command.as_str() {
        "init" => cmd_init(&args),
        "report" => cmd_report(&args),
        "verify" => cmd_verify(&args),
        "key" => cmd_key(&args),
        _ => Err(format!("Unknown command \"{command}\". Run blocra --help.")),
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(e) => {
            eprintln!("{} {e}", red("error"));
            ExitCode::FAILURE
        }
    }
}
