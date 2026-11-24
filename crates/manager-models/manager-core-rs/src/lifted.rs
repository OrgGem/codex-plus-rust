//! Phase 6 idiomatic Pure-Rust domain lift (`lifted.rs`).
//!
//! Consolidates fragmented TS-shaped `serde_json::Value` / `Result<String, String>` / UTF-16 WASM
//! bridge helpers (`code-highlight`, `auto-compact`, `theme-mode`, `overview-model`, `provider-name`)
//! into zero-overhead, strongly-typed, borrowed-slice Rust APIs for in-process `rlib` consumers
//! such as `codex-plus-app`.

use std::borrow::Cow;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use unicode_script::{Script, UnicodeScript};

use crate::common::js_rt;

// ============================================================================
// 1. Zero-Copy Code Syntax Highlighter (`code-highlight.ts` lifted)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HighlightLanguage {
    Toml,
    Json,
}

impl HighlightLanguage {
    #[must_use]
    pub fn parse(raw: &str) -> Self {
        if raw.eq_ignore_ascii_case("json") {
            Self::Json
        } else {
            Self::Toml
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HighlightKind {
    Plain,
    Comment,
    Section,
    Key,
    String,
    Number,
    Boolean,
    Null,
    Date,
    Punct,
    Operator,
}

impl HighlightKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Plain => "plain",
            Self::Comment => "comment",
            Self::Section => "section",
            Self::Key => "key",
            Self::String => "string",
            Self::Number => "number",
            Self::Boolean => "boolean",
            Self::Null => "null",
            Self::Date => "date",
            Self::Punct => "punct",
            Self::Operator => "operator",
        }
    }

    #[must_use]
    pub const fn css_class(self) -> &'static str {
        match self {
            Self::Plain => "cx-tok-plain",
            Self::Comment => "cx-tok-comment",
            Self::Section => "cx-tok-section",
            Self::Key => "cx-tok-key",
            Self::String => "cx-tok-string",
            Self::Number => "cx-tok-number",
            Self::Boolean | Self::Null => "cx-tok-bool",
            Self::Date => "cx-tok-date",
            Self::Punct => "cx-tok-punct",
            Self::Operator => "cx-tok-op",
        }
    }
}

/// A zero-copy highlighted token referencing the original source slice `&'a str`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HighlightSpan<'a> {
    pub kind: HighlightKind,
    pub text: &'a str,
}

struct RangeSpan {
    kind: HighlightKind,
    start: usize,
    end: usize,
}

struct BorrowedLineBuilder<'a> {
    source: &'a str,
    lines: Vec<Vec<RangeSpan>>,
}

impl<'a> BorrowedLineBuilder<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            lines: vec![Vec::new()],
        }
    }

    fn newline(&mut self) {
        self.lines.push(Vec::new());
    }

    fn push_range(&mut self, kind: HighlightKind, start: usize, end: usize) {
        if start >= end {
            return;
        }
        let slice = &self.source[start..end];
        let mut cursor = start;
        for (idx, part) in slice.split('\n').enumerate() {
            if idx > 0 {
                self.newline();
                cursor += 1; // step past '\n'
            }
            let part_end = cursor + part.len();
            if cursor < part_end
                && let Some(line) = self.lines.last_mut()
            {
                if let Some(last) = line.last_mut()
                    && last.kind == kind
                    && last.end == cursor
                {
                    last.end = part_end;
                } else {
                    line.push(RangeSpan {
                        kind,
                        start: cursor,
                        end: part_end,
                    });
                }
            }
            cursor = part_end;
        }
    }

    fn finish(self) -> Vec<Vec<HighlightSpan<'a>>> {
        let src = self.source;
        self.lines
            .into_iter()
            .map(|line| {
                line.into_iter()
                    .map(|span| HighlightSpan {
                        kind: span.kind,
                        text: &src[span.start..span.end],
                    })
                    .collect()
            })
            .collect()
    }
}

