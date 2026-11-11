//! Markdown rendering of a traction report.
//!
//! The signed envelope is embedded at the end of the file in an HTML comment,
//! so a single .md file can be shared and still verified: `blocra verify`
//! checks the signature, re-renders the body from the signed report and
//! compares it with the file, so an edited figure is caught even though the
//! envelope itself is intact.
//!
//! The BlocRA website renders the same files in the browser. Both renderers
//! must stay byte-identical; `tests/parity.rs` checks this one against the
//! golden files in `test/fixtures`, and the website checks its own against
//! the same files.

use base64::alphabet::STANDARD;
use base64::engine::general_purpose::{GeneralPurpose, GeneralPurposeConfig, STANDARD as B64};
use base64::engine::DecodePaddingMode;
use base64::Engine;
use serde_json::Value;

use crate::js::{get, items, locale, math_round, nullish, num, or, str_of, stringify, to_str, truthy, Js};

const DASH: &str = "—";
const MARKER: &str = "blocra-attestation:";

pub fn fmt_int(n: Js) -> String {
    num(n).map_or(DASH.into(), |n| locale(math_round(n), 0))
}

pub fn fmt_num(n: Js, digits: usize) -> String {
    num(n).map_or(DASH.into(), |n| locale(n, digits))
}

pub fn fmt_pct(n: Js) -> String {
    match num(n) {
        None => DASH.into(),
        Some(n) if n > 0.0 && n < 0.05 => "<0.1%".into(),
        Some(n) => format!("{}%", locale(n, 1)),
    }
}

pub fn fmt_change(n: Js) -> String {
    match num(n) {
        None => DASH.into(),
        Some(n) => format!("{}{}%", if n > 0.0 { "+" } else { "" }, locale(n, 1)),
    }
}

/// `GABCDE…WXYZ` for long ids.
fn short(id: Js) -> String {
    match id {
        Some(Value::String(s)) if s.encode_utf16().count() > 16 => {
            let units: Vec<u16> = s.encode_utf16().collect();
            let head = String::from_utf16_lossy(&units[..6]);
            let tail = String::from_utf16_lossy(&units[units.len() - 4..]);
            format!("{head}…{tail}")
        }
        v if nullish(v) => DASH.into(),
        v => to_str(v),
    }
}

fn cell(s: &str) -> String {
    s.replace('|', "\\|").replace('\n', " ")
}

fn table(head: &[&str], rows: &[Vec<String>]) -> String {
    let mut lines = vec![
        format!("| {} |", head.iter().map(|h| cell(h)).collect::<Vec<_>>().join(" | ")),
        format!("| {} |", head.iter().map(|_| "---").collect::<Vec<_>>().join(" | ")),
    ];
    for r in rows {
        lines.push(format!(
            "| {} |",
            r.iter().map(|c| cell(c)).collect::<Vec<_>>().join(" | ")
        ));
    }
    lines.join("\n")
}

fn row<const N: usize>(cells: [String; N]) -> Vec<String> {
    cells.to_vec()
}

fn held_time(c: Js) -> String {
    let days_v = get(c, "days_covered");
    let days = if nullish(days_v) {
        0.0
    } else {
        num(days_v).unwrap_or(f64::NAN)
    };
    if days < 1.0 {
        let hours = get(c, "hours_covered");
        let hours = if nullish(hours) {
            Value::from(days * 24.0)
        } else {
            hours.cloned().unwrap_or(Value::Null)
        };
        return format!("{} hours", fmt_num(Some(&hours), 1));
    }
    format!("{} days", fmt_num(Some(&Value::from(days)), 1))
}

/// Block-explorer links per chain, for account ids in the report.
fn explorer_account_url(chain: &str) -> Option<&'static str> {
    match chain {
        "stellar" => Some("https://stellar.expert/explorer/public/account/"),
        _ => None,
    }
}

fn s(v: Js) -> String {
    to_str(v)
}

