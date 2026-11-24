//! The Rust port of `src/auto-compact.ts`.

#![forbid(unsafe_code)]

pub const DEFAULT_AUTO_COMPACT_PERCENT: &str = "90%";
pub const AUTO_COMPACT_PRESETS: [&str; 8] =
    ["50%", "60%", "70%", "75%", "80%", "85%", "90%", "95%"];

fn is_js_ws(c: char) -> bool {
    c.is_whitespace() || c == '\u{FEFF}'
}

fn js_trim(s: &str) -> &str {
    s.trim_matches(is_js_ws)
}

fn is_valid_percent_pattern(s: &str) -> bool {
    let core = s.strip_suffix('%').unwrap_or(s);
    if core.is_empty() {
        return false;
    }
    match core.split_once('.') {
        Some((int_part, frac_part)) => {
            !int_part.is_empty()
                && int_part.bytes().all(|b| b.is_ascii_digit())
                && !frac_part.is_empty()
                && frac_part.len() <= 6
                && frac_part.bytes().all(|b| b.is_ascii_digit())
        }
        None => core.bytes().all(|b| b.is_ascii_digit()),
    }
}

/// Port of `autoCompactOptions` (`src/auto-compact.ts:10`).
pub fn auto_compact_options(current: String) -> Result<Vec<String>, String> {
    let normalized = normalize_auto_compact_percent(current)?;
    let chosen = if normalized.is_empty() {
        DEFAULT_AUTO_COMPACT_PERCENT.to_string()
    } else {
        normalized
    };
    let mut options: Vec<String> = AUTO_COMPACT_PRESETS
        .iter()
        .map(|s| (*s).to_string())
        .collect();
    if !options.iter().any(|item| item == &chosen) {
        options.push(chosen);
    }
    Ok(options)
}

/// Port of `isValidAutoCompactPercent` (`src/auto-compact.ts:20`).
pub fn is_valid_auto_compact_percent(value: String) -> Result<bool, String> {
    let trimmed = js_trim(&value);
    if trimmed.is_empty() {
        return Ok(true);
    }
    if !is_valid_percent_pattern(trimmed) {
        return Ok(false);
    }
    let body = trimmed.strip_suffix('%').unwrap_or(trimmed);
    let Ok(numeric) = body.parse::<f64>() else {
        return Ok(false);
    };
    Ok(numeric.is_finite() && numeric > 0.0 && numeric <= 100.0)
}

/// Port of `normalizeAutoCompactPercent` (`src/auto-compact.ts:29`).
pub fn normalize_auto_compact_percent(value: String) -> Result<String, String> {
    let trimmed = js_trim(&value);
    if trimmed.is_empty() {
        return Ok(String::new());
    }
    if !is_valid_auto_compact_percent(trimmed.to_string())? {
        return Ok(trimmed.to_string());
    }
    let body = trimmed.strip_suffix('%').unwrap_or(trimmed);
    Ok(format!("{}%", js_trim(body)))
}

/// Port of `normalizeAutoCompactEditing` (`src/auto-compact.ts:37`).
pub fn normalize_auto_compact_editing(
    value: String,
    _previous_value: Option<String>,
) -> Result<String, String> {
    let trimmed = js_trim(&value);
    if trimmed.is_empty() {
        return Ok(String::new());
    }
    let numeric = value.replace('%', "");
    if trimmed.ends_with('%') {
        return Ok(numeric);
    }
    if value.contains('%') {
        return Ok(format!("{numeric}%"));
    }
    Ok(numeric)
}
