//! Shared JS runtime helpers and conversation-canvas model functions.

#![forbid(unsafe_code)]

use serde_json::{Value, json};
use std::collections::{HashMap, HashSet};

pub fn is_js_ws(c: char) -> bool {
    c.is_whitespace() || c == '\u{FEFF}'
}

pub fn undefined() -> Value {
    json!({ "$undefined": true })
}

pub fn is_undefined(v: &Value) -> bool {
    v.as_object()
        .is_some_and(|o| o.len() == 1 && o.get("$undefined") == Some(&Value::Bool(true)))
}

pub fn is_nil(v: &Value) -> bool {
    v.is_null() || is_undefined(v)
}

pub fn as_number(v: &Value) -> Option<f64> {
    match v {
        Value::Number(n) => n.as_f64(),
        Value::Object(o) if o.len() == 1 => match o.get("$num")?.as_str()? {
            "NaN" => Some(f64::NAN),
            "Infinity" => Some(f64::INFINITY),
            "-Infinity" => Some(f64::NEG_INFINITY),
            "-0" => Some(-0.0),
            _ => None,
        },
        _ => None,
    }
}

pub fn js_number(x: f64) -> Value {
    if x.is_nan() {
        json!({ "$num": "NaN" })
    } else if x.is_infinite() {
        json!({ "$num": if x > 0.0 { "Infinity" } else { "-Infinity" } })
    } else if x == 0.0 && x.is_sign_negative() {
        json!({ "$num": "-0" })
    } else if x.fract() == 0.0 && x.abs() < 9_007_199_254_740_992.0 {
        Value::from(x as i64)
    } else {
        Value::from(x)
    }
}

pub fn number_to_string(x: f64) -> String {
    if x.is_nan() {
        return "NaN".into();
    }
    if x.is_infinite() {
        return if x > 0.0 { "Infinity" } else { "-Infinity" }.into();
    }
    if x == 0.0 {
        return "0".into();
    }
    if x.fract() == 0.0 && x.abs() < 9_007_199_254_740_992.0 {
        return (x as i64).to_string();
    }
    let json = serde_json::to_string(&x).unwrap_or_default();
    let (sign, json) = match json.strip_prefix('-') {
        Some(rest) => ("-", rest),
        None => ("", json.as_str()),
    };
    let (mantissa, exponent) = match json.find(['e', 'E']) {
        Some(i) => (&json[..i], json[i + 1..].parse::<i64>().unwrap_or(0)),
        None => (json, 0),
    };
    let (int, frac) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let all = format!("{int}{frac}");
    let lead = all.len() - all.trim_start_matches('0').len();
    let digits = all[lead..].trim_end_matches('0');
    let k = digits.len() as i64;
    let n = int.len() as i64 - lead as i64 + exponent;
    let mut out = String::with_capacity(digits.len() + 8);
    out.push_str(sign);
    if k <= n && n <= 21 {
        out.push_str(digits);
        out.extend(std::iter::repeat_n('0', (n - k) as usize));
    } else if 0 < n && n <= 21 {
        out.push_str(&digits[..n as usize]);
        out.push('.');
        out.push_str(&digits[n as usize..]);
    } else if -6 < n && n <= 0 {
        out.push_str("0.");
        out.extend(std::iter::repeat_n('0', (-n) as usize));
        out.push_str(digits);
    } else {
        let e = n - 1;
        out.push_str(&digits[..1]);
        if k > 1 {
            out.push('.');
            out.push_str(&digits[1..]);
        }
        out.push('e');
        out.push(if e < 0 { '-' } else { '+' });
        out.push_str(&e.abs().to_string());
    }
    out
}

pub fn string_units(v: &Value) -> Option<Vec<u16>> {
    match v {
        Value::String(s) => Some(s.encode_utf16().collect()),
        Value::Object(o) if o.len() == 1 => o
            .get("$utf16")?
            .as_array()?
            .iter()
            .map(|u| u.as_u64().and_then(|u| u16::try_from(u).ok()))
            .collect(),
        _ => None,
    }
}