/// The report body (everything except the embedded attestation).
pub fn render_body(report: &Value, attestation: Option<&Value>) -> String {
    let r = Some(report);
    let attestation = attestation.filter(|a| truthy(Some(a)));
    let subject = get(r, "subject");
    let kind = str_of(get(subject, "kind"));
    let is_contract = kind == Some("contract");
    let noun = if is_contract { "calls" } else { "payments" };
    let noun_title = if is_contract { "Contract calls" } else { "Payments" };
    let chain_v = get(r, "chain");
    let chain = if nullish(chain_v) {
        "stellar".to_string()
    } else {
        s(chain_v)
    };
    let explorer = match chain_v {
        Some(Value::String(c)) => explorer_account_url(c),
        v if nullish(v) => explorer_account_url("stellar"),
        _ => None,
    };
    let period = get(r, "period");
    let mut out: Vec<String> = Vec::new();
    macro_rules! push { ($($l:expr),* $(,)?) => {{ $( out.push(String::from($l)); )* }}; }

    push!(
        format!(
            "# Traction Report: {}",
            s(or(get(subject, "label"), get(subject, "id")))
        ),
        ""
    );
    let network = get(r, "network");
    let method = get(r, "method_version");
    push!(
        table(
            &["Field", "Value"],
            &[
                row(["Subject".into(), format!("`{}`", s(get(subject, "id")))]),
                row([
                    "Kind".into(),
                    if is_contract { "Smart contract" } else { "Asset" }.into()
                ]),
                row([
                    "Chain".into(),
                    format!(
                        "{chain} ({})",
                        if nullish(network) { "mainnet".into() } else { s(network) }
                    )
                ]),
                row([
                    "Period".into(),
                    format!(
                        "{} – {} (UTC, {} days)",
                        s(get(period, "from")),
                        s(get(period, "to")),
                        s(get(period, "days"))
                    ),
                ]),
                row(["Generated".into(), s(get(r, "generated_at"))]),
                row(["Method".into(), if nullish(method) { DASH.into() } else { s(method) }]),
            ],
        ),
        ""
    );

    let coverage = get(r, "coverage");
    if !truthy(get(coverage, "full_period")) {
        let empty = Value::Object(Default::default());
        let started = get(coverage, "collection_started");
        let tail = if truthy(get(period, "has_previous")) {
            "."
        } else {
            ", and there is no previous period to compare with."
        };
        push!(
            format!(
                "> **Partial data:** BlocRA holds {} of the {}-day period ({} of ledgers in that span), collected since {}. Figures cover only the collected time{tail}",
                held_time(if nullish(coverage) { Some(&empty) } else { coverage }),
                s(get(period, "days")),
                fmt_pct(get(coverage, "completeness_pct")),
                if nullish(started) { DASH.into() } else { s(started) },
            ),
            ""
        );
    }

    let k = get(r, "kpis");
    push!("## Scorecard", "");
    let new_accounts = get(k, "new_accounts");
    let new_accounts_cell = if get(new_accounts, "reliable") == Some(&Value::Bool(false)) {
        format!("{} (insufficient history)", fmt_int(get(new_accounts, "value")))
    } else {
        fmt_int(get(new_accounts, "value"))
    };
    let metric = |name: &str| -> [String; 2] {
        let m = get(k, name);
        [fmt_int(get(m, "value")), fmt_change(get(m, "change_pct"))]
    };
    let [aa_v, aa_c] = metric("active_accounts");
    let [calls_v, calls_c] = metric("calls");
    let failed = get(k, "failed_pct");
    let mut rows = vec![
        row(["Active accounts".into(), aa_v, aa_c]),
        row(["New accounts".into(), new_accounts_cell, DASH.into()]),
        row([noun_title.into(), calls_v, calls_c]),
        row([
            "Failure rate".into(),
            fmt_pct(get(failed, "value")),
            fmt_change(get(failed, "change_pct")),
        ]),
    ];
    let volume = get(k, "volume");
    if truthy(volume) {
        rows.push(row([
            "Volume (asset units)".into(),
            fmt_num(get(volume, "value"), 2),
            fmt_change(get(volume, "change_pct")),
        ]));
    }
    rows.push(row([
        format!("{noun_title} per account"),
        fmt_num(get(k, "calls_per_account"), 1),
        DASH.into(),
    ]));
    push!(table(&["Metric", "Value", "Change vs previous period"], &rows), "");

    push!("## Key findings", "");
    for f in items(get(r, "findings")) {
        push!(format!("- {}", s(Some(f))));
    }
    push!("");

    push!("## Activity", "");
    let vol = kind == Some("asset");
    let mut head = vec!["Day (UTC)", "Active accounts", noun_title, "Failed"];
    if vol {
        head.push("Volume");
    }
    let daily: Vec<Vec<String>> = items(get(r, "daily"))
        .iter()
        .map(|d| {
            let d = Some(d);
            let mut cells = vec![
                s(get(d, "day")),
                fmt_int(get(d, "active_accounts")),
                fmt_int(get(d, "calls")),
                fmt_int(get(d, "failed")),
            ];
            if vol {
                cells.push(fmt_num(get(d, "volume"), 2));
            }
            cells
        })
        .collect();
    push!(table(&head, &daily), "");

    let q = get(r, "quality");
    push!("## User quality", "");
    let retention = get(r, "retention");
    let returning = if nullish(get(retention, "returning_pct")) {
        format!("{DASH} (needs a previous period)")
    } else {
        format!(
            "{} ({} of {})",
            fmt_pct(get(retention, "returning_pct")),
            fmt_int(get(retention, "returning_accounts")),
            fmt_int(get(retention, "previous_active"))
        )
    };
    let mut qrows = vec![
        row(["Returning accounts".into(), returning]),
        row(["Top-10 accounts' share".into(), fmt_pct(get(q, "top10_share_pct"))]),
        row([
            format!(
                "Automated accounts (≥ {} {noun} on one day)",
                fmt_int(get(q, "bot_threshold_calls_per_day"))
            ),
            fmt_int(get(q, "bot_like_accounts")),
        ]),
        row([
            "Automated share of activity".into(),
            fmt_pct(get(q, "bot_like_share_pct")),
        ]),
    ];
    if vol && !nullish(get(q, "self_payment_share_pct")) {
        qrows.push(row([
            "Self-payments (excluded from volume)".into(),
            fmt_pct(get(q, "self_payment_share_pct")),
        ]));
    }
    if is_contract && !nullish(get(q, "relayed_share_pct")) {
        qrows.push(row([
            "Calls submitted by a relayer or sponsor (counted for the signing user)".into(),
            format!(
                "{} ({})",
                fmt_pct(get(q, "relayed_share_pct")),
                fmt_int(get(q, "relayed_calls"))
            ),
        ]));
        qrows.push(row([
            "Smart-wallet (contract) accounts".into(),
            fmt_int(get(q, "smart_wallet_accounts")),
        ]));
    }
    let dr = get(r, "declared_relayers");
    let relayers = items(get(dr, "accounts"));
    if !relayers.is_empty() {
        let list = relayers.iter().map(|a| short(Some(a))).collect::<Vec<_>>().join(", ");
        qrows.push(row(["Declared relayers (excluded from account counts)".into(), list]));
        let without = get(dr, "calls_without_user_auth");
        let zero = Value::from(0);
        qrows.push(row([
            "Calls from declared relayers without user authorization".into(),
            fmt_int(if nullish(without) { Some(&zero) } else { without }),
        ]));
    }
    push!(table(&["Measure", "Value"], &qrows), "");
    let top = items(get(q, "top_accounts"));
    if !top.is_empty() {
        let rows: Vec<Vec<String>> = top
            .iter()
            .map(|a| {
                let a = Some(a);
                let account = get(a, "account");
                let link = match explorer {
                    Some(url) => format!("[{}]({url}{})", short(account), s(account)),
                    None => short(account),
                };
                row([
                    link,
                    fmt_int(get(a, "calls")),
                    fmt_pct(get(a, "share_pct")),
                    if truthy(get(a, "bot_like")) { "yes" } else { "no" }.into(),
                ])
            })
            .collect();
        push!(table(&["Top account", noun_title, "Share", "Automated"], &rows), "");
    }

    let functions = items(get(r, "functions"));
    if is_contract && !functions.is_empty() {
        let rows: Vec<Vec<String>> = functions
            .iter()
            .map(|f| {
                let f = Some(f);
                row([
                    format!("`{}`", s(get(f, "function"))),
                    fmt_int(get(f, "calls")),
                    fmt_pct(get(f, "failed_pct")),
                ])
            })
            .collect();
        push!(
            "## Contract functions",
            "",
            table(&["Function", "Calls", "Failure rate"], &rows),
            ""
        );
    }
    let snapshot = get(r, "asset_snapshot");
    if truthy(snapshot) {
        push!(
            "## Asset snapshot",
            "",
            table(
                &["Measure", "Value"],
                &[
                    row(["Holders".into(), fmt_int(get(snapshot, "holders"))]),
                    row([
                        "Supply in accounts".into(),
                        fmt_num(get(snapshot, "supply_in_accounts"), 2)
                    ]),
                    row([
                        "Supply in contracts".into(),
                        fmt_num(get(snapshot, "supply_in_contracts"), 2)
                    ]),
                    row([
                        "Supply in liquidity pools".into(),
                        fmt_num(get(snapshot, "supply_in_liquidity_pools"), 2)
                    ]),
                ],
            ),
            ""
        );
    }

    let e = get(r, "ecosystem");
    let rank = get(e, "rank");
    push!(
        "## Ecosystem position",
        "",
        table(
            &["Measure", "Value"],
            &[
                row([
                    "Rank by active accounts".into(),
                    if nullish(rank) {
                        DASH.into()
                    } else {
                        format!("#{} of {}", s(rank), fmt_int(get(e, "of")))
                    },
                ]),
                row([
                    "Median failure rate of peers".into(),
                    fmt_pct(get(e, "median_failed_pct"))
                ]),
            ],
        ),
        ""
    );

    let empty = Value::Object(Default::default());
    let c = if nullish(coverage) { Some(&empty) } else { coverage };
    let ledger_from = get(c, "ledger_from");
    push!(
        "## Coverage and methodology",
        "",
        table(
            &["Measure", "Value"],
            &[
                row([
                    "Ledger range".into(),
                    if nullish(ledger_from) {
                        DASH.into()
                    } else {
                        format!("{} – {}", fmt_int(ledger_from), fmt_int(get(c, "ledger_to")))
                    },
                ]),
                row(["Ledgers collected".into(), fmt_int(get(c, "ledgers_collected"))]),
                row(["Completeness".into(), fmt_pct(get(c, "completeness_pct"))]),
                row([
                    "Time covered".into(),
                    format!("{} of {} days", held_time(c), s(get(period, "days")))
                ]),
            ],
        ),
        ""
    );
    for m in items(get(r, "methodology")) {
        push!(format!("- {}", s(Some(m))));
    }
    push!("");

    push!("## Verification", "");
    match attestation {
        Some(att) => {
            let a = Some(att);
            push!(
                table(
                    &["Field", "Value"],
                    &[
                        row(["Signed by".into(), format!("`{}` (BlocRA)", s(get(a, "signer")))]),
                        row(["Signature".into(), "Ed25519 over the embedded report".into()]),
                        row(["Report sha256".into(), format!("`{}`", s(get(a, "payload_sha256")))]),
                        row(["Signed at".into(), s(get(a, "signed_at"))]),
                    ],
                ),
                "",
                "Check this file with `npx blocra verify <file>`. Any edit to the figures above breaks verification."
            );
        }
        None => push!("Unsigned report."),
    }
    out.join("\n") + "\n"
}

