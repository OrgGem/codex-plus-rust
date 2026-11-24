//! The Rust port of `src/script-market-model.ts`.

#![forbid(unsafe_code)]

use crate::js;
use chrono::{DateTime, Duration, Local, NaiveDate, NaiveTime, TimeZone};
use manager_common_rs::{is_nil, is_truthy};
use regex::Regex;
use serde_json::{Value, json};
use std::cmp::Ordering;
use std::sync::LazyLock;

const DT_RE: &str = r"^([0-9]{4}-[0-9]{2}-[0-9]{2})[\s\u{FEFF}]+([0-9]{1,2})(:[0-9]{2}(?::[0-9]{2}(?:\.[0-9]+)?)?)[\s\u{FEFF}]*(.*)$";

fn is_dig(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())
}

fn nil_err(p: &str) -> String {
    format!("TypeError: Cannot read properties of null/undefined (reading '{p}')")
}

fn unix_ms(v: f64) -> Option<f64> {
    (v.is_finite() && v >= 1e8).then(|| js::math_round(if v < 1e11 { v * 1000.0 } else { v }))
}

fn local_ms(d: NaiveDate, t: NaiveTime) -> Option<f64> {
    let f = |x: DateTime<Local>| x.timestamp_millis() as f64;
    Local.from_local_datetime(&d.and_time(t)).earliest().map(f)
}

fn rfc_ms(s: &str) -> Option<f64> {
    DateTime::parse_from_rfc3339(s)
        .ok()
        .map(|x| x.timestamp_millis() as f64)
}

fn parse_iso(s: &str) -> Option<f64> {
    if let Some(ms) = rfc_ms(s) {
        return Some(ms);
    }
    let (dp, tp) = s.split_once('T').map_or((s, None), |(d, t)| (d, Some(t)));
    let p: Vec<_> = dp.split('-').collect();
    if p.len() > 3 || p[0].len() != 4 || !is_dig(p[0]) {
        return None;
    }
    let f = |i: usize| {
        p.get(i).map_or(Some(1i64), |x| {
            (x.len() == 2 && is_dig(x))
                .then(|| x.parse().ok())
                .flatten()
        })
    };
    let (y, m, d) = (p[0].parse().ok()?, f(1)? as u32, f(2)?);
    if !(1..=31).contains(&d) {
        return None;
    }
    let base = NaiveDate::from_ymd_opt(y, m, 1)?.checked_add_signed(Duration::days(d - 1))?;
    let Some(t) = tp else {
        return Some(base.and_time(NaiveTime::MIN).and_utc().timestamp_millis() as f64);
    };
    let (h, tz) = t
        .find(['Z', 'z', '+', '-'])
        .map_or((t, ""), |i| t.split_at(i));
    let nt = NaiveTime::parse_from_str(h, "%H:%M")
        .or_else(|_| NaiveTime::parse_from_str(h, "%H:%M:%S%.f"))
        .ok()?;
    if tz.is_empty() {
        return local_ms(base, nt);
    }
    let fz = if tz.eq_ignore_ascii_case("z") {
        "+00:00"
    } else {
        tz
    };
    let sec = if h.len() == 5 { ":00" } else { "" };
    rfc_ms(&format!("{base}T{h}{sec}{fz}"))
}

fn parse_legacy_num(s: &str) -> Option<f64> {
    if s.contains(['{', '}', ':']) {
        return None;
    }
    let mut num = None;
    for t in s.split(|c: char| !c.is_ascii_alphanumeric()) {
        if is_dig(t) && num.is_none() {
            num = Some(t.parse::<i32>().ok()?);
        } else if t.bytes().any(|b| b.is_ascii_digit()) {
            return None;
        }
    }
    let (y, m) = match num? {
        0 => (2000, 1),
        n @ 1..=12 => (2001, n as u32),
        n @ 32..=49 => (2000 + n, 1),
        n @ 50..=99 => (1900 + n, 1),
        n @ 100.. => (n, 1),
        _ => return None,
    };
    local_ms(NaiveDate::from_ymd_opt(y, m, 1)?, NaiveTime::MIN)
}

