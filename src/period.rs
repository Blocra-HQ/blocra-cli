//! The time frame of a report: the last N days (`--days`), a named or
//! relative range (`--range today|7d|30d|1m|1y|all|...`), or explicit UTC
//! dates (`--from YYYY-MM-DD [--to YYYY-MM-DD]`). The API turns any of them
//! into whole UTC days; the signed report states the exact days covered.

use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Period {
    Days(u32),
    Range(String),
    Span { from: String, to: Option<String> },
}

fn is_date(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() == 10
        && b[4] == b'-'
        && b[7] == b'-'
        && b.iter()
            .enumerate()
            .all(|(i, c)| i == 4 || i == 7 || c.is_ascii_digit())
}

fn is_range(s: &str) -> bool {
    matches!(s, "all" | "today" | "yesterday")
        || (s.len() > 1
            && s[..s.len() - 1].bytes().all(|c| c.is_ascii_digit())
            && s[..s.len() - 1].parse::<u64>().is_ok_and(|n| n > 0)
            && matches!(s.as_bytes()[s.len() - 1], b'h' | b'd' | b'w' | b'm' | b'y'))
}

fn days_of(s: &str) -> Result<u32, String> {
    match s.trim().parse::<f64>() {
        Ok(d) if d >= 1.0 && d.fract() == 0.0 && d <= f64::from(u32::MAX) => Ok(d as u32),
        _ => Err("--days must be a whole number of days (1 or more).".into()),
    }
}

impl Period {
    /// From the command-line flags (at most one kind), else from blocra.json
    /// (`range` or `days`), else the last `default_days` days.
    pub fn resolve(
        days: Option<String>,
        range: Option<String>,
        from: Option<String>,
        to: Option<String>,
        config: Option<&Value>,
        default_days: u32,
    ) -> Result<Period, String> {
        let given = [days.is_some(), range.is_some(), from.is_some() || to.is_some()]
            .iter()
            .filter(|g| **g)
            .count();
        if given > 1 {
            return Err("Give one of --days, --range or --from/--to.".into());
        }
        if let Some(d) = days {
            return Ok(Period::Days(days_of(&d)?));
        }
        if let Some(r) = range {
            return Period::range(&r);
        }
        if from.is_some() || to.is_some() {
            let from = from.ok_or("--to needs --from.")?;
            if !is_date(&from) || to.as_deref().is_some_and(|t| !is_date(t)) {
                return Err("--from and --to take UTC dates: YYYY-MM-DD.".into());
            }
            if to.as_deref().is_some_and(|t| t < from.as_str()) {
                return Err("--from is after --to.".into());
            }
            return Ok(Period::Span { from, to });
        }
        let cfg = |k: &str| config.and_then(|c| c.get(k)).filter(|v| !v.is_null());
        if let Some(r) = cfg("range").and_then(Value::as_str) {
            return Period::range(r);
        }
        match cfg("days") {
            Some(Value::Number(n)) => Ok(Period::Days(days_of(&n.to_string())?)),
            Some(Value::String(s)) => Ok(Period::Days(days_of(s)?)),
            Some(_) => Err("\"days\" in blocra.json must be a number.".into()),
            None => Ok(Period::Days(default_days)),
        }
    }

    fn range(r: &str) -> Result<Period, String> {
        let r = r.trim().to_ascii_lowercase();
        if is_range(&r) {
            Ok(Period::Range(r))
        } else {
            Err(format!(
                "Unknown range \"{r}\": use today, yesterday, all, or a number and unit such as 7d, 2w, 1m, 1y."
            ))
        }
    }

    /// Query parameters for /api/reports/traction.
    pub fn query(&self) -> Vec<(&'static str, String)> {
        match self {
            Period::Days(d) => vec![("days", d.to_string())],
            Period::Range(r) => vec![("range", r.clone())],
            Period::Span { from, to } => {
                let mut q = vec![("from", from.clone())];
                if let Some(t) = to {
                    q.push(("to", t.clone()));
                }
                q
            }
        }
    }

    /// For messages: "30-day", "all-time", "2026-11-01 to 2026-11-16".
    pub fn describe(&self) -> String {
        match self {
            Period::Days(d) => format!("{d}-day"),
            Period::Range(r) if r == "all" => "all-time".into(),
            Period::Range(r) => r.clone(),
            Period::Span { from, to: Some(to) } => format!("{from} to {to}"),
            Period::Span { from, to: None } => format!("since-{from}"),
        }
    }

    /// The blocra.json entries for this period.
    pub fn config_entries(&self) -> Vec<(&'static str, Value)> {
        match self {
            Period::Days(d) => vec![("days", Value::from(*d))],
            Period::Range(r) => vec![("range", Value::from(r.clone()))],
            Period::Span { from, to } => vec![
                ("from", Value::from(from.clone())),
                ("to", to.clone().map_or(Value::Null, Value::from)),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn r(
        days: Option<&str>,
        range: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
        config: Option<Value>,
    ) -> Result<Period, String> {
        let s = |x: Option<&str>| x.map(String::from);
        Period::resolve(s(days), s(range), s(from), s(to), config.as_ref(), 7)
    }

    #[test]
    fn resolves_flags_config_and_defaults() {
        assert_eq!(r(None, None, None, None, None), Ok(Period::Days(7)));
        assert_eq!(r(Some("30"), None, None, None, None), Ok(Period::Days(30)));
        assert_eq!(r(None, Some("ALL"), None, None, None), Ok(Period::Range("all".into())));
        assert_eq!(r(None, Some("1y"), None, None, None), Ok(Period::Range("1y".into())));
        assert_eq!(
            r(None, None, Some("2026-11-01"), Some("2026-11-10"), None),
            Ok(Period::Span {
                from: "2026-11-01".into(),
                to: Some("2026-11-10".into())
            })
        );
        assert_eq!(
            r(None, None, None, None, Some(json!({"range": "1m"}))),
            Ok(Period::Range("1m".into()))
        );
        assert_eq!(
            r(None, None, None, None, Some(json!({"days": 30}))),
            Ok(Period::Days(30))
        );
        // Flags win over the config.
        assert_eq!(
            r(None, Some("today"), None, None, Some(json!({"days": 30}))),
            Ok(Period::Range("today".into()))
        );
    }

    #[test]
    fn rejects_bad_periods() {
        for (d, rg, f, t) in [
            (Some("7"), Some("7d"), None, None),
            (None, Some("7d"), Some("2026-11-01"), None),
            (None, Some("forever"), None, None),
            (None, Some("0d"), None, None),
            (None, Some("d"), None, None),
            (Some("0"), None, None, None),
            (Some("2.5"), None, None, None),
            (None, None, None, Some("2026-11-01")),
            (None, None, Some("1 Nov"), None),
            (None, None, Some("2026-11-10"), Some("2026-11-01")),
        ] {
            assert!(r(d, rg, f, t, None).is_err(), "{d:?} {rg:?} {f:?} {t:?}");
        }
    }

    #[test]
    fn builds_queries_and_labels() {
        assert_eq!(Period::Days(30).query(), vec![("days", "30".to_string())]);
        assert_eq!(Period::Range("all".into()).query(), vec![("range", "all".to_string())]);
        assert_eq!(
            Period::Span {
                from: "2026-11-01".into(),
                to: None
            }
            .query(),
            vec![("from", "2026-11-01".to_string())]
        );
        assert_eq!(Period::Range("all".into()).describe(), "all-time");
        assert_eq!(Period::Days(30).describe(), "30-day");
    }
}