fn is_ascii_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn is_bare_key_byte(b: u8) -> bool {
    is_ascii_word_byte(b) || b == b'-'
}

fn is_js_ws_char(c: char) -> bool {
    matches!(
        c,
        '\u{0009}'..='\u{000D}'
            | ' '
            | '\u{00A0}'
            | '\u{1680}'
            | '\u{2000}'..='\u{200A}'
            | '\u{2028}'
            | '\u{2029}'
            | '\u{202F}'
            | '\u{205F}'
            | '\u{3000}'
            | '\u{FEFF}'
    )
}

fn next_byte_or_end(bytes: &[u8], needle: u8, from: usize) -> usize {
    bytes[from..]
        .iter()
        .position(|&b| b == needle)
        .map_or(bytes.len(), |off| from + off)
}

fn last_byte_before(bytes: &[u8], needle: u8, end: usize) -> Option<usize> {
    let limit = end.min(bytes.len());
    let inclusive_end = if limit < bytes.len() {
        limit + 1
    } else {
        limit
    };
    bytes[..inclusive_end].iter().rposition(|&b| b == needle)
}

fn read_quoted_utf8(s: &str, start: usize, escaped: bool) -> usize {
    let bytes = s.as_bytes();
    let quote = bytes[start];
    let mut i = start + 1;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'\n' {
            break;
        }
        if escaped && b == b'\\' {
            let next_step = s
                .get(i + 1..)
                .and_then(|r| r.chars().next())
                .map_or(0, char::len_utf8);
            i = (i + 1 + next_step).min(bytes.len());
            continue;
        }
        if b == quote {
            return (i + 1) - start;
        }
        let step = s[i..].chars().next().map_or(1, char::len_utf8);
        i += step;
    }
    i.saturating_sub(start)
}

fn match_ascii_kw(bytes: &[u8], at: usize, kw: &[u8]) -> bool {
    bytes.get(at..at + kw.len()) == Some(kw)
        && !bytes
            .get(at + kw.len())
            .copied()
            .is_some_and(is_ascii_word_byte)
}

fn match_digits_underscore_bytes(bytes: &[u8], at: usize, pred: fn(u8) -> bool) -> usize {
    let mut i = at;
    while i < bytes.len() && (pred(bytes[i]) || bytes[i] == b'_') {
        i += 1;
    }
    i - at
}

fn match_number_bytes(bytes: &[u8], at: usize) -> usize {
    let mut i = at;
    if i < bytes.len() && matches!(bytes[i], b'+' | b'-') {
        i += 1;
    }
    if i >= bytes.len() {
        return 0;
    }
    let body_start = i;
    if i + 2 <= bytes.len() && bytes[i] == b'0' {
        match bytes[i + 1] {
            b'x' | b'X' => {
                let n = match_digits_underscore_bytes(bytes, i + 2, |b| b.is_ascii_hexdigit());
                if n > 0 {
                    return (i + 2 + n) - at;
                }
            }
            b'o' | b'O' => {
                let n = match_digits_underscore_bytes(bytes, i + 2, |b| (b'0'..=b'7').contains(&b));
                if n > 0 {
                    return (i + 2 + n) - at;
                }
            }
            b'b' | b'B' => {
                let n = match_digits_underscore_bytes(bytes, i + 2, |b| b == b'0' || b == b'1');
                if n > 0 {
                    return (i + 2 + n) - at;
                }
            }
            _ => {}
        }
    }
    if bytes[body_start].is_ascii_digit() {
        i += 1;
        i += match_digits_underscore_bytes(bytes, i, |b| b.is_ascii_digit());
        if i + 1 < bytes.len()
            && bytes[i] == b'.'
            && (bytes[i + 1].is_ascii_digit() || bytes[i + 1] == b'_')
        {
            i += 1;
            i += match_digits_underscore_bytes(bytes, i, |b| b.is_ascii_digit());
        }
        if i < bytes.len() && matches!(bytes[i], b'e' | b'E') {
            let mut exp = i + 1;
            if exp < bytes.len() && matches!(bytes[exp], b'+' | b'-') {
                exp += 1;
            }
            let mut digits = exp;
            while digits < bytes.len() && bytes[digits].is_ascii_digit() {
                digits += 1;
            }
            if digits > exp {
                i = digits;
            }
        }
        return i - at;
    }
    if matches!(bytes.get(body_start..body_start + 3), Some(b"nan" | b"inf")) {
        return (body_start + 3) - at;
    }
    0
}