pub fn utf16_string(units: &[u16]) -> Value {
    match String::from_utf16(units) {
        Ok(s) => Value::String(s),
        Err(_) => json!({ "$utf16": units }),
    }
}

pub fn is_truthy(v: &Value) -> bool {
    !is_nil(v)
        && v != &Value::Bool(false)
        && !string_units(v).is_some_and(|u| u.is_empty())
        && !as_number(v).is_some_and(|n| n == 0.0 || n.is_nan())
}

pub fn to_number(v: &Value) -> f64 {
    if let Some(n) = as_number(v) {
        return n;
    }
    match v {
        Value::Null | Value::Bool(false) => 0.0,
        Value::Bool(true) => 1.0,
        v if is_undefined(v) => f64::NAN,
        Value::String(s) => {
            let t = s.trim_matches(is_js_ws);
            if t.is_empty() {
                0.0
            } else {
                t.parse::<f64>().unwrap_or(f64::NAN)
            }
        }
        Value::Array(a) => match a.len() {
            0 => 0.0,
            1 => to_number(&a[0]),
            _ => f64::NAN,
        },
        _ => f64::NAN,
    }
}

pub fn js_str_u16(v: &Value) -> Vec<u16> {
    if let Some(u) = string_units(v) {
        return u;
    }
    let s = match v {
        Value::Null => "null".into(),
        Value::Bool(b) => b.to_string(),
        Value::Array(a) => a
            .iter()
            .map(|x| {
                if is_nil(x) {
                    String::new()
                } else {
                    String::from_utf16_lossy(&js_str_u16(x))
                }
            })
            .collect::<Vec<_>>()
            .join(","),
        v if is_undefined(v) => "undefined".into(),
        v => as_number(v).map_or_else(|| "[object Object]".into(), number_to_string),
    };
    s.encode_utf16().collect()
}

pub fn js_str(v: &Value) -> String {
    String::from_utf16_lossy(&js_str_u16(v))
}

pub fn js_strict_eq(a: &Value, b: &Value) -> bool {
    if let (Some(x), Some(y)) = (as_number(a), as_number(b)) {
        return x == y;
    }
    if as_number(a).is_some() || as_number(b).is_some() {
        return false;
    }
    if let (Some(x), Some(y)) = (string_units(a), string_units(b)) {
        return x == y;
    }
    match (a, b) {
        (Value::Null, Value::Null) => true,
        (Value::Bool(x), Value::Bool(y)) => x == y,
        _ if is_undefined(a) && is_undefined(b) => true,
        _ => false,
    }
}

pub fn eq_key(v: &Value) -> String {
    match as_number(v) {
        Some(n) if n.is_nan() => "N:NaN".into(),
        Some(n) => format!("N:{}", if n == 0.0 { 0.0 } else { n }.to_bits()),
        None => v.to_string(),
    }
}

pub fn set_has(collapsed: &Value, id: &Value) -> Result<bool, String> {
    if is_undefined(collapsed) {
        return Ok(false);
    }
    if collapsed.is_null() {
        return Err("TypeError: Cannot read properties of null (reading 'has')".into());
    }
    let Some(arr) = collapsed
        .as_object()
        .filter(|o| o.len() == 1)
        .and_then(|o| o.get("$set"))
        .and_then(Value::as_array)
    else {
        return Err("TypeError: collapsed.has is not a function".into());
    };
    let k = eq_key(id);
    Ok(arr.iter().any(|x| eq_key(x) == k))
}

pub fn push_esc_u16(out: &mut Vec<u16>, units: &[u16]) {
    for &c in units {
        let rep = match c {
            38 => "&amp;",
            60 => "&lt;",
            62 => "&gt;",
            34 => "&quot;",
            39 => "&#39;",
            _ => {
                out.push(c);
                continue;
            }
        };
        out.extend(rep.encode_utf16());
    }
}

