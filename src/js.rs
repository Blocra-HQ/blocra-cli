//! JavaScript-compatible value handling and formatting.
//!
//! Reports are rendered by this CLI and by the BlocRA website (in the
//! browser), and the two outputs must be byte-identical so a report exported
//! from either one verifies with the other. These helpers reproduce the
//! JavaScript rules the website's renderer relies on: `String(x)`,
//! `Number.prototype.toString`, `toLocaleString("en-US")`, `Math.round`,
//! `JSON.stringify` and truthiness. A field that is absent is `None` (JS
//! `undefined`); a field that is `null` is `Some(Value::Null)`.

use serde_json::Value;

pub type Js<'a> = Option<&'a Value>;

/// `v?.key`: the property when `v` is an object, otherwise undefined.
pub fn get<'a>(v: Js<'a>, key: &str) -> Js<'a> {
    match v {
        Some(Value::Object(m)) => m.get(key),
        _ => None,
    }
}

/// `v == null`: undefined or null.
pub fn nullish(v: Js) -> bool {
    matches!(v, None | Some(Value::Null))
}

/// `a ?? b`.
pub fn or<'a>(a: Js<'a>, b: Js<'a>) -> Js<'a> {
    if nullish(a) {
        b
    } else {
        a
    }
}

/// JavaScript truthiness.
pub fn truthy(v: Js) -> bool {
    match v {
        None | Some(Value::Null) => false,
        Some(Value::Bool(b)) => *b,
        Some(Value::Number(n)) => n.as_f64().is_some_and(|f| f != 0.0 && !f.is_nan()),
        Some(Value::String(s)) => !s.is_empty(),
        Some(_) => true,
    }
}

/// The number, when the value is a (finite) number.
pub fn num(v: Js) -> Option<f64> {
    match v {
        Some(Value::Number(n)) => n.as_f64().filter(|f| f.is_finite()),
        _ => None,
    }
}

/// The string, when the value is a string.
pub fn str_of<'a>(v: Js<'a>) -> Option<&'a str> {
    v.and_then(Value::as_str)
}

/// The array items, or none.
pub fn items<'a>(v: Js<'a>) -> &'a [Value] {
    match v {
        Some(Value::Array(a)) => a,
        _ => &[],
    }
}

/// `String(v)`, as used in template literals.
pub fn to_str(v: Js) -> String {
    match v {
        None => "undefined".into(),
        Some(Value::Null) => "null".into(),
        Some(Value::Bool(b)) => b.to_string(),
        Some(Value::Number(n)) => number_to_string(n.as_f64().unwrap_or(f64::NAN)),
        Some(Value::String(s)) => s.clone(),
        Some(Value::Array(a)) => a
            .iter()
            .map(|x| {
                if nullish(Some(x)) {
                    String::new()
                } else {
                    to_str(Some(x))
                }
            })
            .collect::<Vec<_>>()
            .join(","),
        Some(Value::Object(_)) => "[object Object]".into(),
    }
}

/// `Number.prototype.toString()`: shortest round-trip digits, exponent
/// notation below 1e-6 and from 1e21.
pub fn number_to_string(n: f64) -> String {
    if n.is_nan() {
        return "NaN".into();
    }
    if n.is_infinite() {
        return if n > 0.0 { "Infinity" } else { "-Infinity" }.into();
    }
    if n == 0.0 {
        return "0".into();
    }
    let a = n.abs();
    if (1e-6..1e21).contains(&a) {
        return format!("{n}");
    }
    // Rust writes "1e21" / "1.5e-7"; JavaScript writes "1e+21" / "1.5e-7".
    let s = format!("{n:e}");
    match s.split_once('e') {
        Some((m, e)) if !e.starts_with('-') => format!("{m}e+{e}"),
        _ => s,
    }
}

/// `Math.round`: halves round toward +Infinity, and -0 is kept.
pub fn math_round(n: f64) -> f64 {
    let f = n.floor();
    let r = if n - f >= 0.5 { f + 1.0 } else { f };
    if r == 0.0 && (n < 0.0 || n.is_sign_negative()) {
        -0.0
    } else {
        r
    }
}

