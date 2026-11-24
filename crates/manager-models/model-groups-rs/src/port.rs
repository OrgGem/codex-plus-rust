//! The Rust port of `src/model-groups.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::js_trim;
use serde_json::{Value, json};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub const OTHER_MODEL_GROUP: &str = "Other";
const FAMILY_MAP: &str = "gpt=GPT,chatgpt=GPT,codex=Codex,claude=Claude,gemini=Gemini,deepseek=DeepSeek,qwen=Qwen,qwq=Qwen,glm=GLM,kimi=Kimi,grok=Grok,llama=Llama,mistral=Mistral";
const ORDER: &str =
    "GPT,OpenAI o-series,Codex,Claude,Gemini,DeepSeek,Qwen,GLM,Kimi,Grok,Llama,Mistral,Other";
const ASCII_EN: &str = "\t\n\r _-,;:!?.'\"()[]{}@*/\\&#%`^+<=>|~$0123456789aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ";

fn family_of(model: &str) -> &'static str {
    let norm: String = js_trim(model)
        .to_lowercase()
        .chars()
        .map(|c| {
            if matches!(c, '.' | '_' | '/' | ':') {
                '-'
            } else {
                c
            }
        })
        .collect();
    let tail_ok = |s: &str| {
        let rest = s.trim_start_matches(|c: char| c.is_ascii_digit());
        rest.is_empty() || rest.starts_with('-')
    };
    let b = norm.as_bytes();
    if b.len() >= 2 && b[0] == b'o' && (b'1'..=b'9').contains(&b[1]) && tail_ok(&norm[2..]) {
        return "OpenAI o-series";
    }
    FAMILY_MAP
        .split(',')
        .filter_map(|entry| entry.split_once('='))
        .find(|(p, _)| norm.strip_prefix(p).is_some_and(tail_ok))
        .map_or(OTHER_MODEL_GROUP, |(_, label)| label)
}

type Ce = (u32, u16, u8);

fn push_char_ces(ch: char, out: &mut Vec<Ce>) {
    if matches!(ch, '\u{FEFF}' | '\u{AD}' | '\u{200B}'..='\u{200F}' | '\u{202A}'..='\u{202E}' | '\u{2060}'..='\u{206F}' | '\u{FE00}'..='\u{FE0F}')
    {
        return;
    }
    if ch.is_ascii()
        && let Some(pos) = ASCII_EN.find(ch)
    {
        return out.push(if ch.is_ascii_alphabetic() {
            (
                0x0500_0000 + u32::from(ch.to_ascii_lowercase() as u8 - b'a') * 10,
                5,
                if ch.is_ascii_uppercase() { 8 } else { 2 },
            )
        } else if ch.is_ascii_digit() {
            (0x0400_0000 + u32::from(ch as u8 - b'0'), 5, 2)
        } else {
            (0x0100_0000 + pos as u32, 5, 2)
        });
    }
    if ('\u{300}'..='\u{36F}').contains(&ch) {
        return out.push((0, 30 + (ch as u32 - 0x300) as u16, 2));
    }
    if ch == '\u{DF}' {
        let p = 0x0500_0000 + u32::from(b's' - b'a') * 10;
        out.push((p, 6, 2));
        return out.push((p, 5, 2));
    }
    let latin = match ch {
        '\u{E9}' | '\u{C9}' => Some((b'e', ch.is_uppercase(), 31)),
        '\u{EF}' | '\u{CF}' => Some((b'i', ch.is_uppercase(), 38)),
        '\u{F6}' | '\u{D6}' => Some((b'o', ch.is_uppercase(), 38)),
        '\u{FC}' | '\u{DC}' => Some((b'u', ch.is_uppercase(), 38)),
        '\u{130}' => Some((b'i', true, 37)),
        _ => None,
    };
    if let Some((b, upper, acc)) = latin {
        out.push((
            0x0500_0000 + u32::from(b - b'a') * 10,
            5,
            if upper { 8 } else { 2 },
        ));
        return out.push((0, acc, 2));
    }
    let cp = ch as u32;
    if ch == '\u{A0}' {
        out.push((0x0100_0003, 5, 2));
    } else if matches!(cp, 0x2000..=0x2BFF | 0x1F000..=0x1FFFF) {
        out.push((0x0300_0000 + cp, 5, 2));
    } else if ('\u{370}'..='\u{3FF}').contains(&ch) {
        let lc = ch.to_lowercase().next().unwrap_or(ch);
        let (base, acc) = match lc {
            '\u{3AF}' => (0x3B9, 31),
            '\u{3AC}' => (0x3B1, 31),
            '\u{3AD}' => (0x3B5, 31),
            '\u{3AE}' => (0x3B7, 31),
            '\u{3CC}' => (0x3BF, 31),
            '\u{3CD}' => (0x3C5, 31),
            '\u{3CE}' => (0x3C9, 31),
            '\u{3C2}' => (0x3C3, 5),
            _ => (lc as u32, 5),
        };
        out.push((0x0580_0000 + base, 5, if ch != lc { 8 } else { 2 }));
        if acc > 5 {
            out.push((0, acc, 2));
        }
    } else {
        let pri = if ('\u{30A1}'..='\u{30F6}').contains(&ch) {
            cp - 0x60
        } else {
            cp
        };
        out.push((0x0600_0000 + pri, 5, if pri != cp { 4 } else { 2 }));
    }
}