pub fn esc_html(v: &Value) -> String {
    let mut u = Vec::new();
    let src = if is_nil(v) { Vec::new() } else { js_str_u16(v) };
    push_esc_u16(&mut u, &src);
    String::from_utf16_lossy(&u)
}

pub fn esc_html_u16(v: &Value) -> Vec<u16> {
    let mut u = Vec::new();
    let src = if is_nil(v) { Vec::new() } else { js_str_u16(v) };
    push_esc_u16(&mut u, &src);
    u
}

pub fn strip_pair_tags(s: &str, open: &str, close: &str) -> String {
    let (mut out, mut r) = (String::with_capacity(s.len()), s);
    while let Some(i) = r.find(open) {
        out.push_str(&r[..i]);
        let Some(j) = r[i + open.len()..].find(close) else {
            return out + &r[i..];
        };
        r = &r[i + open.len() + j + close.len()..];
    }
    out + r
}

pub fn parse_message(record: Value, fallback_id: Value) -> Result<Value, String> {
    if is_nil(&record) {
        return Err("TypeError: Cannot read properties of null/undefined".into());
    }
    let u = undefined();
    let p = record.get("payload").unwrap_or(&u);
    let role = p.get("role").and_then(Value::as_str).unwrap_or("");
    if record.get("type") != Some(&json!("response_item"))
        || p.get("type") != Some(&json!("message"))
        || !matches!(role, "user" | "assistant")
    {
        return Ok(Value::Null);
    }
    let items = match p.get("content") {
        None | Some(Value::Null) => &[][..],
        Some(v) if is_undefined(v) || v == &json!(false) => &[][..],
        Some(Value::Array(a)) => a.as_slice(),
        _ => return Err("TypeError: content.map is not a function".into()),
    };
    let mut parts = Vec::with_capacity(items.len());
    for c in items {
        if is_nil(c) {
            return Err("TypeError: Cannot read properties of null/undefined".into());
        }
        let t = c.get("text").unwrap_or(&u);
        parts.push(if is_truthy(t) {
            js_str(t)
        } else {
            String::new()
        });
    }
    let mut text = parts.join("\n").trim_matches(is_js_ws).to_string();
    if text.is_empty()
        || text.starts_with("<recommended_plugins")
        || text.starts_with("<environment_context")
        || text.starts_with("# AGENTS.md instructions")
    {
        return Ok(Value::Null);
    }
    if text.starts_with("<send_user_message_question_reply>") {
        let raw = text
            .replace("<send_user_message_question_reply>", "")
            .replace("</send_user_message_question_reply>", "");
        let Ok(Value::Array(arr)) = serde_json::from_str::<Value>(raw.trim_matches(is_js_ws))
        else {
            return Ok(Value::Null);
        };
        let mut ans = Vec::with_capacity(arr.len());
        for a in arr {
            if is_nil(&a) {
                return Ok(Value::Null);
            }
            ans.push(
                a.get("answer")
                    .filter(|v| !is_nil(v))
                    .map(js_str)
                    .unwrap_or_default(),
            );
        }
        text = ans.join("\n");
    }
    text = strip_pair_tags(&text, "<oai-mem-citation>", "</oai-mem-citation>")
        .trim_matches(is_js_ws)
        .to_string();
    let pid = p.get("id").cloned().unwrap_or_else(undefined);
    let id = [pid, fallback_id]
        .into_iter()
        .find(is_truthy)
        .ok_or("Message without an ID requires a stable fallback ID")?;
    let phase = p
        .get("phase")
        .filter(|v| is_truthy(v))
        .cloned()
        .unwrap_or_else(|| json!(""));
    let turn_id = p
        .get("internal_chat_message_metadata_passthrough")
        .and_then(|m| m.get("turn_id"))
        .filter(|v| is_truthy(v))
        .cloned()
        .unwrap_or(Value::Null);
    Ok(json!({
        "id": id,
        "text": text,
        "role": role,
        "phase": phase,
        "timestamp": record.get("timestamp").cloned().unwrap_or_else(undefined),
        "ordinal": record.get("ordinal").cloned().unwrap_or_else(undefined),
        "turnId": turn_id,
    }))
}

