//! The Rust port of `src/dream-skin.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::{
    as_number, is_js_ws, is_nil, is_plain_object, is_truthy, is_undefined, js_trim,
    number_to_string, string_units, utf16_string,
};
use serde_json::{Map, Value, json};

fn default_colors_map() -> Map<String, Value> {
    let pairs = [
        ("background", "#F7F4F5"),
        ("panel", "#FFFFFF"),
        ("panelAlt", "#FFF7F8"),
        ("accent", "#E25563"),
        ("accentAlt", "#F07A86"),
        ("secondary", "#F3A8AF"),
        ("highlight", "#C93D4C"),
        ("text", "#2B2224"),
        ("muted", "#8A7A7D"),
        ("line", "rgba(196, 120, 128, .22)"),
    ];
    pairs
        .into_iter()
        .map(|(k, v)| (k.to_string(), Value::String(v.to_string())))
        .collect()
}

/// Port of `defaultDreamSkinColors` (`src/dream-skin.ts:195`).
pub fn default_dream_skin_colors() -> Result<Value, String> {
    Ok(Value::Object(default_colors_map()))
}

/// Port of `defaultDreamSkinTheme` (`src/dream-skin.ts:210`).
pub fn default_dream_skin_theme() -> Result<Value, String> {
    // Node `navigator.userAgent` is "Node.js/22", taking the non-Windows branch.
    Ok(json!({
        "schemaVersion": 1,
        "id": "custom-1784123441349",
        "name": "Dream Skin",
        "brandSubtitle": "CODEX DREAM SKIN",
        "tagline": "把喜欢的画面变成可交互的 Codex 工作台。",
        "projectPrefix": "选择项目 · ",
        "projectLabel": "◉  选择项目",
        "statusText": "THEME ONLINE",
        "quote": "Make something wonderful",
        "colors": default_dream_skin_colors()?,
        "image": "portal-hero.png",
        "promoTitle": "感谢 Passion8 赞助",
        "promoSub": "passion8.cc",
        "promoUrl": "https://passion8.cc/register?aff=TuPe",
    }))
}

fn valid_color(value: Option<&Value>) -> Option<String> {
    let raw = match value {
        Some(Value::String(s)) => s.clone(),
        Some(v) => string_units(v).map(|u| String::from_utf16_lossy(&u))?,
        None => return None,
    };
    let color = js_trim(&raw);
    if let Some(hex) = color.strip_prefix('#')
        && (3..=8).contains(&hex.len())
        && hex.bytes().all(|b| b.is_ascii_hexdigit())
    {
        return Some(color.to_string());
    }
    let lower_prefix = color.get(..5).map(|p| p.to_ascii_lowercase());
    let rest = if lower_prefix.as_deref() == Some("rgba(") {
        color.get(5..color.len().saturating_sub(1))
    } else if color.get(..4).map(|p| p.to_ascii_lowercase()).as_deref() == Some("rgb(") {
        color.get(4..color.len().saturating_sub(1))
    } else {
        None
    };
    if let Some(inner) = rest
        && color.ends_with(')')
    {
        let parts: Vec<&str> = inner.split(',').collect();
        if matches!(parts.len(), 3 | 4)
            && parts.iter().all(|part| {
                let p = js_trim(part);
                !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit() || b == b'.')
            })
        {
            return Some(color.to_string());
        }
    }
    None
}

fn text_or(value: Option<&Value>, fallback: &str) -> Value {
    if let Some(v) = value {
        if let Some(s) = v.as_str() {
            let trimmed = js_trim(s);
            if !trimmed.is_empty() {
                return Value::String(trimmed.to_string());
            }
        } else if let Some(units) = string_units(v) {
            let start = units
                .iter()
                .position(|&u| !char::from_u32(u as u32).is_some_and(is_js_ws))
                .unwrap_or(units.len());
            let end = units
                .iter()
                .rposition(|&u| !char::from_u32(u as u32).is_some_and(is_js_ws))
                .map_or(start, |i| i + 1);
            if start < end {
                return utf16_string(&units[start..end]);
            }
        }
    }
    Value::String(fallback.to_string())
}

fn spread_js(v: &Value) -> Map<String, Value> {
    let mut map = Map::new();
    match v {
        Value::Object(o) if is_plain_object(v) => {
            for (k, val) in o {
                map.insert(k.clone(), val.clone());
            }
        }
        Value::Array(arr) => {
            for (i, val) in arr.iter().enumerate() {
                map.insert(i.to_string(), val.clone());
            }
        }
        _ => {
            if let Some(units) = string_units(v) {
                for (i, u) in units.into_iter().enumerate() {
                    map.insert(i.to_string(), utf16_string(&[u]));
                }
            }
        }
    }
    map
}