fn sort_key(s: &str) -> Vec<u32> {
    const ORD: &[u8] =
        b"\t\n\r _-,;:!?.'\"()[]{}@*/\\&#%`^+<=>|~$0123456789abcdefghijklmnopqrstuvwxyz";
    const MAP: &str = "iİïìíî eéèêë uüùúû oöòóôõ aäàáâãå nñ cç";
    let (mut out, mut it) = (Vec::new(), s.chars().peekable());
    while let Some(c) = it.next() {
        if c.is_ascii_digit() {
            let mut d = String::from(c);
            while it.peek().is_some_and(char::is_ascii_digit) {
                d.push(it.next().unwrap_or('0'));
            }
            let t = d.trim_start_matches('0');
            out.extend(
                [37, t.len() as u32]
                    .into_iter()
                    .chain(t.bytes().map(u32::from)),
            );
        } else if matches!(c as u32, 0..=8 | 14..=31 | 127 | 0xFEFF | 0x0300..=0x036F) {
        } else if c == 'ß' {
            out.extend([65, 65]);
        } else {
            let fb = MAP
                .split(' ')
                .find_map(|g| g.contains(c).then(|| g.as_bytes()[0]))
                .or(u8::try_from(c).ok());
            if let Some(p) = fb.and_then(|b| ORD.iter().position(|&x| x == b.to_ascii_lowercase()))
            {
                out.push(p as u32 + 1);
            } else if matches!(c, '\u{2000}'..='\u{2BFF}' | '\u{1F000}'..='\u{1FAFF}') {
                out.extend([38, 0x10_0000 + c as u32]);
            } else {
                out.push(0x20_0000 + c.to_lowercase().next().unwrap_or(c) as u32);
            }
        }
    }
    out
}

/// Port of `parseMarketTime` (`src/script-market-model.ts:28`).
pub fn parse_market_time(value: Value) -> Result<Option<f64>, String> {
    static RE: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(DT_RE).ok());
    if let Some(num) = js::as_number(&value) {
        return Ok(unix_ms(num));
    }
    let text = match &value {
        Value::Null => return Ok(None),
        v if js::is_undefined(v) => return Ok(None),
        Value::String(s) => {
            s.trim_matches(|c: char| c != '\u{0085}' && (c.is_whitespace() || c == '\u{FEFF}'))
        }
        _ => return Err("TypeError: (value ?? \"\").trim is not a function".into()),
    };
    if text.is_empty() {
        return Ok(None);
    }
    let is_num = match text.split_once('.') {
        Some((i, f)) => is_dig(i) && is_dig(f),
        None => is_dig(text) && text.len() != 4,
    };
    if is_num {
        return Ok(unix_ms(text.parse::<f64>().unwrap_or(f64::NAN)));
    }
    let norm = RE.as_ref().and_then(|r| r.captures(text)).map(|c| {
        let z = c[4].to_ascii_lowercase();
        let off = if z == "utc" || z == "gmt" { "Z" } else { &c[4] };
        format!("{}T{:0>2}{}{off}", &c[1], &c[2], &c[3])
    });
    let s = norm.as_deref().unwrap_or(text);
    Ok(parse_iso(s).or_else(|| parse_legacy_num(s)))
}

fn updated_at(s: &Value) -> Result<Value, String> {
    if is_nil(s) {
        return Err(nil_err("updatedAt"));
    }
    Ok(s.get("updatedAt").cloned().unwrap_or_else(js::undefined))
}

/// Port of `defaultMarketSort` (`src/script-market-model.ts:50`).
pub fn default_market_sort(scripts: Value) -> Result<String, String> {
    if is_nil(&scripts) {
        return Err(nil_err("length"));
    }
    let Some(arr) = scripts.as_array() else {
        let has_len = js::string_units(&scripts).is_some_and(|u| !u.is_empty())
            || scripts.get("length").is_some_and(is_truthy);
        if has_len {
            return Err("TypeError: scripts.every is not a function".into());
        }
        return Ok("default".into());
    };
    for s in arr {
        if parse_market_time(updated_at(s)?)?.is_none() {
            return Ok("default".into());
        }
    }
    Ok((if arr.is_empty() { "default" } else { "updated" }).into())
}

