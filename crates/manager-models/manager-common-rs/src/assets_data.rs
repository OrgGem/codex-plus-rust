//! Embedded static assets extracted read-only from `apps/codex-plus-manager/src/` and `codex-plus-core`.

#![forbid(unsafe_code)]

use crate::js_rt::{is_nil, js_str};
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::OnceLock;

pub const PRESETS_JSON: &str = include_str!("../assets/presets.json");
pub const I18N_EN_JSON: &str = include_str!("../assets/i18n-en.json");
pub const ENHANCEMENT_SETTINGS_JSON: &str =
    include_str!("../../../../../crates/codex-plus-core/assets/enhancement-settings.json");

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawI18nData {
    plain: HashMap<String, String>,
    template: HashMap<String, String>,
    backend: HashMap<String, String>,
    backend_patterns: Vec<(String, String, String)>,
}

pub struct I18nDict {
    pub plain: HashMap<String, String>,
    pub template: HashMap<String, String>,
    pub backend: HashMap<String, String>,
    pub backend_patterns: Vec<(Regex, String)>,
}

static PRESETS_CACHE: OnceLock<Value> = OnceLock::new();
static ENHANCEMENT_CACHE: OnceLock<Value> = OnceLock::new();
static I18N_CACHE: OnceLock<I18nDict> = OnceLock::new();

pub fn presets_value() -> &'static Value {
    PRESETS_CACHE.get_or_init(|| serde_json::from_str(PRESETS_JSON).expect("valid presets.json"))
}

pub fn enhancement_settings_value() -> &'static Value {
    ENHANCEMENT_CACHE.get_or_init(|| {
        serde_json::from_str(ENHANCEMENT_SETTINGS_JSON).expect("valid enhancement-settings.json")
    })
}

pub fn i18n_en() -> &'static I18nDict {
    I18N_CACHE.get_or_init(|| {
        let raw: RawI18nData = serde_json::from_str(I18N_EN_JSON).expect("valid i18n-en.json");
        let patterns = raw
            .backend_patterns
            .into_iter()
            .filter_map(|(src, _flags, rep)| Regex::new(&src).ok().map(|re| (re, rep)))
            .collect();
        I18nDict {
            plain: raw.plain,
            template: raw.template,
            backend: raw.backend,
            backend_patterns: patterns,
        }
    })
}

/// Port of `t(zh)` given active language (`"zh"` | `"en"`).
pub fn translate_plain(zh: &str, lang: &str) -> String {
    if lang != "en" {
        return zh.to_string();
    }
    let dict = i18n_en();
    if let Some(v) = dict.plain.get(zh).or_else(|| dict.backend.get(zh)) {
        return v.clone();
    }
    for (re, rep) in &dict.backend_patterns {
        if re.is_match(zh) {
            return re.replace(zh, rep.as_str()).into_owned();
        }
    }
    zh.to_string()
}

/// Port of `tf(key, args)` given active language (`"zh"` | `"en"`).
pub fn translate_template(key: &str, args: &[Value], lang: &str) -> String {
    let dict = i18n_en();
    let tmpl = if lang == "en" {
        dict.template.get(key).map(String::as_str).unwrap_or(key)
    } else {
        key
    };
    let mut out = String::with_capacity(tmpl.len());
    let bytes = tmpl.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'{'
            && let Some(rel_end) = bytes[i + 1..].iter().position(|&b| b == b'}')
            && rel_end > 0
            && bytes[i + 1..i + 1 + rel_end]
                .iter()
                .all(|b| b.is_ascii_digit())
            && let Ok(idx) = std::str::from_utf8(&bytes[i + 1..i + 1 + rel_end])
                .unwrap_or("")
                .parse::<usize>()
        {
            if let Some(arg) = args.get(idx)
                && !is_nil(arg)
            {
                out.push_str(&js_str(arg));
            } else {
                out.push_str(&tmpl[i..=i + rel_end + 1]);
            }
            i += rel_end + 2;
            continue;
        }
        let ch = tmpl[i..].chars().next().unwrap_or('\0');
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}