/// `n.toLocaleString("en-US", { maximumFractionDigits: digits })`:
/// thousands separators, at most `digits` decimals (half away from zero),
/// trailing zeros dropped.
pub fn locale(n: f64, digits: usize) -> String {
    let negative = n.is_sign_negative();
    // ICU (behind Intl in browsers and Node) rounds the shortest round-trip
    // decimal form, not the exact binary value: 2.675 becomes "2.68". Rust's
    // `{}` gives that same shortest form, without exponent notation.
    let exact = format!("{}", n.abs());
    let (int_part, frac_part) = exact.split_once('.').unwrap_or((&exact, ""));
    let kept_frac = frac_part.bytes().chain(std::iter::repeat(b'0')).take(digits);
    let mut digits_vec: Vec<u8> = int_part.bytes().chain(kept_frac).map(|b| b - b'0').collect();
    let round_up = frac_part.as_bytes().get(digits).is_some_and(|&b| b >= b'5');
    if round_up {
        let mut i = digits_vec.len();
        loop {
            if i == 0 {
                digits_vec.insert(0, 1);
                break;
            }
            i -= 1;
            if digits_vec[i] == 9 {
                digits_vec[i] = 0;
            } else {
                digits_vec[i] += 1;
                break;
            }
        }
    }
    let int_len = digits_vec.len() - digits;
    let int_digits: String = digits_vec[..int_len].iter().map(|d| char::from(b'0' + d)).collect();
    let frac: String = digits_vec[int_len..].iter().map(|d| char::from(b'0' + d)).collect();
    let frac = frac.trim_end_matches('0');
    let int_digits = int_digits.trim_start_matches('0');
    let int_digits = if int_digits.is_empty() { "0" } else { int_digits };
    let mut grouped = String::new();
    for (i, c) in int_digits.chars().enumerate() {
        if i > 0 && (int_digits.len() - i) % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(c);
    }
    let sign = if negative { "-" } else { "" };
    if frac.is_empty() {
        format!("{sign}{grouped}")
    } else {
        format!("{sign}{grouped}.{frac}")
    }
}

/// `JSON.stringify(v)` (compact) or `JSON.stringify(v, null, 2)` (pretty).
pub fn stringify(v: &Value, pretty: bool) -> String {
    let mut out = String::new();
    write_json(&mut out, v, pretty, 0);
    out
}

fn write_json(out: &mut String, v: &Value, pretty: bool, depth: usize) {
    let newline = |out: &mut String, depth: usize| {
        if pretty {
            out.push('\n');
            out.push_str(&"  ".repeat(depth));
        }
    };
    match v {
        Value::Null => out.push_str("null"),
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Number(n) => {
            let f = n.as_f64().unwrap_or(f64::NAN);
            out.push_str(&if f.is_finite() {
                number_to_string(f)
            } else {
                "null".into()
            });
        }
        Value::String(s) => write_json_string(out, s),
        Value::Array(a) => {
            if a.is_empty() {
                out.push_str("[]");
                return;
            }
            out.push('[');
            for (i, x) in a.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                newline(out, depth + 1);
                write_json(out, x, pretty, depth + 1);
            }
            newline(out, depth);
            out.push(']');
        }
        Value::Object(m) => {
            if m.is_empty() {
                out.push_str("{}");
                return;
            }
            out.push('{');
            for (i, (k, x)) in m.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                newline(out, depth + 1);
                write_json_string(out, k);
                out.push_str(if pretty { ": " } else { ":" });
                write_json(out, x, pretty, depth + 1);
            }
            newline(out, depth);
            out.push('}');
        }
    }
}

fn write_json_string(out: &mut String, s: &str) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_print_like_javascript() {
        assert_eq!(number_to_string(30.0), "30");
        assert_eq!(number_to_string(0.1), "0.1");
        assert_eq!(number_to_string(-0.0), "0");
        assert_eq!(number_to_string(1.5e-7), "1.5e-7");
        assert_eq!(number_to_string(1e21), "1e+21");
        assert_eq!(number_to_string(123456789012345680000.0), "123456789012345680000");
        assert_eq!(number_to_string(0.000001), "0.000001");
    }

    #[test]
    fn locale_matches_intl() {
        assert_eq!(locale(1234567.895, 2), "1,234,567.9");
        assert_eq!(locale(0.125, 2), "0.13");
        assert_eq!(locale(0.125, 1), "0.1");
        assert_eq!(locale(2.675, 2), "2.68");
        assert_eq!(locale(999.96, 1), "1,000");
        assert_eq!(locale(-0.001, 2), "-0");
        assert_eq!(locale(0.0, 0), "0");
        assert_eq!(locale(1e21, 0), "1,000,000,000,000,000,000,000");
    }

    #[test]
    fn math_round_halves_go_up() {
        assert_eq!(math_round(2.5), 3.0);
        assert_eq!(math_round(-2.5), -2.0);
        assert!(math_round(-0.3).is_sign_negative());
        assert_eq!(math_round(1.4999999999999998), 1.0);
    }
}