/// Port of `sortMarketScripts` (`src/script-market-model.ts:56`).
pub fn sort_market_scripts(scripts: Vec<Value>, sort: String) -> Result<Vec<Value>, String> {
    if sort == "default" {
        return Ok(scripts);
    }
    let mut rows = Vec::with_capacity(scripts.len());
    for (i, s) in scripts.into_iter().enumerate() {
        let t = parse_market_time(updated_at(&s)?)?;
        rows.push((s, i, t));
    }
    let by_upd = sort == "updated";
    if !by_upd && rows.iter().any(|(s, _, _)| !s["name"].is_string()) {
        return Err("TypeError: left.script.name.localeCompare is not a function".into());
    }
    rows.sort_by(|l, r| {
        let o = if by_upd {
            match (l.2, r.2) {
                (Some(a), Some(b)) => b.partial_cmp(&a).unwrap_or(Ordering::Equal),
                (a, b) => b.is_some().cmp(&a.is_some()),
            }
        } else {
            sort_key(l.0["name"].as_str().unwrap_or(""))
                .cmp(&sort_key(r.0["name"].as_str().unwrap_or("")))
        };
        o.then(l.1.cmp(&r.1))
    });
    Ok(rows.into_iter().map(|(s, _, _)| s).collect())
}

/// Port of `relativeAge` (`src/script-market-model.ts:80`).
pub fn relative_age(time: f64, now: f64) -> Result<Value, String> {
    let (h, d, df) = (3_600_000.0, 86_400_000.0, now - time);
    let e = if df.is_nan() { df } else { df.max(0.0) };
    if e < h {
        return Ok(json!({ "unit": "justNow" }));
    }
    for (lim, u, q) in [
        (d, "hours", h),
        (30. * d, "days", d),
        (365. * d, "months", 30. * d),
    ] {
        if e < lim {
            return Ok(json!({ "unit": u, "value": js::number((e / q).floor()) }));
        }
    }
    Ok(json!({ "unit": "years", "value": js::number((e / (365.0 * d)).floor()) }))
}

/// Port of `localScriptState` (`src/script-market-model.ts:100`).
pub fn local_script_state(script: Value, global_enabled: bool) -> Result<String, String> {
    if is_nil(&script) {
        return Err(nil_err("status"));
    }
    let st = script["status"].as_str();
    let res = match (st, is_truthy(&script["enabled"]), global_enabled) {
        (Some("unknown"), _, _) => "unknown",
        (_, false, _) => "off",
        (_, _, false) => "globalOff",
        (Some("loaded"), _, _) => "loaded",
        (Some("loading"), _, _) => "loading",
        (Some("failed"), _, _) => "failed",
        _ => "notLoaded",
    };
    Ok(res.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn port_smoke() {
        assert_eq!(
            parse_market_time(json!("2026-10-29 12:00:00 +08:00")),
            Ok(Some(17932464e5))
        );
        assert_eq!(parse_market_time(json!("key-1{name} x")), Ok(None));
        let k = |n: &str| json!({"name": n});
        let xs = vec![k("Item 10"), k("A"), k("item 2")];
        assert_eq!(
            default_market_sort(Value::Array(xs.clone())),
            Ok("default".into())
        );
        assert_eq!(default_market_sort(json!(0)), Ok("default".into()));
        let s = sort_market_scripts(xs, "name".into()).unwrap_or_default();
        assert_eq!(s[1]["name"], json!("item 2"));
        assert_eq!(
            relative_age(0.0, 7_200_000.0),
            Ok(json!({"unit":"hours","value":2}))
        );
        assert_eq!(
            local_script_state(json!({"enabled":true,"status":"loaded"}), true),
            Ok("loaded".into())
        );
    }
}