fn locale_compare_en(a: &str, b: &str) -> Ordering {
    let ces = |s: &str| {
        let mut v = Vec::with_capacity(s.len());
        s.chars().for_each(|c| push_char_ces(c, &mut v));
        v
    };
    let (ca, cb) = (ces(a), ces(b));
    ca.iter()
        .filter(|e| e.0 > 0)
        .map(|e| e.0)
        .cmp(cb.iter().filter(|e| e.0 > 0).map(|e| e.0))
        .then_with(|| ca.iter().map(|e| e.1).cmp(cb.iter().map(|e| e.1)))
        .then_with(|| ca.iter().map(|e| e.2).cmp(cb.iter().map(|e| e.2)))
}

/// Port of `groupModelIds` (`src/model-groups.ts:51`).
pub fn group_model_ids(models: Vec<String>) -> Result<Vec<Value>, String> {
    let mut seen = HashSet::new();
    let mut buckets: HashMap<&'static str, Vec<String>> = HashMap::new();
    for raw in models {
        let model = js_trim(&raw);
        if !model.is_empty() && seen.insert(model.to_string()) {
            buckets
                .entry(family_of(model))
                .or_default()
                .push(model.to_string());
        }
    }
    Ok(ORDER
        .split(',')
        .filter_map(|label| {
            let mut list = buckets.remove(label)?;
            list.sort_by(|a, b| locale_compare_en(a, b));
            Some(json!({ "label": label, "models": list }))
        })
        .collect())
}

/// Port of `filterModelGroups` (`src/model-groups.ts:75`).
pub fn filter_model_groups(models: Vec<String>, query: String) -> Result<Vec<Value>, String> {
    let normalized = js_trim(&query).to_lowercase();
    if normalized.is_empty() {
        return group_model_ids(models);
    }
    group_model_ids(
        models
            .into_iter()
            .filter(|m| m.to_lowercase().contains(&normalized))
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_sorts_and_filters_models() {
        let g = group_model_ids(vec![
            "claude-opus-5".into(),
            "gpt-5.4".into(),
            "gpt-5.2".into(),
        ])
        .unwrap();
        assert_eq!(
            g[0],
            json!({ "label": "GPT", "models": ["gpt-5.2", "gpt-5.4"] })
        );
        let s = group_model_ids(vec![
            "qwen3.5-max".into(),
            "qwen_vl".into(),
            "o3-mini".into(),
            "vendor-gpt-5".into(),
        ])
        .unwrap();
        assert_eq!(
            s[1],
            json!({ "label": "Qwen", "models": ["qwen_vl", "qwen3.5-max"] })
        );
        assert_eq!(
            filter_model_groups(vec!["gpt-5.4".into(), "gpt-5.4-mini".into()], "MINI".into())
                .unwrap()
                .len(),
            1
        );
    }
}
