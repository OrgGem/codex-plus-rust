//! The Rust port of `release-meta.mjs`.

#![forbid(unsafe_code)]

use crate::js;
use regex::RegexBuilder;
use serde_json::Value;

const CHANGELOG: &str = include_str!("../../../../../CHANGELOG.md");

fn js_num(s: &str) -> f64 {
    let t = s.trim_matches(|c: char| c.is_whitespace() || c == '\u{FEFF}');
    if t.is_empty() {
        0.0
    } else {
        t.parse().unwrap_or(f64::NAN)
    }
}

fn parse_ver(s: &str) -> (Vec<f64>, &str) {
    let v = s.strip_prefix(['v', 'V']).unwrap_or(s);
    let mut sp = v.split('-');
    let core = sp.next().unwrap_or("");
    let pre = sp.next().unwrap_or("");
    (core.split('.').map(js_num).collect(), pre)
}

/// Port of `changelogSection` (`release-meta.mjs:89`).
pub fn changelog_section(version: Value, text: Value) -> Result<Value, String> {
    let Some(ver) = version.as_str() else {
        return Err("TypeError: text.replace is not a function".into());
    };
    let body = if js::is_undefined(&text) {
        CHANGELOG
    } else {
        text.as_str()
            .ok_or("TypeError: text.split is not a function")?
    };
    let lines: Vec<&str> = body.split('\n').collect();
    let pat = format!(r"^## \[?v?{}\]?(?:\s|$)", regex::escape(ver));
    let re = RegexBuilder::new(&pat)
        .unicode(true)
        .build()
        .map_err(|e| e.to_string())?;
    let Some(start) = lines.iter().position(|l| re.is_match(l)) else {
        return Ok(Value::String(String::new()));
    };
    let end = lines
        .iter()
        .enumerate()
        .skip(start + 1)
        .find(|(_, l)| l.starts_with("## "))
        .map_or(lines.len(), |(i, _)| i);
    let section = lines[start + 1..end].join("\n");
    Ok(Value::String(
        section
            .trim_matches(|c: char| c.is_whitespace() || c == '\u{FEFF}')
            .into(),
    ))
}

/// Port of `compareVersions` (`release-meta.mjs:100`).
pub fn compare_versions(a: Value, b: Value) -> Result<Value, String> {
    let (Some(sa), Some(sb)) = (a.as_str(), b.as_str()) else {
        return Err("TypeError: v.replace is not a function".into());
    };
    let ((pa, pre_a), (pb, pre_b)) = (parse_ver(sa), parse_ver(sb));
    for i in 0..3 {
        let va = pa
            .get(i)
            .copied()
            .filter(|x| !x.is_nan() && *x != 0.0)
            .unwrap_or(0.0);
        let vb = pb
            .get(i)
            .copied()
            .filter(|x| !x.is_nan() && *x != 0.0)
            .unwrap_or(0.0);
        let diff = va - vb;
        if !diff.is_nan() && diff != 0.0 {
            return Ok(js::number(diff));
        }
    }
    let ord = if pre_a == pre_b {
        0
    } else if pre_a.is_empty() {
        1
    } else if pre_b.is_empty() || pre_a.encode_utf16().lt(pre_b.encode_utf16()) {
        -1
    } else {
        1
    };
    Ok(Value::from(ord))
}
