//! The Rust port of `src/context-config-text.ts`.

#![forbid(unsafe_code)]

use crate::utf16;
use manager_common_rs::{
    as_number, is_js_ws, is_nil, is_truthy, is_undefined, js_str, js_trim, split_crlf_lines,
    string_units, undefined, utf16_string,
};
use serde_json::{Value, json};
use std::collections::HashSet;

const CONTEXT_KIND_TABLES: [(&str, &str); 3] = [
    ("mcp", "mcp_servers"),
    ("skill", "skills"),
    ("plugin", "plugins"),
];

fn ensure_trailing_newline(value: &str) -> String {
    if js_trim(value).is_empty() {
        String::new()
    } else {
        format!("{value}\n")
    }
}

fn toml_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn toml_key(key: &str) -> String {
    if !key.is_empty()
        && key
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
    {
        key.to_string()
    } else {
        format!("\"{}\"", toml_string(key))
    }
}

fn header_inner(line: &str) -> Option<&str> {
    let inner = js_trim(line).strip_prefix('[')?.strip_suffix(']')?;
    if inner.is_empty() || inner.contains(']') {
        None
    } else {
        Some(inner)
    }
}

fn is_enabled_eq(trimmed_start: &str) -> bool {
    trimmed_start
        .strip_prefix("enabled")
        .is_some_and(|r| r.trim_start_matches(is_js_ws).starts_with('='))
}

fn is_enabled_false_line(line: &str) -> bool {
    let s = line.trim_start_matches(is_js_ws);
    if !s
        .get(..7)
        .is_some_and(|h| h.eq_ignore_ascii_case("enabled"))
    {
        return false;
    }
    let Some(after_eq) = s[7..].trim_start_matches(is_js_ws).strip_prefix('=') else {
        return false;
    };
    let val = after_eq.trim_start_matches(is_js_ws);
    if !val
        .get(..5)
        .is_some_and(|f| f.eq_ignore_ascii_case("false"))
    {
        return false;
    }
    let tail = val[5..].trim_start_matches(is_js_ws);
    tail.is_empty() || (tail.starts_with('#') && !tail.contains(['\r', '\u{2028}', '\u{2029}']))
}

fn relative_subtable_to_absolute(line: &str, table_name: &str, id: &str) -> String {
    let Some(inner) = header_inner(line) else {
        return line.to_string();
    };
    let Ok(Some(path)) = parse_toml_dotted_path(js_trim(inner).to_string()) else {
        return line.to_string();
    };
    if path.is_empty() {
        return line.to_string();
    }
    let joined = std::iter::once(table_name)
        .chain(std::iter::once(id))
        .chain(path.iter().map(String::as_str))
        .map(toml_key)
        .collect::<Vec<_>>()
        .join(".");
    format!("[{joined}]")
}

fn same_js_key(a: &Value, b: &Value) -> bool {
    if is_undefined(a) || is_undefined(b) {
        return is_undefined(a) && is_undefined(b);
    }
    if a.is_null() || b.is_null() {
        return a.is_null() && b.is_null();
    }
    if let (Some(x), Some(y)) = (as_number(a), as_number(b)) {
        return x == y || (x.is_nan() && y.is_nan());
    }
    if let (Some(x), Some(y)) = (string_units(a), string_units(b)) {
        return x == y;
    }
    a == b
}

/// Port of `unquoteTomlKey` (`src/context-config-text.ts:41`).
pub fn unquote_toml_key(key: String) -> Result<Value, String> {
    let out = if key.len() >= 2
        && ((key.starts_with('"') && key.ends_with('"'))
            || (key.starts_with('\'') && key.ends_with('\'')))
    {
        &key[1..key.len() - 1]
    } else {
        &key
    };
    Ok(Value::String(out.to_string()))
}

/// Port of `tomlTablePathFromLine` (`src/context-config-text.ts:48`).
pub fn toml_table_path_from_line(line: String) -> Result<Option<Vec<String>>, String> {
    let Some(inner) = header_inner(&line) else {
        return Ok(None);
    };
    parse_toml_dotted_path(js_trim(inner).to_string())
}