fn match_date_bytes(bytes: &[u8], at: usize) -> usize {
    if at + 10 > bytes.len() {
        return 0;
    }
    let d4 = (0..4).all(|k| bytes[at + k].is_ascii_digit());
    let d2a = (5..7).all(|k| bytes[at + k].is_ascii_digit());
    let d2b = (8..10).all(|k| bytes[at + k].is_ascii_digit());
    if !(d4 && bytes[at + 4] == b'-' && d2a && bytes[at + 7] == b'-' && d2b) {
        return 0;
    }
    let mut end = at + 10;
    if end + 6 <= bytes.len()
        && matches!(bytes[end], b'T' | b't' | b' ')
        && bytes[end + 1].is_ascii_digit()
        && bytes[end + 2].is_ascii_digit()
        && bytes[end + 3] == b':'
        && bytes[end + 4].is_ascii_digit()
        && bytes[end + 5].is_ascii_digit()
    {
        end += 6;
        if end + 3 <= bytes.len()
            && bytes[end] == b':'
            && bytes[end + 1].is_ascii_digit()
            && bytes[end + 2].is_ascii_digit()
        {
            end += 3;
            if end + 1 < bytes.len() && bytes[end] == b'.' && bytes[end + 1].is_ascii_digit() {
                end += 2;
                while end < bytes.len() && bytes[end].is_ascii_digit() {
                    end += 1;
                }
            }
        }
        if end < bytes.len() && matches!(bytes[end], b'Z' | b'z') {
            end += 1;
        } else if end + 6 <= bytes.len()
            && matches!(bytes[end], b'+' | b'-')
            && bytes[end + 1].is_ascii_digit()
            && bytes[end + 2].is_ascii_digit()
            && bytes[end + 3] == b':'
            && bytes[end + 4].is_ascii_digit()
            && bytes[end + 5].is_ascii_digit()
        {
            end += 6;
        }
    }
    end - at
}