pub fn excerpt(text: Value, length: Option<f64>) -> Result<Value, String> {
    let Some(u) = string_units(&text) else {
        return Err("TypeError: text.replace is not a function".into());
    };
    let cleaned =
        strip_pair_tags(&String::from_utf16_lossy(&u), "```", "```").replace(['*', '#', '`'], "");
    let joined = cleaned
        .split(is_js_ws)
        .filter(|w| !w.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    let cu: Vec<u16> = joined.encode_utf16().collect();
    let n = length.unwrap_or(46.0);
    let end = if n.is_nan() || (n <= 0.0 && n.is_infinite()) {
        0
    } else if n.is_infinite() {
        cu.len()
    } else if n < 0.0 {
        (cu.len() as isize + n.trunc() as isize).max(0) as usize
    } else {
        (n.trunc() as usize).min(cu.len())
    };
    Ok(utf16_string(&cu[..end.min(cu.len())]))
}

pub fn build_graph(messages: Value, annotations: Value) -> Result<Value, String> {
    let Value::Array(msgs) = &messages else {
        return Err("TypeError: messages.map is not a function".into());
    };
    if annotations.is_null() {
        return Err("TypeError: Cannot read properties of null (reading 'nodes')".into());
    }
    let ann = if is_undefined(&annotations) {
        json!({ "nodes": [] })
    } else {
        annotations
    };
    let u = undefined();
    let mut ids = HashSet::new();
    for m in msgs {
        if is_nil(m) {
            return Err("TypeError: null msg".into());
        }
        ids.insert(eq_key(m.get("id").unwrap_or(&u)));
    }
    let (mut nodes, mut cov) = (Vec::new(), HashSet::new());
    for n in ann
        .get("nodes")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        let Some(src) = n
            .get("sources")
            .and_then(Value::as_array)
            .filter(|s| !s.is_empty() && s.iter().all(|id| ids.contains(&eq_key(id))))
        else {
            continue;
        };
        for id in src {
            cov.insert(eq_key(id));
        }
        let mut o = n.as_object().cloned().unwrap_or_default();
        o.insert("summaryKind".into(), json!("已整理"));
        nodes.push(Value::Object(o));
    }
    for id in ann
        .get("incompleteSources")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        cov.remove(&eq_key(id));
    }
    let mut prev = nodes
        .iter()
        .rfind(|n| n.get("lane") == Some(&json!("main")))
        .and_then(|n| n.get("id").filter(|v| is_truthy(v)).cloned())
        .unwrap_or(Value::Null);
    let v2 = ann.get("schemaVersion").and_then(as_number) == Some(2.0);
    for m in msgs {
        let mid = m.get("id").unwrap_or(&u);
        if cov.contains(&eq_key(mid))
            || (m.get("role") == Some(&json!("assistant"))
                && m.get("phase") != Some(&json!("final_answer")))
        {
            continue;
        }
        let mt = m.get("text").cloned().unwrap_or_else(undefined);
        let aid = json!(format!("auto-{}", js_str(mid)));
        nodes.push(json!({
            "id": aid,
            "parent": if v2 { Value::Null } else { prev },
            "lane": if v2 { "pending" } else { "main" },
            "title": excerpt(mt.clone(), Some(34.0))?,
            "summary": excerpt(mt.clone(), Some(160.0))?,
            "description": mt,
            "status": "待整理",
            "summaryKind": "原文摘录",
            "sources": [mid]
        }));
        prev = aid;
    }
    let nids: HashSet<String> = nodes
        .iter()
        .map(|n| eq_key(n.get("id").unwrap_or(&u)))
        .collect();
    let edges: Vec<Value> = nodes
        .iter()
        .filter_map(|n| {
            let p = n
                .get("parent")
                .filter(|p| is_truthy(p) && nids.contains(&eq_key(p)))?;
            let kind = if n.get("lane") == Some(&json!("branch")) {
                "branch"
            } else {
                "main"
            };
            Some(json!({ "from": p, "to": n["id"], "kind": kind }))
        })
        .collect();
    let sv = ann
        .get("schemaVersion")
        .filter(|v| is_truthy(v))
        .cloned()
        .unwrap_or_else(|| json!(1));
    let cur = ann
        .get("currentNodeId")
        .filter(|v| nids.contains(&eq_key(v)))
        .cloned()
        .unwrap_or(Value::Null);
    Ok(json!({ "schemaVersion": sv, "currentNodeId": cur, "nodes": nodes, "edges": edges }))
}