/// Port of `parseTomlDottedPath` (`src/context-config-text.ts:54`).
pub fn parse_toml_dotted_path(path: String) -> Result<Option<Vec<String>>, String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    let mut escaping = false;

    for ch in path.chars() {
        if let Some(q) = quote {
            if q == '"' && escaping {
                current.push(ch);
                escaping = false;
            } else if q == '"' && ch == '\\' {
                escaping = true;
            } else if ch == q {
                quote = None;
            } else {
                current.push(ch);
            }
            continue;
        }
        if ch == '"' || ch == '\'' {
            quote = Some(ch);
            continue;
        }
        if ch == '.' {
            let trimmed = js_trim(&current);
            if trimmed.is_empty() {
                return Ok(None);
            }
            parts.push(trimmed.to_string());
            current.clear();
            continue;
        }
        current.push(ch);
    }

    let trimmed = js_trim(&current);
    if quote.is_some() || escaping || trimmed.is_empty() {
        return Ok(None);
    }
    parts.push(trimmed.to_string());
    Ok(Some(parts))
}

/// Port of `contextEntrySummary` (`src/context-config-text.ts:93`).
pub fn context_entry_summary(toml_body: String) -> Result<Value, String> {
    let found = split_crlf_lines(&toml_body)
        .into_iter()
        .map(js_trim)
        .find(|line| !line.is_empty() && !line.starts_with('#') && !is_enabled_eq(line));
    let summary = found
        .map(|line| utf16::slice(&utf16::units(line), 0, 96))
        .unwrap_or_default();
    Ok(Value::String(summary))
}

/// Port of `contextEntryEnabled` (`src/context-config-text.ts:101`).
pub fn context_entry_enabled(toml_body: String) -> Result<Value, String> {
    Ok(Value::Bool(
        !split_crlf_lines(&toml_body)
            .into_iter()
            .any(is_enabled_false_line),
    ))
}

/// Port of `setContextEntryEnabled` (`src/context-config-text.ts:105`).
pub fn set_context_entry_enabled(toml_body: String, enabled: bool) -> Result<Value, String> {
    let next_value = format!("enabled = {}", if enabled { "true" } else { "false" });
    let mut replaced = false;
    let mut next: Vec<String> = split_crlf_lines(toml_body.trim_end_matches(is_js_ws))
        .into_iter()
        .map(|line| {
            if is_enabled_eq(line.trim_start_matches(is_js_ws)) {
                replaced = true;
                next_value.clone()
            } else {
                line.to_string()
            }
        })
        .collect();
    if !replaced {
        next.insert(0, next_value);
    }
    let joined = next.join("\n");
    Ok(Value::String(ensure_trailing_newline(
        joined.trim_end_matches(is_js_ws),
    )))
}

fn context_header_parts(line: &str) -> Option<(&'static str, String)> {
    let path = toml_table_path_from_line(line.to_string()).ok()??;
    if path.len() < 2 {
        return None;
    }
    let (kind, _) = CONTEXT_KIND_TABLES
        .iter()
        .find(|(_, tbl)| *tbl == path[0])?;
    Some((*kind, path[1].clone()))
}

/// Port of `contextHeaderFromLine` (`src/context-config-text.ts:121`).
pub fn context_header_from_line(line: String) -> Result<Option<Value>, String> {
    Ok(context_header_parts(&line).map(|(kind, id)| json!({ "kind": kind, "id": id })))
}

/// Port of `parseContextEntries` (`src/context-config-text.ts:132`).
pub fn parse_context_entries(
    common_config: String,
    kind: String,
    table_name: String,
) -> Result<Vec<Value>, String> {
    struct Draft {
        id: String,
        root: Vec<String>,
        subs: Vec<(String, Vec<String>)>,
    }
    let mut drafts: Vec<Draft> = Vec::new();
    let mut current_idx: Option<usize> = None;
    let mut current_sub: Option<usize> = None;

    for line in split_crlf_lines(&common_config) {
        let path = toml_table_path_from_line(line.to_string())?;
        if let Some(ref p) = path
            && p.len() >= 2
            && p[0] == table_name
        {
            let id = &p[1];
            let idx = match drafts.iter().position(|d| d.id == *id) {
                Some(i) => i,
                None => {
                    drafts.push(Draft {
                        id: id.clone(),
                        root: Vec::new(),
                        subs: Vec::new(),
                    });
                    drafts.len() - 1
                }
            };
            current_idx = Some(idx);
            if p.len() == 2 {
                current_sub = None;
            } else {
                let hdr = format!(
                    "[{}]",
                    p[2..]
                        .iter()
                        .map(|s| toml_key(s))
                        .collect::<Vec<_>>()
                        .join(".")
                );
                drafts[idx].subs.push((hdr, Vec::new()));
                current_sub = Some(drafts[idx].subs.len() - 1);
            }
            continue;
        }
        if current_idx.is_some() && header_inner(line).is_some() {
            current_idx = None;
            current_sub = None;
            continue;
        }
        if let Some(idx) = current_idx {
            match current_sub {
                None => drafts[idx].root.push(line.to_string()),
                Some(sub_idx) => drafts[idx].subs[sub_idx].1.push(line.to_string()),
            }
        }
    }

    Ok(drafts
        .into_iter()
        .map(|draft| {
            let mut body = draft.root;
            for (hdr, lines) in draft.subs {
                body.push(hdr);
                body.extend(lines);
            }
            let joined = body.join("\n");
            let toml_body = ensure_trailing_newline(joined.trim_end_matches(is_js_ws));
            let summary = context_entry_summary(toml_body.clone())
                .ok()
                .and_then(|v| v.as_str().map(str::to_string))
                .unwrap_or_default();
            let enabled = context_entry_enabled(toml_body.clone())
                .ok()
                .and_then(|v| v.as_bool())
                .unwrap_or(true);
            json!({
                "id": draft.id,
                "kind": kind,
                "title": draft.id,
                "summary": summary,
                "tomlBody": toml_body,
                "enabled": enabled,
            })
        })
        .collect())
}