fn tokenize_toml_borrowed(s: &str) -> Vec<Vec<HighlightSpan<'_>>> {
    let bytes = s.as_bytes();
    let mut out = BorrowedLineBuilder::new(s);
    let mut i = 0usize;
    let mut expect_key = true;
    let mut brace_depth = 0usize;

    while i < bytes.len() {
        let b = bytes[i];
        if b == b'\n' {
            out.newline();
            i += 1;
            if brace_depth == 0 {
                expect_key = true;
            }
            continue;
        }
        if matches!(b, b' ' | b'\t' | b'\r') {
            out.push_range(HighlightKind::Plain, i, i + 1);
            i += 1;
            continue;
        }
        if b == b'#' {
            let end = next_byte_or_end(bytes, b'\n', i);
            out.push_range(HighlightKind::Comment, i, end);
            i = end;
            continue;
        }
        if expect_key && b == b'[' {
            let end = next_byte_or_end(bytes, b'\n', i);
            if let Some(close) = last_byte_before(bytes, b']', end)
                && close > i
            {
                out.push_range(HighlightKind::Section, i, close + 1);
                i = close + 1;
                continue;
            }
        }
        if !expect_key {
            let multiline_delim = if s[i..].starts_with("\"\"\"") {
                Some("\"\"\"")
            } else if s[i..].starts_with("'''") {
                Some("'''")
            } else {
                None
            };
            if let Some(delim) = multiline_delim {
                let end = s[i + 3..]
                    .find(delim)
                    .map_or(bytes.len(), |off| i + 3 + off + 3);
                out.push_range(HighlightKind::String, i, end);
                i = end;
                continue;
            }
        }
        if b == b'"' || b == b'\'' {
            let len = read_quoted_utf8(s, i, b == b'"');
            let kind = if expect_key {
                HighlightKind::Key
            } else {
                HighlightKind::String
            };
            out.push_range(kind, i, i + len);
            i += len;
            continue;
        }
        if b == b'=' {
            out.push_range(HighlightKind::Operator, i, i + 1);
            expect_key = false;
            i += 1;
            continue;
        }
        if b == b'{' {
            out.push_range(HighlightKind::Punct, i, i + 1);
            brace_depth += 1;
            expect_key = true;
            i += 1;
            continue;
        }
        if b == b'}' {
            out.push_range(HighlightKind::Punct, i, i + 1);
            brace_depth = brace_depth.saturating_sub(1);
            expect_key = false;
            i += 1;
            continue;
        }
        if b == b'[' || b == b']' {
            out.push_range(HighlightKind::Punct, i, i + 1);
            i += 1;
            continue;
        }
        if b == b',' {
            out.push_range(HighlightKind::Operator, i, i + 1);
            if brace_depth > 0 {
                expect_key = true;
            }
            i += 1;
            continue;
        }
        if b == b'.' {
            out.push_range(HighlightKind::Operator, i, i + 1);
            i += 1;
            continue;
        }
        if expect_key {
            let mut k = i;
            while k < bytes.len() && is_bare_key_byte(bytes[k]) {
                k += 1;
            }
            if k > i {
                out.push_range(HighlightKind::Key, i, k);
                i = k;
                continue;
            }
            let step = s[i..].chars().next().map_or(1, char::len_utf8);
            out.push_range(HighlightKind::Plain, i, i + step);
            i += step;
            continue;
        }
        let date_len = match_date_bytes(bytes, i);
        if date_len > 0 {
            out.push_range(HighlightKind::Date, i, i + date_len);
            i += date_len;
            continue;
        }
        if match_ascii_kw(bytes, i, b"true") {
            out.push_range(HighlightKind::Boolean, i, i + 4);
            i += 4;
            continue;
        }
        if match_ascii_kw(bytes, i, b"false") {
            out.push_range(HighlightKind::Boolean, i, i + 5);
            i += 5;
            continue;
        }
        if b.is_ascii_digit() || matches!(b, b'+' | b'-') {
            let num_len = match_number_bytes(bytes, i);
            if num_len > 0 {
                out.push_range(HighlightKind::Number, i, i + num_len);
                i += num_len;
                continue;
            }
        }
        let mut k = i;
        while k < bytes.len() && is_bare_key_byte(bytes[k]) {
            k += 1;
        }
        if k > i {
            out.push_range(HighlightKind::Plain, i, k);
            i = k;
            continue;
        }
        let step = s[i..].chars().next().map_or(1, char::len_utf8);
        out.push_range(HighlightKind::Plain, i, i + step);
        i += step;
    }
    out.finish()
}