pub fn visible_tree_rows(
    nodes: Value,
    current_node_id: Value,
    collapsed: Value,
) -> Result<Value, String> {
    let Value::Array(all) = &nodes else {
        return Err("TypeError: nodes.filter is not a function".into());
    };
    let (mut pending, mut ready) = (Vec::new(), Vec::new());
    for n in all {
        if is_nil(n) {
            return Err("TypeError: null node".into());
        }
        if n.get("summaryKind") == Some(&json!("原文摘录")) {
            pending.push(n.clone());
        } else {
            ready.push(n.clone());
        }
    }
    let u = undefined();
    let by_id: HashMap<String, Value> = ready
        .iter()
        .map(|n| (eq_key(n.get("id").unwrap_or(&u)), n.clone()))
        .collect();
    let (mut ch_map, mut roots): (HashMap<String, Vec<Value>>, Vec<Value>) =
        (HashMap::new(), Vec::new());
    for n in ready {
        let p = n.get("parent").unwrap_or(&u);
        let pk = eq_key(p);
        if is_truthy(p) && by_id.contains_key(&pk) {
            ch_map.entry(pk).or_default().push(n);
        } else {
            roots.push(n);
        }
    }
    let (mut active, mut ck) = (HashSet::new(), eq_key(&current_node_id));
    while let Some(n) = by_id.get(&ck) {
        if !active.insert(ck) {
            break;
        }
        ck = eq_key(n.get("parent").unwrap_or(&u));
    }
    let (mut rows, mut seen) = (Vec::new(), HashSet::new());
    let mut stack: Vec<(Value, usize)> = roots.into_iter().rev().map(|n| (n, 0)).collect();
    while let Some((n, d)) = stack.pop() {
        let id = n.get("id").unwrap_or(&u);
        let k = eq_key(id);
        if !seen.insert(k.clone()) {
            continue;
        }
        let ch = ch_map.get(&k).map(Vec::as_slice).unwrap_or(&[]);
        rows.push(
            json!({ "node": n, "depth": d, "childCount": ch.len(), "active": active.contains(&k) }),
        );
        if !set_has(&collapsed, id)? {
            stack.extend(ch.iter().rev().cloned().map(|c| (c, d + 1)));
        }
    }
    Ok(json!({ "rows": rows, "pending": pending }))
}

fn vp_pg(v: Option<&Value>, total: usize) -> usize {
    match v.and_then(as_number) {
        Some(n) if n.is_finite() && n > 0.0 => (n.floor() as usize).min(total - 1),
        _ => 0,
    }
}

