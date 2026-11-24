//! The Rust port of `src/agent-cache-model.ts`.

#![forbid(unsafe_code)]

use crate::js;
use manager_common_rs::js_rt::{is_nil, is_truthy, to_number};
use serde_json::{Value, json};

const KIND_ORDER: [&str; 4] = ["cache", "update", "logs", "temporary"];

fn num(v: &Value, key: &str) -> f64 {
    v.get(key).map_or(f64::NAN, to_number)
}

fn same_vz(a: &Value, b: &Value) -> bool {
    match (js::as_number(a), js::as_number(b)) {
        (Some(x), Some(y)) => (x.is_nan() && y.is_nan()) || x == y,
        _ => a == b,
    }
}

fn push_unique(list: &mut Vec<Value>, val: Value) {
    if !list.iter().any(|x| same_vz(x, &val)) {
        list.push(val);
    }
}

fn select_eligible(groups: &[Value]) -> Result<Vec<Value>, String> {
    let mut out = Vec::new();
    for g in groups {
        if is_nil(g) {
            return Err("TypeError: Cannot read properties of null/undefined".into());
        }
        if g.get("cleanable").is_some_and(is_truthy) && num(g, "eligibleFiles") > 0.0 {
            out.push(g.clone());
        }
    }
    Ok(out)
}

/// Port of `selectableCacheGroups` (`src/agent-cache-model.ts:13`).
pub fn selectable_cache_groups(groups: Vec<Value>) -> Result<Value, String> {
    Ok(Value::Array(select_eligible(&groups)?))
}

/// Port of `cacheSelection` (`src/agent-cache-model.ts:18`).
pub fn cache_selection(groups: Vec<Value>, mode: String) -> Result<Value, String> {
    let out = select_eligible(&groups)?
        .into_iter()
        .filter(|g| mode == "all" || g.get("kind").and_then(Value::as_str) != Some("logs"))
        .map(|g| g.get("id").cloned().unwrap_or_else(js::undefined))
        .collect();
    Ok(Value::Array(out))
}

/// Port of `cacheCleanupSummary` (`src/agent-cache-model.ts:25`).
pub fn cache_cleanup_summary(groups: Vec<Value>) -> Result<Value, String> {
    let (mut apps, mut files, mut bytes) = (Vec::new(), 0.0_f64, 0.0_f64);
    for g in &select_eligible(&groups)? {
        push_unique(
            &mut apps,
            g.get("app").cloned().unwrap_or_else(js::undefined),
        );
        files += num(g, "eligibleFiles");
        bytes += num(g, "eligibleBytes");
    }
    Ok(json!({ "apps": apps, "files": js::number(files), "bytes": js::number(bytes) }))
}

/// Port of `cacheKindTotals` (`src/agent-cache-model.ts:38`).
pub fn cache_kind_totals(groups: Value) -> Result<Value, String> {
    let slice = match &groups {
        Value::Array(a) => a.as_slice(),
        Value::String(_) => return Ok(json!([])),
        _ => return Err("TypeError: groups is not iterable".into()),
    };
    let mut totals: Vec<(Value, f64, f64)> = Vec::new();
    for g in slice {
        if is_nil(g) {
            return Err("TypeError: Cannot read properties of null/undefined".into());
        }
        let kind = g.get("kind").cloned().unwrap_or_else(js::undefined);
        let i = totals
            .iter()
            .position(|(k, _, _)| same_vz(k, &kind))
            .unwrap_or_else(|| {
                totals.push((kind, 0.0, 0.0));
                totals.len() - 1
            });
        totals[i].1 += num(g, "totalBytes");
        if g.get("cleanable").is_some_and(is_truthy) {
            totals[i].2 += num(g, "eligibleBytes");
        }
    }
    let rank = |k: &Value| {
        k.as_str()
            .and_then(|s| KIND_ORDER.iter().position(|&x| x == s))
            .unwrap_or(4) as f64
    };
    totals.retain(|(_, tb, _)| *tb > 0.0);
    js::sort(&mut totals, |a, b| rank(&a.0) - rank(&b.0));
    let out = totals.into_iter().map(|(k, tb, eb)| json!({ "kind": k, "totalBytes": js::number(tb), "eligibleBytes": js::number(eb) })).collect();
    Ok(Value::Array(out))
}

/// Port of `cacheApps` (`src/agent-cache-model.ts:55`).
pub fn cache_apps(groups: Vec<Value>) -> Result<Value, String> {
    let mut names = vec![json!("Codex"), json!("Claude"), json!("Codex++")];
    let undef = js::undefined();
    for g in &groups {
        if is_nil(g) {
            return Err("TypeError: Cannot read properties of null/undefined".into());
        }
        push_unique(
            &mut names,
            g.get("app").cloned().unwrap_or_else(js::undefined),
        );
    }
    let out = names.into_iter().map(|name| {
        let (mut app_groups, mut total, mut eligible) = (Vec::new(), 0.0_f64, 0.0_f64);
        for g in &groups {
            let app = g.get("app").unwrap_or(&undef);
            if matches!((js::as_number(app), js::as_number(&name)), (Some(x), Some(y)) if x == y) || (js::as_number(app).is_none() && app == &name) {
                total += num(g, "totalBytes");
                eligible += num(g, "eligibleBytes");
                app_groups.push(g.clone());
            }
        }
        json!({ "name": name, "groups": app_groups, "totalBytes": js::number(total), "eligibleBytes": js::number(eligible) })
    }).collect();
    Ok(Value::Array(out))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_summary_totals_and_apps() {
        let g = |id: &str, kind: &str, tb: i64, eb: i64, ef: i64, c: bool| json!({"id": id, "app": "Codex", "kind": kind, "path": "/t", "totalBytes": tb, "eligibleBytes": eb, "eligibleFiles": ef, "skippedFiles": 1, "cleanable": c});
        let gs = vec![
            g("cache", "cache", 100, 60, 2, true),
            g("logs", "logs", 50, 40, 1, true),
            g("temp", "temporary", 80, 80, 1, false),
        ];
        assert_eq!(
            cache_selection(gs.clone(), "recommended".into()).unwrap(),
            json!(["cache"])
        );
        assert_eq!(
            cache_selection(gs.clone(), "all".into()).unwrap(),
            json!(["cache", "logs"])
        );
        assert_eq!(
            cache_cleanup_summary(gs.clone()).unwrap()["bytes"],
            json!(100)
        );
        assert_eq!(
            cache_kind_totals(json!(gs)).unwrap()[0]["totalBytes"],
            json!(100)
        );
        assert_eq!(cache_apps(gs).unwrap()[0]["eligibleBytes"], json!(180));
        assert!(selectable_cache_groups(vec![Value::Null]).is_err());
    }
}