fn tokenize_json_borrowed(s: &str) -> Vec<Vec<HighlightSpan<'_>>> {
    let bytes = s.as_bytes();
    let mut out = BorrowedLineBuilder::new(s);
    let mut i = 0usize;

    while i < bytes.len() {
        let b = bytes[i];
        if b == b'\n' {
            out.newline();
            i += 1;
            continue;
        }
        if b == b'"' {
            let len = read_quoted_utf8(s, i, true);
            let rest = &s[i + len..];
            let trimmed_after = rest.trim_start_matches(is_js_ws_char);
            let kind = if trimmed_after.starts_with(':') {
                HighlightKind::Key
            } else {
                HighlightKind::String
            };
            out.push_range(kind, i, i + len);
            i += len;
            continue;
        }
        if b == b'/' && i + 1 < bytes.len() && matches!(bytes[i + 1], b'/' | b'*') {
            let end = if bytes[i + 1] == b'/' {
                next_byte_or_end(bytes, b'\n', i)
            } else {
                s[i + 2..]
                    .find("*/")
                    .map_or(bytes.len(), |off| i + 2 + off + 2)
            };
            out.push_range(HighlightKind::Comment, i, end);
            i = end;
            continue;
        }
        if matches!(b, b'{' | b'}' | b'[' | b']') {
            out.push_range(HighlightKind::Punct, i, i + 1);
            i += 1;
            continue;
        }
        if matches!(b, b':' | b',') {
            out.push_range(HighlightKind::Operator, i, i + 1);
            i += 1;
            continue;
        }
        if match_ascii_kw(bytes, i, b"true") {
            out.push_range(HighlightKind::Boolean, i, i + 4);
            i += 4;
            continue;
        }
        if match_ascii_kw(bytes, i, b"false") {
            out.push_range(HighlightKind::Boolean, i, i + 5);
            i += 5;
            continue;
        }
        if match_ascii_kw(bytes, i, b"null") {
            out.push_range(HighlightKind::Null, i, i + 4);
            i += 4;
            continue;
        }
        if b.is_ascii_digit() || matches!(b, b'+' | b'-') {
            let num_len = match_number_bytes(bytes, i);
            if num_len > 0 {
                out.push_range(HighlightKind::Number, i, i + num_len);
                i += num_len;
                continue;
            }
        }
        let step = s[i..].chars().next().map_or(1, char::len_utf8);
        out.push_range(HighlightKind::Plain, i, i + step);
        i += step;
    }
    out.finish()
}

/// Tokenizes TOML or JSON source text into borrowed zero-copy `HighlightSpan<'a>` lines.
#[must_use]
pub fn tokenize_code_borrowed(
    text: &str,
    language: HighlightLanguage,
) -> Vec<Vec<HighlightSpan<'_>>> {
    match language {
        HighlightLanguage::Json => tokenize_json_borrowed(text),
        HighlightLanguage::Toml => tokenize_toml_borrowed(text),
    }
}

// ============================================================================
// 2. Strongly-Typed Theme Preference (`theme-mode.ts` lifted)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemePreference {
    Light,
    Dark,
    #[default]
    System,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResolvedTheme {
    Light,
    Dark,
}

impl ThemePreference {
    #[must_use]
    pub fn parse(raw: Option<&str>) -> Self {
        match raw {
            Some("dark") => Self::Dark,
            Some("light") => Self::Light,
            _ => Self::System,
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
            Self::System => "system",
        }
    }

    #[must_use]
    pub const fn resolve(self, system_prefers_dark: bool) -> ResolvedTheme {
        match self {
            Self::Dark => ResolvedTheme::Dark,
            Self::Light => ResolvedTheme::Light,
            Self::System => {
                if system_prefers_dark {
                    ResolvedTheme::Dark
                } else {
                    ResolvedTheme::Light
                }
            }
        }
    }
}

impl ResolvedTheme {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    #[must_use]
    pub const fn is_dark(self) -> bool {
        matches!(self, Self::Dark)
    }

    #[must_use]
    pub const fn toggled(self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
        }
    }
}

// ============================================================================
// 3. Borrowed Auto-Compact Policy (`auto-compact.ts` lifted)
// ============================================================================

pub const DEFAULT_AUTO_COMPACT_PERCENT: &str = "90%";
pub const AUTO_COMPACT_PRESETS: [&str; 8] =
    ["50%", "60%", "70%", "75%", "80%", "85%", "90%", "95%"];

