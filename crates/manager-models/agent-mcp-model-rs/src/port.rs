//! The Rust port of `src/agent-mcp-model.ts`.

#![forbid(unsafe_code)]

use crate::js;
use manager_common_rs::{domain::split_crlf_lines, js_rt as rt};
use serde_json::{Map, Value, json};
use std::cmp::Ordering;

pub const MCP_AGENTS: [&str; 3] = ["codex", "claude", "grok"];

fn by_agent(row: &Value) -> Result<Option<&Map<String, Value>>, String> {
    let v = (!rt::is_nil(row)).then(|| row.get("byAgent")).flatten();
    Ok(v.filter(|x| !rt::is_nil(x))
        .ok_or("TypeError: null")?
        .as_object())
}

fn same_key(a: &Value, b: &Value) -> bool {
    match (js::as_number(a), js::as_number(b)) {
        (Some(x), Some(y)) => (x.is_nan() && y.is_nan()) || x == y,
        _ => a == b,
    }
}

fn uca_key(text: &str) -> (Vec<u32>, Vec<u8>, Vec<u32>) {
    const P: &str = "\t\n\r _-,;:!?.'\"()[]{}@*/\\&#%`^+<=>|~\0$0123456789";
    const F: [&str; 5] = ["äÄa", "öÖo", "üÜu", "ïÏİi", "éÉèÈêÊe"];
    let (mut pr, mut sc, mut tr) = (Vec::new(), Vec::new(), Vec::new());
    for c in text.chars() {
        if c == '\u{FEFF}' || ('\u{0300}'..='\u{036F}').contains(&c) {
            if c != '\u{FEFF}' {
                sc.push(1);
            }
            continue;
        }
        if c == 'ß' {
            pr.extend([78, 78]);
            sc.extend([0, 0]);
            tr.extend([0, 0]);
            continue;
        }
        let (ch, s, up) = match F.iter().find(|s| !c.is_ascii() && s.contains(c)) {
            Some(f) => (f.chars().last().unwrap_or(c), 1, c.is_uppercase()),
            None => (c, 0, c.is_ascii_uppercase()),
        };
        let k = if let Some(i) = P.chars().position(|y| y == ch) {
            1 + i as u32
        } else if ch.is_ascii_alphabetic() {
            60 + (ch as u32 | 32) - 97
        } else if ch.is_alphanumeric() {
            100 + ch as u32
        } else {
            36
        };
        pr.push(k);
        sc.push(s);
        tr.push(u32::from(up));
    }
    (pr, sc, tr)
}

/// Port of `mcpOn` (`src/agent-mcp-model.ts:19`).
pub fn mcp_on(row: Value, agent: String) -> Result<bool, String> {
    let e = by_agent(&row)?.and_then(|m| m.get(&agent));
    Ok(e.filter(|v| rt::is_truthy(v))
        .and_then(|v| v.get("enabled"))
        .is_some_and(rt::is_truthy))
}

/// Port of `mcpPresent` (`src/agent-mcp-model.ts:25`).
pub fn mcp_present(row: Value) -> Result<Vec<String>, String> {
    Ok(by_agent(&row)?.map_or(Vec::new(), |b| {
        MCP_AGENTS
            .iter()
            .filter(|&&a| b.get(a).is_some_and(rt::is_truthy))
            .map(|&a| a.into())
            .collect()
    }))
}

