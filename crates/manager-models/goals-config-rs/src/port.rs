//! The Rust port of `src/goals-config.ts`.

#![forbid(unsafe_code)]

use serde_json::{Value, json};

fn is_js_ws(c: char) -> bool {
    c.is_whitespace() || c == '\u{FEFF}'
}

fn js_trim(s: &str) -> &str {
    s.trim_matches(is_js_ws)
}

fn js_trim_end(s: &str) -> &str {
    s.trim_end_matches(is_js_ws)
}

fn split_crlf_lines(s: &str) -> Vec<&str> {
    s.split('\n')
        .map(|line| line.strip_suffix('\r').unwrap_or(line))
        .collect()
}

fn is_features_header(trimmed: &str) -> bool {
    trimmed == "[features]"
}

fn is_any_section_header(trimmed: &str) -> bool {
    trimmed.starts_with('[')
        && trimmed.ends_with(']')
        && trimmed.len() >= 3
        && !trimmed[1..trimmed.len() - 1].contains(']')
}

fn is_ascii_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn match_goals_eq_prefix(trimmed: &str) -> Option<&str> {
    let rest = trimmed.strip_prefix("goals")?;
    let after_ws = rest.trim_start_matches(is_js_ws);
    let after_eq = after_ws.strip_prefix('=')?;
    Some(after_eq.trim_start_matches(is_js_ws))
}

fn match_goals_bool(trimmed: &str) -> Option<bool> {
    let val = match_goals_eq_prefix(trimmed)?;
    if let Some(after) = val.strip_prefix("true") {
        let next_is_word = after
            .as_bytes()
            .first()
            .copied()
            .is_some_and(is_ascii_word_byte);
        if !next_is_word {
            return Some(true);
        }
    }
    if let Some(after) = val.strip_prefix("false") {
        let next_is_word = after
            .as_bytes()
            .first()
            .copied()
            .is_some_and(is_ascii_word_byte);
        if !next_is_word {
            return Some(false);
        }
    }
    None
}

fn ensure_trailing_newline(value: &str) -> String {
    if value.ends_with('\n') {
        value.to_string()
    } else {
        format!("{value}\n")
    }
}

fn join_toml_sections(sections: &[String]) -> String {
    let normalized: Vec<&str> = sections
        .iter()
        .map(|s| js_trim(s))
        .filter(|s| !s.is_empty())
        .collect();
    if normalized.is_empty() {
        String::new()
    } else {
        format!("{}\n", normalized.join("\n\n"))
    }
}

/// Port of `codexGoalsFeatureValue` (`src/goals-config.ts:6`).
pub fn codex_goals_feature_value(config_contents: String) -> Result<Option<bool>, String> {
    let mut in_features = false;
    for line in split_crlf_lines(&config_contents) {
        let trimmed = js_trim(line);
        if is_features_header(trimmed) {
            in_features = true;
            continue;
        }
        if in_features && is_any_section_header(trimmed) {
            in_features = false;
        }
        if in_features && let Some(val) = match_goals_bool(trimmed) {
            return Ok(Some(val));
        }
    }
    Ok(None)
}

/// Port of `codexGoalsFeatureState` (`src/goals-config.ts:25`).
pub fn codex_goals_feature_state(
    profile_config_contents: String,
    common_config_contents: String,
    use_common_config: bool,
) -> Result<Value, String> {
    if let Some(profile_value) = codex_goals_feature_value(profile_config_contents)? {
        return Ok(json!({ "enabled": profile_value, "inherited": false }));
    }
    let common_value = if use_common_config {
        codex_goals_feature_value(common_config_contents)?
    } else {
        None
    };
    if let Some(common_value) = common_value {
        return Ok(json!({ "enabled": common_value, "inherited": true }));
    }
    Ok(json!({ "enabled": false, "inherited": false }))
}

/// Port of `setCodexGoalsFeatureInConfig` (`src/goals-config.ts:41`).
pub fn set_codex_goals_feature_in_config(
    config_contents: String,
    enabled: bool,
) -> Result<String, String> {
    let lines = split_crlf_lines(&config_contents);
    let mut next: Vec<String> = Vec::with_capacity(lines.len() + 1);
    let mut in_features = false;
    let mut saw_features = false;
    let mut features_has_goals = false;
    let mut features_header_index: usize = 0;

    let goals_line = || format!("goals = {}", if enabled { "true" } else { "false" });

    let maybe_insert_goals = |next: &mut Vec<String>,
                              saw_features: bool,
                              features_has_goals: &mut bool,
                              features_header_index: usize| {
        if saw_features && !*features_has_goals {
            next.insert(features_header_index + 1, goals_line());
            *features_has_goals = true;
        }
    };

    for line in lines {
        let trimmed = js_trim(line);
        if is_features_header(trimmed) {
            if in_features {
                maybe_insert_goals(
                    &mut next,
                    saw_features,
                    &mut features_has_goals,
                    features_header_index,
                );
            }
            in_features = true;
            saw_features = true;
            features_has_goals = false;
            next.push(line.to_string());
            features_header_index = next.len() - 1;
            continue;
        }
        if in_features && is_any_section_header(trimmed) {
            maybe_insert_goals(
                &mut next,
                saw_features,
                &mut features_has_goals,
                features_header_index,
            );
            in_features = false;
        }
        if in_features && match_goals_eq_prefix(trimmed).is_some() {
            if !features_has_goals {
                next.push(goals_line());
                features_has_goals = true;
            }
            continue;
        }
        next.push(line.to_string());
    }

    if in_features {
        maybe_insert_goals(
            &mut next,
            saw_features,
            &mut features_has_goals,
            features_header_index,
        );
    }
    let joined = next.join("\n");
    let trimmed_end = ensure_trailing_newline(js_trim_end(&joined));
    if !saw_features {
        return Ok(join_toml_sections(&[
            trimmed_end,
            format!("[features]\n{}", goals_line()),
        ]));
    }

    Ok(trimmed_end)
}