/// Port of `dedupeContextEntryList` (`src/context-config-text.ts:186`).
pub fn dedupe_context_entry_list(entries: Value) -> Result<Vec<Value>, String> {
    let arr = match entries {
        Value::Array(arr) => arr,
        v if string_units(&v).is_some() => {
            if let Value::String(s) = v {
                return Ok(s
                    .chars()
                    .last()
                    .map(|c| vec![Value::String(c.to_string())])
                    .unwrap_or_default());
            }
            let units = string_units(&v).unwrap_or_default();
            return Ok(units
                .last()
                .map(|&u| vec![utf16_string(&[u])])
                .unwrap_or_default());
        }
        _ => return Err("TypeError: entries is not iterable".into()),
    };
    let mut by_id: Vec<(Value, Value)> = Vec::new();
    let undef = undefined();
    for entry in arr {
        if is_nil(&entry) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'id')".into(),
            );
        }
        let id = entry.get("id").unwrap_or(&undef).clone();
        if let Some((_, slot)) = by_id.iter_mut().find(|(k, _)| same_js_key(k, &id)) {
            *slot = entry;
        } else {
            by_id.push((id, entry));
        }
    }
    Ok(by_id.into_iter().map(|(_, v)| v).collect())
}

/// Port of `contextEntriesByKind` (`src/context-config-text.ts:194`).
pub fn context_entries_by_kind(entries: Value, kind: String) -> Result<Vec<Value>, String> {
    if is_nil(&entries) {
        return Err("TypeError: Cannot read properties of null/undefined".into());
    }
    let field = match kind.as_str() {
        "mcp" => "mcpServers",
        "skill" => "skills",
        _ => "plugins",
    };
    dedupe_context_entry_list(entries.get(field).cloned().unwrap_or_else(undefined))
}

/// Port of `contextEntryToTomlSection` (`src/context-config-text.ts:200`).
pub fn context_entry_to_toml_section(table_name: String, entry: Value) -> Result<String, String> {
    if is_nil(&entry) {
        return Err("TypeError: Cannot read properties of null/undefined (reading 'id')".into());
    }
    let undef = undefined();
    let id = js_str(entry.get("id").unwrap_or(&undef));
    let toml_body = entry
        .get("tomlBody")
        .and_then(Value::as_str)
        .ok_or("TypeError: Cannot read properties of undefined (reading 'trimEnd')")?;
    let parent_header = format!("[{table_name}.{}]", toml_key(&id));
    let body = split_crlf_lines(toml_body.trim_end_matches(is_js_ws))
        .into_iter()
        .map(|line| relative_subtable_to_absolute(line, &table_name, &id))
        .collect::<Vec<_>>()
        .join("\n");
    Ok(format!("{parent_header}\n{body}"))
}

fn render_context_toml(entries: Value, only_enabled: bool) -> Result<String, String> {
    if is_nil(&entries) {
        return Err("TypeError: Cannot read properties of null/undefined".into());
    }
    let mut sections = Vec::new();
    for (kind, table_name) in CONTEXT_KIND_TABLES {
        let list = context_entries_by_kind(entries.clone(), kind.to_string())?;
        for entry in list {
            if only_enabled && !entry.get("enabled").is_some_and(is_truthy) {
                continue;
            }
            sections.push(context_entry_to_toml_section(
                table_name.to_string(),
                entry,
            )?);
        }
    }
    Ok(ensure_trailing_newline(&sections.join("\n\n")))
}