pub fn transcript_page(messages: Value, state: Value) -> Result<Value, String> {
    let Value::Array(msgs) = &messages else {
        return Err("TypeError: messages.find is not a function".into());
    };
    if state.is_null() {
        return Err("TypeError: Cannot read properties of null (reading 'focusId')".into());
    }
    let st = if is_undefined(&state) {
        json!({})
    } else {
        state
    };
    let u0 = undefined();
    let fid = st.get("focusId").unwrap_or(&u0);
    let mut focus = None;
    for m in msgs {
        if is_nil(m) {
            return Err("TypeError: null msg".into());
        }
        if js_strict_eq(m.get("id").unwrap_or(&u0), fid) {
            focus = Some(m.clone());
            break;
        }
    }
    let focused = focus.is_some();
    let (rows, page, total, info) = if let Some(f) = focus {
        let u = f
            .get("text")
            .and_then(string_units)
            .ok_or("TypeError: text")?;
        let total = u.len().div_ceil(12000).max(1);
        let page = vp_pg(st.get("part"), total);
        let bnd = |o: usize| {
            if o > 0
                && o < u.len()
                && (0xDC00..=0xDFFF).contains(&u[o])
                && (0xD800..=0xDBFF).contains(&u[o - 1])
            {
                o - 1
            } else {
                o
            }
        };
        let (s, e) = (
            bnd((page * 12000).min(u.len())),
            bnd(((page + 1) * 12000).min(u.len())),
        );
        let mut r = f.as_object().cloned().unwrap_or_default();
        r.insert("text".into(), utf16_string(&u[s..e]));
        (
            vec![Value::Object(r)],
            page,
            total,
            format!("消息全文 · 第 {}/{total} 段", page + 1),
        )
    } else {
        let total = msgs.len().div_ceil(20).max(1);
        let page = vp_pg(st.get("page"), total);
        let (s, e) = (
            (page * 20).min(msgs.len()),
            ((page + 1) * 20).min(msgs.len()),
        );
        (
            msgs[s..e].to_vec(),
            page,
            total,
            format!(
                "消息 {}–{e} / {}",
                usize::from(!msgs.is_empty()) * (s + 1),
                msgs.len()
            ),
        )
    };
    let mut h = Vec::new();
    for m in &rows {
        let tu = m
            .get("text")
            .and_then(string_units)
            .ok_or("TypeError: text")?;
        let iu = m
            .get("id")
            .filter(|v| !is_nil(v))
            .map(js_str_u16)
            .unwrap_or_default();
        let cls = ["", "highlight"][usize::from(focused)];
        let who = ["Codex", "你"][usize::from(m.get("role") == Some(&json!("user")))];
        let ph = ["", " · 进展"][usize::from(m.get("phase") == Some(&json!("commentary")))];
        h.extend(format!(r#"<article class="message {cls}" id="source-"#).encode_utf16());
        push_esc_u16(&mut h, &iu);
        h.extend(format!(r#""><small>{who}{ph}</small><pre>"#).encode_utf16());
        let preview_end = if focused {
            tu.len()
        } else {
            let mut end = tu.len().min(2000);
            if end < tu.len() && end > 0 && (0xD800..=0xDBFF).contains(&tu[end - 1]) {
                end -= 1;
            }
            end
        };
        push_esc_u16(&mut h, &tu[..preview_end]);
        h.extend("</pre>".encode_utf16());
        if !focused && tu.len() > 2000 {
            h.extend(r#"<button data-source-full=""#.encode_utf16());
            push_esc_u16(&mut h, &iu);
            h.extend(format!(r#"">分段查看完整消息（{} 字符）</button>"#, tu.len()).encode_utf16());
        }
        h.extend("</article>".encode_utf16());
    }
    Ok(
        json!({ "html": utf16_string(&h), "info": info, "page": page, "total": total, "focused": focused }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transcript_page_does_not_split_surrogate_pair_at_2000_units() {
        // 1999 BMP units + '🚀' (2 UTF-16 units at indices 1999..2001) => total 2001 UTF-16 units
        let text = format!("{}🚀尾", "a".repeat(1999));
        assert_eq!(text.encode_utf16().count(), 2002);
        let res = transcript_page(
            json!([{ "id": "m1", "role": "user", "text": text }]),
            json!({}),
        )
        .expect("transcript_page");
        let html = res["html"]
            .as_str()
            .expect("html must remain valid UTF-8/UTF-16 string without lone surrogate");
        assert!(html.contains(&format!("<pre>{}</pre>", "a".repeat(1999))));
        assert!(html.contains("分段查看完整消息（2002 字符）"));
    }
}