/// Full Markdown file: body plus the embedded signed envelope.
pub fn render_markdown(envelope: &Value) -> Result<String, String> {
    let text = str_of(get(Some(envelope), "report_json")).ok_or("The envelope has no report_json")?;
    let report: Value = serde_json::from_str(text).map_err(|e| format!("report_json is not valid JSON: {e}"))?;
    let encoded = B64.encode(stringify(envelope, false));
    Ok(format!(
        "{}\n<!-- {MARKER}{encoded} -->\n",
        render_body(&report, get(Some(envelope), "attestation"))
    ))
}

/// Lenient base64 like Node's `Buffer.from(s, "base64")`: ignores characters
/// outside the alphabet (spaces, line breaks), accepts the URL-safe alphabet,
/// and stops at padding.
fn decode_lenient(encoded: &str) -> Option<Vec<u8>> {
    let cleaned: String = encoded
        .chars()
        .take_while(|&c| c != '=')
        .filter_map(|c| match c {
            '-' => Some('+'),
            '_' => Some('/'),
            c if c.is_ascii_alphanumeric() || c == '+' || c == '/' => Some(c),
            _ => None,
        })
        .collect();
    let engine = GeneralPurpose::new(
        &STANDARD,
        GeneralPurposeConfig::new()
            .with_decode_padding_mode(DecodePaddingMode::RequireNone)
            .with_decode_allow_trailing_bits(true),
    );
    let usable = cleaned.len() - usize::from(cleaned.len() % 4 == 1);
    engine.decode(&cleaned[..usable]).ok()
}

/// A Markdown report split into its body and embedded envelope (if any).
pub struct Parsed {
    pub body: String,
    pub envelope: Option<Value>,
}

pub fn parse_markdown(text: &str) -> Parsed {
    let open = format!("<!-- {MARKER}");
    let Some(i) = text.rfind(&open) else {
        return Parsed {
            body: text.to_string(),
            envelope: None,
        };
    };
    let start = i + open.len();
    let end = text[i..]
        .find(" -->")
        .map_or(text.len().saturating_sub(1).max(start), |e| i + e);
    let envelope = text
        .get(start..end)
        .and_then(decode_lenient)
        .and_then(|bytes| serde_json::from_str(&String::from_utf8_lossy(&bytes)).ok());
    let body = &text[..i];
    Parsed {
        body: body.strip_suffix('\n').unwrap_or(body).to_string(),
        envelope,
    }
}

/// True when a Markdown body (from `parse_markdown`) is exactly what
/// `render_body` produces for the signed report, i.e. the text was not edited
/// after signing.
pub fn body_matches_report(body: &str, report: &Value, attestation: Option<&Value>) -> bool {
    let rendered = render_body(report, attestation);
    body.trim_end_matches('\n') == rendered.strip_suffix('\n').unwrap_or(&rendered)
}