/// Port of `mergeMcpRows` (`src/agent-mcp-model.ts:29`).
pub fn merge_mcp_rows(codex: Vec<Value>, others: Value) -> Result<Vec<Value>, String> {
    if rt::is_nil(&others) {
        return Err("TypeError: null".into());
    }
    let mut srcs: Vec<(&str, Vec<Value>)> = vec![("codex", codex.clone())];
    for a in ["claude", "grok"] {
        if let Some(v) = others.get(a).filter(|v| !rt::is_nil(v)) {
            srcs.push((a, v.as_array().ok_or("TypeError: not iterable")?.clone()));
        }
    }
    let mut rows: Vec<(Value, Value, Map<String, Value>)> = Vec::new();
    for (agent, entries) in srcs {
        for e in entries {
            if rt::is_nil(&e) {
                return Err("TypeError: null".into());
            }
            let id = e.get("id").cloned().unwrap_or_else(js::undefined);
            let body = e.get("tomlBody").cloned().unwrap_or_else(js::undefined);
            if let Some(r) = rows.iter_mut().find(|(k, _, _)| same_key(k, &id)) {
                r.2.insert(agent.into(), e);
            } else {
                rows.push((id, body, [(agent.into(), e)].into_iter().collect()));
            }
        }
    }
    let u = js::undefined();
    let rk = |id: &Value| {
        codex
            .iter()
            .rposition(|e| same_key(e.get("id").unwrap_or(&u), id))
    };
    rows.sort_by(|(l, _, _), (r, _, _)| match (rk(l), rk(r)) {
        (Some(a), Some(b)) => a.cmp(&b),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => uca_key(&rt::js_str(l)).cmp(&uca_key(&rt::js_str(r))),
    });
    Ok(rows
        .into_iter()
        .map(|(id, body, by)| json!({"id": id, "body": body, "byAgent": by}))
        .collect())
}

/// Port of `mcpCount` (`src/agent-mcp-model.ts:54`).
pub fn mcp_count(rows: Vec<Value>, agent: String) -> Result<f64, String> {
    rows.into_iter().try_fold(0.0, |n, r| {
        mcp_on(r, agent.clone()).map(|b| n + f64::from(b))
    })
}

/// Port of `bodyEnabledState` (`src/agent-mcp-model.ts:60`).
pub fn body_enabled_state(body: Option<String>) -> Result<Option<bool>, String> {
    let text = body.unwrap_or_default();
    for line in split_crlf_lines(&text) {
        let rest = line.trim_start_matches(rt::is_js_ws);
        if rest.starts_with('[') {
            break;
        }
        if let Some((lhs, rhs)) = rest.split_once('=') {
            let key = lhs.trim_end_matches(rt::is_js_ws).to_ascii_lowercase();
            let r = rhs.trim_start_matches(rt::is_js_ws);
            let (val, tail) =
                r.split_at(r.find(|c| rt::is_js_ws(c) || c == '#').unwrap_or(r.len()));
            let after = tail.trim_start_matches(rt::is_js_ws);
            if matches!(key.as_str(), "enabled" | "disabled")
                && !tail.contains(['\u{2028}', '\u{2029}'])
                && (after.is_empty() || after.starts_with('#'))
                && let Ok(b) = val.to_ascii_lowercase().parse::<bool>()
            {
                return Ok(Some((key == "enabled") == b));
            }
        }
    }
    Ok(None)
}

/// Port of `editedEnabledState` (`src/agent-mcp-model.ts:71`).
pub fn edited_enabled_state(orig: Option<String>, edit: String) -> Result<Option<bool>, String> {
    match body_enabled_state(Some(edit))? {
        Some(n) if n != body_enabled_state(orig)?.unwrap_or(true) => Ok(Some(n)),
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_counts_and_enabled_state() {
        let e = |id: &str, en: bool| json!({"id": id, "enabled": en, "tomlBody": "c=1\n"});
        let o = json!({"claude": [e("alpha", true), e("beta", true)], "grok": [e("Alpha", true), e("_u", true)]});
        let rs = merge_mcp_rows(vec![e("zeta", true), e("alpha", false)], o).unwrap();
        let ids: Vec<_> = rs.iter().map(|r| r["id"].as_str().unwrap()).collect();
        assert_eq!(ids, ["zeta", "alpha", "_u", "Alpha", "beta"]);
        assert!(!mcp_on(rs[1].clone(), "codex".into()).unwrap());
        assert_eq!(mcp_present(rs[1].clone()).unwrap(), ["codex", "claude"]);
        assert_eq!(mcp_count(rs, "codex".into()).unwrap(), 1.0);
        let (b, ed) = (body_enabled_state, edited_enabled_state);
        assert_eq!(b(Some("disabled=true\n".into())).unwrap(), Some(false));
        assert_eq!(ed(None, "enabled=false\n".into()).unwrap(), Some(false));
    }
}