fn is_valid_percent_pattern_str(s: &str) -> bool {
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

#[must_use]
pub fn is_valid_auto_compact_percent_str(value: &str) -> bool {
    let trimmed = js_rt::js_trim(value);
    if trimmed.is_empty() {
        return true;
    }
    if !is_valid_percent_pattern_str(trimmed) {
        return false;
    }
    let body = trimmed.strip_suffix('%').unwrap_or(trimmed);
    body.parse::<f64>()
        .is_ok_and(|numeric| numeric.is_finite() && numeric > 0.0 && numeric <= 100.0)
}

#[must_use]
pub fn normalize_auto_compact_percent_cow(value: &str) -> Cow<'_, str> {
    let trimmed = js_rt::js_trim(value);
    if trimmed.is_empty() || !is_valid_auto_compact_percent_str(trimmed) {
        return Cow::Borrowed(trimmed);
    }
    if trimmed.ends_with('%')
        && js_rt::js_trim(&trimmed[..trimmed.len() - 1]) == &trimmed[..trimmed.len() - 1]
    {
        return Cow::Borrowed(trimmed);
    }
    let body = js_rt::js_trim(trimmed.strip_suffix('%').unwrap_or(trimmed));
    Cow::Owned(format!("{body}%"))
}

#[must_use]
pub fn auto_compact_options_vec(current: &str) -> Vec<String> {
    let normalized = normalize_auto_compact_percent_cow(current);
    let chosen = if normalized.is_empty() {
        DEFAULT_AUTO_COMPACT_PERCENT
    } else {
        normalized.as_ref()
    };
    let mut options: Vec<String> = AUTO_COMPACT_PRESETS
        .iter()
        .map(|&s| s.to_string())
        .collect();
    if !options.iter().any(|item| item == chosen) {
        options.push(chosen.to_string());
    }
    options
}

// ============================================================================
// 4. Zero-Regex Provider Display Name & Avatar (`provider-name.ts` lifted)
// ============================================================================

fn parse_toml_section_header(line: &str) -> Option<(bool, &str)> {
    let trimmed = js_rt::js_trim(line);
    if !trimmed.starts_with('[') {
        return None;
    }
    let is_array = trimmed.starts_with("[[");
    let start = if is_array { 2 } else { 1 };
    let bytes = trimmed.as_bytes();
    let mut i = start;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'"' || b == b'\'' {
            let len = read_quoted_utf8(trimmed, i, b == b'"');
            if len < 2 || bytes.get(i + len - 1).copied() != Some(b) {
                return None;
            }
            i += len;
            continue;
        }
        if b == b']' {
            let after_idx = if is_array {
                if bytes.get(i + 1).copied() != Some(b']') {
                    return None;
                }
                i + 2
            } else {
                i + 1
            };
            let rest = js_rt::js_trim(&trimmed[after_idx..]);
            if !rest.is_empty() && !rest.starts_with('#') {
                return None;
            }
            let inner = js_rt::js_trim(&trimmed[start..i]);
            if inner.is_empty() {
                return None;
            }
            return Some((is_array, inner));
        }
        let step = trimmed[i..].chars().next().map_or(1, char::len_utf8);
        i += step;
    }
    None
}