/// Port of `allContextConfigToml` (`src/context-config-text.ts:219`).
pub fn all_context_config_toml(entries: Value) -> Result<String, String> {
    render_context_toml(entries, false)
}

/// Port of `selectedContextConfigToml` (`src/context-config-text.ts:229`).
pub fn selected_context_config_toml(entries: Value) -> Result<String, String> {
    render_context_toml(entries, true)
}

/// Port of `contextEntriesFromConfig` (`src/context-config-text.ts:240`).
pub fn context_entries_from_config(config_contents: String) -> Result<Value, String> {
    Ok(json!({
        "mcpServers": parse_context_entries(config_contents.clone(), "mcp".into(), "mcp_servers".into())?,
        "skills": parse_context_entries(config_contents.clone(), "skill".into(), "skills".into())?,
        "plugins": parse_context_entries(config_contents, "plugin".into(), "plugins".into())?,
    }))
}

/// Port of `splitContextConfigText` (`src/context-config-text.ts:248`).
pub fn split_context_config_text(config_contents: String) -> Result<Value, String> {
    let entries = context_entries_from_config(config_contents.clone())?;
    let common = strip_context_entries_from_config(config_contents, entries.clone())?;
    let context = all_context_config_toml(entries)?;
    Ok(json!({ "common": common, "context": context }))
}

/// Port of `stripContextEntriesFromConfig` (`src/context-config-text.ts:258`).
pub fn strip_context_entries_from_config(
    config_contents: String,
    entries: Value,
) -> Result<String, String> {
    if is_nil(&entries) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'mcpServers')".into(),
        );
    }
    let collect_ids = |field: &str| -> Result<HashSet<String>, String> {
        let arr = entries
            .get(field)
            .and_then(Value::as_array)
            .ok_or_else(|| {
                "TypeError: Cannot read properties of undefined (reading 'map')".to_string()
            })?;
        let mut set = HashSet::with_capacity(arr.len());
        let undef = undefined();
        for item in arr {
            if is_nil(item) {
                return Err(
                    "TypeError: Cannot read properties of null/undefined (reading 'id')".into(),
                );
            }
            set.insert(js_str(item.get("id").unwrap_or(&undef)));
        }
        Ok(set)
    };
    let mcp_ids = collect_ids("mcpServers")?;
    let skill_ids = collect_ids("skills")?;
    let plugin_ids = collect_ids("plugins")?;

    let mut kept = Vec::new();
    let mut skipping = false;
    for line in split_crlf_lines(&config_contents) {
        if let Some((kind, id)) = context_header_parts(line) {
            skipping = match kind {
                "mcp" => mcp_ids.contains(&id),
                "skill" => skill_ids.contains(&id),
                _ => plugin_ids.contains(&id),
            };
        } else if header_inner(line).is_some() {
            skipping = false;
        }
        if !skipping {
            kept.push(line);
        }
    }
    let joined = kept.join("\n");
    Ok(ensure_trailing_newline(joined.trim_end_matches(is_js_ws)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_non_adjacent_subtables_and_strips_known_ids() {
        let cfg = "model = \"gpt-5\"\n\n[mcp_servers.premiere.env]\nA = \"1\"\n\n[mcp_servers.vivado]\ncommand = \"python\"\n\n[mcp_servers.vivado.env]\nV = \"2\"\n";
        let split = split_context_config_text(cfg.into()).unwrap();
        assert_eq!(split["common"], "model = \"gpt-5\"\n");
        let entries =
            parse_context_entries(cfg.into(), "mcp".into(), "mcp_servers".into()).unwrap();
        assert_eq!(entries.len(), 2);
        let vivado = entries.iter().find(|e| e["id"] == "vivado").unwrap();
        let body = vivado["tomlBody"].as_str().unwrap();
        assert!(body.find("command").unwrap() < body.find("[env]").unwrap());
    }

    #[test]
    fn truncates_summary_in_utf16_units() {
        let line = format!("{}{}", "😀", "a".repeat(95));
        let sum = context_entry_summary(line).unwrap();
        assert_eq!(sum.as_str().unwrap().encode_utf16().count(), 96);
    }
}