/// Port of `resolveDreamSkinStylePreset` (`src/dream-skin.ts:261`).
pub fn resolve_dream_skin_style_preset(id: String, style_preset: Value) -> Result<String, String> {
    if let Some(units) = string_units(&style_preset) {
        let s = String::from_utf16_lossy(&units);
        let preset = js_trim(&s);
        if !preset.is_empty() && preset != "dream-original" {
            return Ok(preset.to_string());
        }
    }
    let mapped = match js_trim(&id) {
        "caishen-lite" => "caishen-lite",
        "caishen-max" => "caishen-max",
        "caishen-readable" => "caishen-readable",
        "export-night" => "export-night",
        "global-founder-bright" => "global-founder-bright",
        "mythic-guardian-noir" => "mythic-guardian-noir",
        "codex-snow-skin" => "codex-snow",
        "glass-vision" => "glass-vision",
        "preset-midnight-aurora" => "midnight-aurora",
        "preset-amber-dusk" => "amber-dusk",
        "preset-forest-mist" => "forest-mist",
        "preset-cyber-neon" => "cyber-neon",
        "preset-sakura-dawn" => "sakura-dawn",
        _ => "dream-original",
    };
    Ok(mapped.to_string())
}

/// Port of `normalizeDreamSkinTheme` (`src/dream-skin.ts:283`).
pub fn normalize_dream_skin_theme(value: Option<Value>) -> Result<Value, String> {
    let fallback_colors = default_colors_map();
    let val = value.as_ref().filter(|v| !is_nil(v));
    let obj = val
        .and_then(Value::as_object)
        .filter(|_| val.is_some_and(is_plain_object));
    let get = |k: &str| obj.and_then(|o| o.get(k));

    let mut normalized = val.map(spread_js).unwrap_or_default();
    // schemaVersion === 1 ? 1 : fallback.schemaVersion (1)
    normalized.insert("schemaVersion".into(), json!(1));
    normalized.insert("id".into(), text_or(get("id"), "custom-1784123441349"));
    normalized.insert("name".into(), text_or(get("name"), "Dream Skin"));
    normalized.insert(
        "brandSubtitle".into(),
        text_or(get("brandSubtitle"), "CODEX DREAM SKIN"),
    );
    normalized.insert(
        "tagline".into(),
        text_or(get("tagline"), "把喜欢的画面变成可交互的 Codex 工作台。"),
    );
    normalized.insert(
        "projectPrefix".into(),
        text_or(get("projectPrefix"), "选择项目 · "),
    );
    normalized.insert(
        "projectLabel".into(),
        text_or(get("projectLabel"), "◉  选择项目"),
    );
    normalized.insert(
        "statusText".into(),
        text_or(get("statusText"), "THEME ONLINE"),
    );
    normalized.insert(
        "quote".into(),
        text_or(get("quote"), "Make something wonderful"),
    );

    let style_preset_val = get("stylePreset").filter(|v| string_units(v).is_some());
    if let Some(sp) = style_preset_val {
        let trimmed = text_or(Some(sp), "");
        if trimmed.as_str().is_some_and(|s| !s.is_empty())
            || string_units(&trimmed).is_some_and(|u| !u.is_empty())
        {
            normalized.insert("stylePreset".into(), trimmed);
        } else {
            normalized.shift_remove("stylePreset");
        }
    } else {
        normalized.shift_remove("stylePreset");
    }

    if let Some(colors_val) = get("colors").filter(|v| is_truthy(v)) {
        let colors_obj = Some(colors_val)
            .filter(|v| is_plain_object(v))
            .and_then(Value::as_object);
        let mut merged = spread_js(colors_val);
        for (k, def_val) in &fallback_colors {
            let def_str = def_val.as_str().unwrap_or("");
            let candidate = colors_obj.and_then(|o| o.get(k.as_str()));
            let resolved = valid_color(candidate).unwrap_or_else(|| def_str.to_string());
            merged.insert(k.clone(), Value::String(resolved));
        }
        normalized.insert("colors".into(), Value::Object(merged));
    } else if val.is_none_or(|v| !is_truthy(v)) {
        normalized.insert("colors".into(), Value::Object(fallback_colors));
    } else {
        normalized.shift_remove("colors");
    }

    Ok(Value::Object(normalized))
}

fn push_json_str_from_units(units: &[u16], out: &mut String) {
    out.push('"');
    let mut i = 0;
    while i < units.len() {
        let u = units[i];
        if (0xD800..=0xDBFF).contains(&u)
            && i + 1 < units.len()
            && (0xDC00..=0xDFFF).contains(&units[i + 1])
        {
            let cp = 0x10000 + ((u32::from(u) - 0xD800) << 10) + (u32::from(units[i + 1]) - 0xDC00);
            if let Some(ch) = char::from_u32(cp) {
                out.push(ch);
            }
            i += 2;
        } else if (0xD800..=0xDFFF).contains(&u) {
            out.push_str(&format!("\\u{u:04x}"));
            i += 1;
        } else if let Some(ch) = char::from_u32(u32::from(u)) {
            match ch {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\x08' => out.push_str("\\b"),
                '\x0c' => out.push_str("\\f"),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
                c => out.push(c),
            }
            i += 1;
        } else {
            i += 1;
        }
    }
    out.push('"');
}