fn unescape_toml_quoted(raw: &str) -> String {
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

/// Fast zero-regex lookup of `key = "..."` inside `[section_name]` in TOML contents.
#[must_use]
pub fn toml_section_string_value_fast(
    contents: &str,
    section_name: &str,
    key: &str,
) -> Option<String> {
    let mut in_section = false;
    for raw_line in contents.split('\n') {
        let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        if let Some((is_array, sec)) = parse_toml_section_header(line) {
            in_section = !is_array && sec == section_name;
            continue;
        }
        if !in_section {
            continue;
        }
        let trimmed = js_rt::js_trim(line);
        let Some(after_key) = trimmed.strip_prefix(key) else {
            continue;
        };
        let after_key_ws = js_rt::js_trim(after_key);
        let Some(after_eq) = after_key_ws.strip_prefix('=') else {
            continue;
        };
        let rhs = js_rt::js_trim(after_eq);
        let Some(quote) = rhs.chars().next().filter(|&c| c == '"' || c == '\'') else {
            continue;
        };
        // Match greedy `(.*)\1\s*(?:#.*)?$` like JS regex: find last matching `quote`
        // whose trailing remainder is empty or starts with `#` after whitespace.
        let inner_and_tail = &rhs[1..];
        for (pos, _) in inner_and_tail.rmatch_indices(quote) {
            let tail = js_rt::js_trim(&inner_and_tail[pos + 1..]);
            if tail.is_empty() || tail.starts_with('#') {
                return Some(unescape_toml_quoted(&inner_and_tail[..pos]));
            }
        }
    }
    None
}

#[must_use]
pub fn resolve_provider_name_fast<'a>(contents: &'a str, provider: &'a str) -> Cow<'a, str> {
    let section = format!("model_providers.{provider}");
    if let Some(val) = toml_section_string_value_fast(contents, &section, "name") {
        let trimmed = js_rt::js_trim(&val);
        if !trimmed.is_empty() {
            return Cow::Owned(trimmed.to_string());
        }
    }
    Cow::Borrowed(provider)
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

fn is_avatar_word_separator(c: char) -> bool {
    is_js_ws_char(c)
        || c.is_whitespace()
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

#[must_use]
pub fn provider_avatar_text_fast(name: &str, fallback: Option<&str>) -> String {
    let fb = fallback.unwrap_or("供");
    let trimmed = js_rt::js_trim(name);
    let meaningful: Vec<(char, AvatarCharKind)> = trimmed
        .chars()
        .filter_map(|c| avatar_char_kind(c).map(|k| (c, k)))
        .collect();
    let Some(&(first_char, first_kind)) = meaningful.first() else {
        return fb.to_string();
    };
    let up = |c: char| c.to_uppercase().to_string();
    if let Some(&(other_char, _)) = meaningful.iter().find(|(_, k)| *k != first_kind) {
        return format!("{}{}", up(first_char), up(other_char));
    }
    let words: Vec<&str> = trimmed
        .split(is_avatar_word_separator)
        .filter(|w| !w.is_empty() && w.chars().any(|c| avatar_char_kind(c).is_some()))
        .collect();
    if words.len() >= 2
        && let Some(second) = words[1].chars().find(|&c| avatar_char_kind(c).is_some())
    {
        return format!("{}{}", up(first_char), up(second));
    }
    if first_kind == AvatarCharKind::Latin
        && let Some(&(camel, _)) = meaningful[1..].iter().find(|(c, _)| c.is_ascii_uppercase())
    {
        return format!("{}{camel}", up(first_char));
    }
    if let Some(&(second, _)) = meaningful.get(1) {
        format!("{}{}", up(first_char), up(second))
    } else {
        up(first_char)
    }
}

// ============================================================================
// 5. O(N) Strongly-Typed Overview Request Metrics (`overview-model.ts` lifted)
// ============================================================================

pub const OVERVIEW_DEFAULT_WINDOW_MS: f64 = 24.0 * 60.0 * 60.0 * 1000.0;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentRequestsSummary {
    pub total: usize,
    pub failed: usize,
    pub success_rate: Option<f64>,
    pub avg_duration_ms: Option<i64>,
    pub top_model: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub struct RequestMetricInput<'a> {
    pub timestamp_ms: f64,
    pub duration_ms: f64,
    pub success: bool,
    pub model: &'a str,
}

/// Idiomatic `O(N)` aggregator over strongly-typed `RequestMetricInput` slices.
#[must_use]
pub fn summarize_recent_requests_typed(
    entries: &[RequestMetricInput<'_>],
    now_ms: f64,
    window_ms: Option<f64>,
) -> RecentRequestsSummary {
    let window = window_ms.unwrap_or(OVERVIEW_DEFAULT_WINDOW_MS);
    let mut total = 0usize;
    let mut succeeded = 0usize;
    let mut duration_sum = 0.0_f64;
    // Keyed by trimmed model slice -> (first_seen_order, count, latest_ts)
    let mut counts: HashMap<&str, (usize, usize, f64)> = HashMap::new();
    let mut order_seq = 0usize;

    for item in entries {
        if !(item.timestamp_ms <= now_ms && now_ms - item.timestamp_ms <= window) {
            continue;
        }
        total += 1;
        if item.success {
            succeeded += 1;
        }
        let clamped_dur = if item.duration_ms.is_nan() {
            f64::NAN
        } else {
            item.duration_ms.max(0.0)
        };
        duration_sum += clamped_dur;

        let model = js_rt::js_trim(item.model);
        if model.is_empty() {
            continue;
        }
        if let Some((_, count, latest)) = counts.get_mut(model) {
            *count += 1;
            if item.timestamp_ms.is_nan() || latest.is_nan() {
                *latest = f64::NAN;
            } else if item.timestamp_ms > *latest {
                *latest = item.timestamp_ms;
            }
        } else {
            counts.insert(model, (order_seq, 1, item.timestamp_ms));
            order_seq += 1;
        }
    }

    if total == 0 {
        return RecentRequestsSummary {
            total: 0,
            failed: 0,
            success_rate: None,
            avg_duration_ms: None,
            top_model: None,
        };
    }

    let mut ordered_models: Vec<(&str, usize, usize, f64)> = counts
        .into_iter()
        .map(|(m, (ord, cnt, latest))| (m, ord, cnt, latest))
        .collect();
    ordered_models.sort_by_key(|&(_, ord, _, _)| ord);

    let mut top_model: Option<String> = None;
    let mut best_count = 0usize;
    let mut best_latest = -1.0_f64;
    for (model, _, count, latest) in ordered_models {
        if count > best_count || (count == best_count && latest > best_latest) {
            top_model = Some(model.to_string());
            best_count = count;
            best_latest = latest;
        }
    }

    let failed = total - succeeded;
    let rate = succeeded as f64 / total as f64;
    let avg_dur = js_rt::math_round(duration_sum / total as f64);

    RecentRequestsSummary {
        total,
        failed,
        success_rate: rate.is_finite().then_some(rate),
        avg_duration_ms: avg_dur.is_finite().then_some(avg_dur as i64),
        top_model,
    }
}

/// Convenience bridge from raw `serde_json::Value` request log entries (such as SQLite/JSONL logs)
/// directly to `RecentRequestsSummary` without allocating intermediate `Value` maps.
#[must_use]
pub fn summarize_recent_request_values(
    entries: &[Value],
    now_ms: f64,
    window_ms: Option<f64>,
) -> RecentRequestsSummary {
    let mut typed: Vec<RequestMetricInput<'_>> = Vec::with_capacity(entries.len());
    for entry in entries {
        let Some(obj) = entry.as_object() else {
            continue;
        };
        let ts = obj
            .get("timestampMs")
            .and_then(Value::as_f64)
            .or_else(|| {
                obj.get("timestampMs")
                    .and_then(Value::as_str)
                    .and_then(|s| js_rt::js_trim(s).parse::<f64>().ok())
            })
            .unwrap_or(f64::NAN);
        let dur = obj.get("durationMs").and_then(Value::as_f64).unwrap_or(0.0);
        let success = match obj.get("success") {
            Some(Value::Bool(b)) => *b,
            Some(Value::Number(n)) => n.as_f64().is_some_and(|x| x != 0.0 && !x.is_nan()),
            Some(Value::String(s)) => !s.is_empty(),
            _ => obj
                .get("statusCode")
                .and_then(Value::as_u64)
                .is_some_and(|code| (200..400).contains(&code)),
        };
        let model = obj
            .get("model")
            .and_then(Value::as_str)
            .filter(|s| !s.is_empty())
            .or_else(|| {
                obj.get("upstreamModel")
                    .and_then(Value::as_str)
                    .filter(|s| !s.is_empty())
            })
            .unwrap_or("");
        typed.push(RequestMetricInput {
            timestamp_ms: ts,
            duration_ms: dur,
            success,
            model,
        });
    }
    summarize_recent_requests_typed(&typed, now_ms, window_ms)
}
