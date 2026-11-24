//! The Rust port of `src/management-model.ts`.

#![forbid(unsafe_code)]

use crate::js;
use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, NaiveTime, TimeZone};
use serde_json::{Value, json};

fn is_ws(c: char) -> bool {
    c.is_whitespace() || c == '\u{FEFF}'
}

fn ts_of(s: &Value) -> f64 {
    s.get("updatedAtMs").and_then(js::as_number).unwrap_or(0.0)
}

fn is_drv(s: &str, lo: bool) -> bool {
    let b = s.as_bytes();
    let a = |c: &u8| c.is_ascii_alphabetic() && (!lo || c.is_ascii_lowercase());
    b.first().is_some_and(a) && b.get(1) == Some(&b':') && matches!(b.get(2), None | Some(&b'/'))
}

/// Port of `matchesManagementQuery` (`src/management-model.ts:6`).
pub fn matches_management_query(values: Vec<String>, query: String) -> Result<bool, String> {
    let (q, t) = (query.to_lowercase(), values.join(" ").to_lowercase());
    Ok(q.split(is_ws).all(|w| w.is_empty() || t.contains(w)))
}

/// Port of `normalizeProjectPath` (`src/management-model.ts:17`).
pub fn normalize_project_path(cwd: String) -> Result<String, String> {
    let s = cwd.replace('\\', "/");
    let mut p = s.trim_end_matches('/');
    if let Some(r) = p.strip_prefix("//?/").or_else(|| p.strip_prefix("//./")) {
        if r.get(..4).is_some_and(|h| h.eq_ignore_ascii_case("unc/")) {
            return Ok(format!("//{}", &r[4..]));
        }
        if is_drv(r, false) {
            p = r;
        }
    }
    if !is_drv(p, true) {
        return Ok(p.into());
    }
    let c = p.as_bytes()[0].to_ascii_uppercase() as char;
    Ok(format!("{c}{}", &p[1..]))
}

fn mid_ms(d: NaiveDate) -> f64 {
    d.and_time(NaiveTime::MIN)
        .and_local_timezone(Local)
        .earliest()
        .map_or(f64::NAN, |x| x.timestamp_millis() as f64)
}

fn day_boundaries(ms: f64) -> (f64, f64, f64) {
    let ok = ms.is_finite() && ms.abs() <= 8.64e15;
    let s = (ms / 1000.0).floor() as i64;
    let ns = ((ms - s as f64 * 1000.0) * 1e6).clamp(0.0, 999_999_999.0) as u32;
    let Some(dt) = ok.then(|| Local.timestamp_opt(s, ns).single()).flatten() else {
        return (f64::NAN, f64::NAN, f64::NAN);
    };
    let d = dt.date_naive();
    let w = (d.weekday().num_days_from_sunday() as i64 + 6) % 7;
    let dd = Duration::days;
    (mid_ms(d), mid_ms(d - dd(1)), mid_ms(d - dd(w)))
}

fn parse_now_str(s: &str) -> f64 {
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return dt.timestamp_millis() as f64;
    }
    let mut it = s
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter_map(|t| t.parse::<i32>().ok());
    if let (Some(y), None) = (it.next(), it.next())
        && (0..=99).contains(&y)
    {
        return NaiveDate::from_ymd_opt(2000 + y, 1, 1).map_or(f64::NAN, mid_ms);
    }
    f64::NAN
}

/// Port of `sessionGroups` (`src/management-model.ts:24`).
pub fn session_groups(sessions: Vec<Value>, mode: String, now: Value) -> Result<Value, String> {
    let mut sorted = sessions;
    js::sort(&mut sorted, |a, b| {
        let d = ts_of(b) - ts_of(a);
        if d.is_nan() { 0.0 } else { d }
    });
    let now_ms = match &now {
        v if js::is_undefined(v) => Local::now().timestamp_millis() as f64,
        Value::Null | Value::Bool(false) => 0.0,
        Value::Bool(true) => 1.0,
        Value::String(s) => parse_now_str(s),
        Value::Array(a) => {
            let f = |x: &Value| {
                x.as_str()
                    .map(str::to_string)
                    .or_else(|| js::as_number(x).map(js::number_to_string))
            };
            parse_now_str(&a.iter().filter_map(f).collect::<Vec<_>>().join(","))
        }
        v => js::as_number(v).unwrap_or(f64::NAN),
    };
    let (t0, t1, t2) = day_boundaries(now_ms);
    let is_proj = mode == "project";
    let mut groups: Vec<Value> = Vec::new();
    for s in sorted {
        let cwd = s
            .get("cwd")
            .and_then(Value::as_str)
            .ok_or("TypeError: Cannot read properties of undefined (reading 'replace')")?;
        let path = normalize_project_path(cwd.into())?;
        let ts = ts_of(&s);
        let (k, lbl) = if is_proj {
            (
                path.as_str(),
                path.rsplit('/')
                    .next()
                    .filter(|t| !t.is_empty())
                    .unwrap_or("未知目录"),
            )
        } else if ts >= t0 {
            ("today", "今天")
        } else if ts >= t1 {
            ("yesterday", "昨天")
        } else if ts >= t2 {
            ("week", "本周")
        } else {
            ("earlier", "更早")
        };
        match groups.iter_mut().find(|g| g["key"] == k) {
            Some(g) => if let Some(Value::Array(arr)) = g.get_mut("sessions") { arr.push(s); },
            None => groups.push(json!({ "key": k, "label": lbl, "path": if is_proj { &path } else { "" }, "sessions": [s] })),
        }
    }
    Ok(Value::Array(groups))
}

/// Port of `skillRepoKey` (`src/management-model.ts:39`).
pub fn skill_repo_key(repo: Value) -> Result<String, String> {
    if repo.is_null() || js::is_undefined(&repo) {
        return Err("TypeError: Cannot read properties of null/undefined (reading 'owner')".into());
    }
    let f = |k| repo.get(k).and_then(Value::as_str).unwrap_or("undefined");
    let sub = repo.get("subdir").and_then(Value::as_str).unwrap_or("");
    let sep = if sub.is_empty() { "" } else { ":" };
    Ok(format!(
        "{}/{}@{}{sep}{sub}",
        f("owner"),
        f("name"),
        f("branch")
    ))
}
