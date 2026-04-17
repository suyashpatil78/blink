use meval::eval_str;
use once_cell::sync::Lazy;
use regex::Regex;

static CALC_CHARS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[0-9+\-*/().\s^%]+$").expect("calculator char regex"));

static PURE_NUMBER: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\d+(\.\d+)?$").expect("pure number regex"));

pub fn try_eval(expr: &str) -> Option<String> {
    let s = expr.trim();
    if s.is_empty() {
        return None;
    }

    if !CALC_CHARS.is_match(s) {
        return None;
    }

    if PURE_NUMBER.is_match(s) {
        return None;
    }

    let value = eval_str(s).ok()?;

    if !value.is_finite() {
        return Some(value.to_string());
    }

    Some(pretty_float(value))
}

fn pretty_float(n: f64) -> String {
    let rounded = (n * 1e12).round() / 1e12;
    if (rounded - rounded.round()).abs() < 1e-9 {
        return format!("{}", rounded as i64);
    }

    let mut out = format!("{:.10}", rounded);
    while out.contains('.') && (out.ends_with('0') || out.ends_with('.')) {
        out.pop();
    }
    out
}