fn json_stringify_into(v: &Value, out: &mut String) -> Result<(), String> {
    if is_undefined(v) || v.is_null() {
        out.push_str("null");
        return Ok(());
    }
    if let Some(b) = v.as_bool() {
        out.push_str(if b { "true" } else { "false" });
        return Ok(());
    }
    if let Some(n) = as_number(v) {
        if !n.is_finite() {
            out.push_str("null");
        } else {
            out.push_str(&number_to_string(n));
        }
        return Ok(());
    }
    if let Some(s) = v.as_str() {
        let encoded = serde_json::to_string(s).map_err(|e| e.to_string())?;
        out.push_str(&encoded);
        return Ok(());
    }
    if let Some(units) = string_units(v) {
        push_json_str_from_units(&units, out);
        return Ok(());
    }
    match v {
        Value::Array(arr) => {
            out.push('[');
            for (idx, item) in arr.iter().enumerate() {
                if idx > 0 {
                    out.push(',');
                }
                json_stringify_into(item, out)?;
            }
            out.push(']');
        }
        Value::Object(map) => {
            out.push('{');
            let mut first = true;
            for (k, val) in map {
                if is_undefined(val) {
                    continue;
                }
                if !first {
                    out.push(',');
                }
                first = false;
                let key_encoded = serde_json::to_string(k).map_err(|e| e.to_string())?;
                out.push_str(&key_encoded);
                out.push(':');
                json_stringify_into(val, out)?;
            }
            out.push('}');
        }
        _ => out.push_str("null"),
    }
    Ok(())
}

/// Port of `dreamSkinDraftFingerprint` (`src/dream-skin.ts:333`).
pub fn dream_skin_draft_fingerprint(draft: Value) -> Result<String, String> {
    if is_nil(&draft) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'config')".into(),
        );
    }
    let obj = draft.as_object().filter(|_| is_plain_object(&draft));
    let config_arg = obj.and_then(|o| o.get("config")).cloned();
    let config = normalize_dream_skin_theme(config_arg)?;
    let image_path_val = obj.and_then(|o| o.get("imagePath"));
    let Some(image_path_raw) = image_path_val.filter(|v| string_units(v).is_some()) else {
        return Err("TypeError: Cannot read properties of undefined (reading 'trim')".into());
    };
    let image_path = text_or(Some(image_path_raw), "");
    let builtin = obj
        .and_then(|o| o.get("builtin"))
        .cloned()
        .unwrap_or_else(manager_common_rs::undefined);
    let payload = json!({
        "config": config,
        "imagePath": image_path,
        "builtin": builtin,
    });
    let mut out = String::new();
    json_stringify_into(&payload, &mut out)?;
    Ok(out)
}

/// Port of `isDreamSkinDraftDirty` (`src/dream-skin.ts:341`).
pub fn is_dream_skin_draft_dirty(saved: Value, draft: Value) -> Result<bool, String> {
    let a = dream_skin_draft_fingerprint(saved)?;
    let b = dream_skin_draft_fingerprint(draft)?;
    Ok(a != b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_and_normalize_behavior() {
        let theme = default_dream_skin_theme().expect("default theme");
        assert_eq!(theme["schemaVersion"], 1);
        assert_eq!(theme["name"], "Dream Skin");
        assert_eq!(theme["colors"]["accent"], "#E25563");

        let mut mod_theme = theme.clone();
        mod_theme["name"] = json!("My Theme");
        mod_theme["colors"]["accent"] = json!("javascript:bad");
        let norm = normalize_dream_skin_theme(Some(mod_theme)).expect("normalize");
        assert_eq!(norm["name"], "My Theme");
        assert_eq!(norm["colors"]["accent"], "#E25563");
    }

    #[test]
    fn style_preset_and_dirty_checks() {
        assert_eq!(
            resolve_dream_skin_style_preset("preset-cyber-neon".into(), json!(null)).unwrap(),
            "cyber-neon"
        );
        assert_eq!(
            resolve_dream_skin_style_preset("codex-snow-skin".into(), json!("dream-original"))
                .unwrap(),
            "codex-snow"
        );
        let d1 = json!({ "config": default_dream_skin_theme().unwrap(), "imagePath": "current.png", "builtin": false });
        let d2 = json!({ "config": default_dream_skin_theme().unwrap(), "imagePath": "  current.png  ", "builtin": false });
        assert!(!is_dream_skin_draft_dirty(d1.clone(), d2).unwrap());
        assert!(dream_skin_draft_fingerprint(Value::Null).is_err());
    }
}
