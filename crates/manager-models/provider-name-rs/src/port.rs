//! The Rust port of `src/provider-name.ts`.

#![forbid(unsafe_code)]

use fancy_regex::Regex;
use unicode_script::{Script, UnicodeScript};

fn is_js_ws(c: char) -> bool {
    c.is_whitespace() || c == '\u{FEFF}'
}

fn js_trim(s: &str) -> &str {
    s.trim_matches(is_js_ws)
}

fn split_crlf_lines(s: &str) -> Vec<&str> {
    s.split('\n')
        .map(|line| line.strip_suffix('\r').unwrap_or(line))
        .collect()
}

fn parse_section_header(line: &str) -> Option<&str> {
    let trimmed = js_trim(line);
    if !trimmed.starts_with('[') || !trimmed.ends_with(']') || trimmed.len() < 3 {
        return None;
    }
    let inner = &trimmed[1..trimmed.len() - 1];
    if inner.contains(']') {
        return None;
    }
    Some(js_trim(inner))
}

fn unescape_toml_quotes(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut chars = raw.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\'
            && let Some(&next) = chars.peek()
            && matches!(next, '"' | '\'' | '\\')
        {
            out.push(next);
            chars.next();
            continue;
        }
        out.push(c);
    }
    out
}

/// Port of `tomlSectionStringValue` (`src/provider-name.ts:6`).
pub fn toml_section_string_value(
    contents: String,
    section_name: String,
    key: String,
) -> Result<String, String> {
    // Match JS: `new RegExp("^\\s*" + key + "\\s*=\\s*([\"'])(.*)\\1\\s*(?:#.*)?$")`
    // Reject unescaped `/` like JS RegExp syntax? Wait: in `new RegExp(".../x...")`, JS `new RegExp` allows `/`.
    // Use `(?-u)` for `\s`? Wait: in JS without `/u`, `\s` STILL matches Unicode whitespace (including `\uFEFF`),
    // while `(?-u)` in Rust regex makes `\s` ASCII-only! Wait: TOML keys/lines usually use ASCII spaces,
    // or we can use `(?u)` for `\s` (plus `\u{FEFF}`? Rust `\s` lacks `\u{FEFF}`). Better: `[\s\u{FEFF}]`.
    let pattern = format!(
        r#"^[\s\u{{FEFF}}]*{key}[\s\u{{FEFF}}]*=[\s\u{{FEFF}}]*(['"])(.*)\1[\s\u{{FEFF}}]*(?:#.*)?$"#
    );
    let re = Regex::new(&pattern)
        .map_err(|e| format!("SyntaxError: Invalid regular expression: {e}"))?;

    let mut in_section = false;
    for line in split_crlf_lines(&contents) {
        if let Some(sec) = parse_section_header(line) {
            in_section = sec == section_name;
            continue;
        }
        if !in_section {
            continue;
        }
        if let Some(caps) = re
            .captures(js_trim(line))
            .map_err(|e| format!("RegexError: {e}"))?
            && let Some(m) = caps.get(2)
        {
            return Ok(unescape_toml_quotes(m.as_str()));
        }
    }
    Ok(String::new())
}

/// Port of `resolveProviderName` (`src/provider-name.ts:24`).
pub fn resolve_provider_name(contents: String, provider: String) -> Result<String, String> {
    let raw = toml_section_string_value(
        contents,
        format!("model_providers.{provider}"),
        "name".to_string(),
    )?;
    let trimmed = js_trim(&raw);
    if trimmed.is_empty() {
        Ok(provider)
    } else {
        Ok(trimmed.to_string())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum AvatarCharKind {
    Han,
    Latin,
    Digit,
}

fn avatar_char_kind(ch: char) -> Option<AvatarCharKind> {
    if ch.is_ascii_alphabetic() {
        Some(AvatarCharKind::Latin)
    } else if ch.is_ascii_digit() {
        Some(AvatarCharKind::Digit)
    } else if ch.script() == Script::Han {
        Some(AvatarCharKind::Han)
    } else {
        None
    }
}

fn is_word_separator(c: char) -> bool {
    is_js_ws(c)
        || matches!(
            c,
            '(' | ')'
                | '（'
                | '）'
                | '['
                | ']'
                | '【'
                | '】'
                | ','
                | '，'
                | '.'
                | '。'
                | '·'
                | '_'
                | '-'
                | '/'
                | '|'
        )
}

fn upper(ch: char) -> String {
    ch.to_uppercase().to_string()
}

/// Port of `providerAvatarText` (`src/provider-name.ts:42`).
pub fn provider_avatar_text(name: String, fallback: Option<String>) -> Result<String, String> {
    let fallback = fallback.unwrap_or_else(|| "供".to_string());
    let trimmed = js_trim(&name);
    let meaningful: Vec<(char, AvatarCharKind)> = trimmed
        .chars()
        .filter_map(|c| avatar_char_kind(c).map(|k| (c, k)))
        .collect();
    let Some(&(first_char, first_kind)) = meaningful.first() else {
        return Ok(fallback);
    };
    if let Some(&(other_char, _)) = meaningful.iter().find(|(_, k)| *k != first_kind) {
        return Ok(format!("{}{}", upper(first_char), upper(other_char)));
    }
    let words: Vec<&str> = trimmed
        .split(is_word_separator)
        .filter(|w| !w.is_empty() && w.chars().any(|c| avatar_char_kind(c).is_some()))
        .collect();
    if words.len() >= 2
        && let Some(second) = words[1].chars().find(|&c| avatar_char_kind(c).is_some())
    {
        return Ok(format!("{}{}", upper(first_char), upper(second)));
    }
    if first_kind == AvatarCharKind::Latin
        && let Some(&(camel, _)) = meaningful[1..].iter().find(|(c, _)| c.is_ascii_uppercase())
    {
        return Ok(format!("{}{camel}", upper(first_char)));
    }
    if let Some(&(second, _)) = meaningful.get(1) {
        Ok(format!("{}{}", upper(first_char), upper(second)))
    } else {
        Ok(upper(first_char))
    }
}
