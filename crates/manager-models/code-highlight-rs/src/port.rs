//! The Rust port of `src/code-highlight.ts`.

#![forbid(unsafe_code)]

use crate::{js, utf16};
use serde_json::{Value, json};

#[derive(Clone, Copy, PartialEq, Eq)]
enum TokenKind {
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

impl TokenKind {
    fn as_str(self) -> &'static str {
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
}

struct TokenItem {
    kind: TokenKind,
    units: Vec<u16>,
}

struct LineBuilder {
    lines: Vec<Vec<TokenItem>>,
}

impl LineBuilder {
    fn new() -> Self {
        Self {
            lines: vec![vec![]],
        }
    }

    fn newline(&mut self) {
        self.lines.push(vec![]);
    }

    fn push_str(&mut self, kind: TokenKind, text: &str) {
        let units: Vec<u16> = text.encode_utf16().collect();
        self.push_units(kind, &units);
    }

    fn push_units(&mut self, kind: TokenKind, units: &[u16]) {
        if units.is_empty() {
            return;
        }
        let nl = u16::from(b'\n');
        for (index, part) in units.split(|&u| u == nl).enumerate() {
            if index > 0 {
                self.newline();
            }
            if part.is_empty() {
                continue;
            }
            if let Some(line) = self.lines.last_mut() {
                if let Some(last) = line.last_mut()
                    && last.kind == kind
                {
                    last.units.extend_from_slice(part);
                } else {
                    line.push(TokenItem {
                        kind,
                        units: part.to_vec(),
                    });
                }
            }
        }
    }

    fn into_json(self) -> Vec<Vec<Value>> {
        self.lines
            .into_iter()
            .map(|line| {
                line.into_iter()
                    .map(|t| json!({ "kind": t.kind.as_str(), "text": js::utf16_string(&t.units) }))
                    .collect()
            })
            .collect()
    }
}

fn u16_eq_ascii(u: u16, ch: u8) -> bool {
    u == u16::from(ch)
}

fn is_ascii_digit(u: u16) -> bool {
    (u16::from(b'0')..=u16::from(b'9')).contains(&u)
}

fn is_ascii_word(u: u16) -> bool {
    (u16::from(b'A')..=u16::from(b'Z')).contains(&u)
        || (u16::from(b'a')..=u16::from(b'z')).contains(&u)
        || is_ascii_digit(u)
        || u == u16::from(b'_')
}

fn is_bare_key_unit(u: u16) -> bool {
    is_ascii_word(u) || u == u16::from(b'-')
}

fn is_js_ws_u16(u: u16) -> bool {
    matches!(
        u,
        0x0009..=0x000D
            | 0x0020
            | 0x00A0
            | 0x1680
            | 0x2000..=0x200A
            | 0x2028
            | 0x2029
            | 0x202F
            | 0x205F
            | 0x3000
            | 0xFEFF
    )
}

fn starts_with_units(u: &[u16], at: usize, needle: &[u16]) -> bool {
    u.get(at..at + needle.len()) == Some(needle)
}

fn next_unit_or(u: &[u16], needle: u16, from: usize) -> usize {
    u[from..]
        .iter()
        .position(|&x| x == needle)
        .map_or(u.len(), |pos| from + pos)
}

fn last_unit_before(u: &[u16], needle: u16, end: usize) -> Option<usize> {
    let limit = end.min(u.len());
    let inclusive_end = if limit < u.len() { limit + 1 } else { limit };
    u[..inclusive_end].iter().rposition(|&x| x == needle)
}

fn index_of_units_or_end(u: &[u16], needle: &[u16], from: usize) -> usize {
    utf16::index_of(u, needle, from).unwrap_or(u.len())
}

fn read_quoted(u: &[u16], start: usize, escaped: bool) -> usize {
    let quote = u[start];
    let mut i = start + 1;
    while i < u.len() {
        let ch = u[i];
        if ch == u16::from(b'\n') {
            break;
        }
        if escaped && ch == u16::from(b'\\') {
            i += 2;
            continue;
        }
        i += 1;
        if ch == quote {
            return i - start;
        }
    }
    i.saturating_sub(start)
}

fn match_ascii_keyword(u: &[u16], at: usize, kw: &[u8]) -> bool {
    if at + kw.len() > u.len() {
        return false;
    }
    for (idx, &b) in kw.iter().enumerate() {
        if u[at + idx] != u16::from(b) {
            return false;
        }
    }
    !u.get(at + kw.len()).copied().is_some_and(is_ascii_word)
}

fn match_bare_key_len(u: &[u16], at: usize) -> usize {
    let mut i = at;
    while i < u.len() && is_bare_key_unit(u[i]) {
        i += 1;
    }
    i - at
}

fn match_digits_underscore(u: &[u16], at: usize, pred: fn(u16) -> bool) -> usize {
    let mut i = at;
    while i < u.len() && (pred(u[i]) || u[i] == u16::from(b'_')) {
        i += 1;
    }
    i - at
}

fn is_hex(u: u16) -> bool {
    is_ascii_digit(u)
        || (u16::from(b'a')..=u16::from(b'f')).contains(&u)
        || (u16::from(b'A')..=u16::from(b'F')).contains(&u)
}

fn is_oct(u: u16) -> bool {
    (u16::from(b'0')..=u16::from(b'7')).contains(&u)
}

fn is_bin(u: u16) -> bool {
    u == u16::from(b'0') || u == u16::from(b'1')
}

fn match_number_len(u: &[u16], at: usize) -> usize {
    let mut i = at;
    if i < u.len() && (u[i] == u16::from(b'+') || u[i] == u16::from(b'-')) {
        i += 1;
    }
    if i >= u.len() {
        return 0;
    }
    let body_start = i;
    if i + 2 <= u.len() && u[i] == u16::from(b'0') {
        let kind = u[i + 1];
        if kind == u16::from(b'x') || kind == u16::from(b'X') {
            let n = match_digits_underscore(u, i + 2, is_hex);
            if n > 0 {
                return (i + 2 + n) - at;
            }
        } else if kind == u16::from(b'o') || kind == u16::from(b'O') {
            let n = match_digits_underscore(u, i + 2, is_oct);
            if n > 0 {
                return (i + 2 + n) - at;
            }
        } else if kind == u16::from(b'b') || kind == u16::from(b'B') {
            let n = match_digits_underscore(u, i + 2, is_bin);
            if n > 0 {
                return (i + 2 + n) - at;
            }
        }
    }
    if is_ascii_digit(u[body_start]) {
        i += 1;
        i += match_digits_underscore(u, i, is_ascii_digit);
        if i + 1 < u.len()
            && u[i] == u16::from(b'.')
            && (is_ascii_digit(u[i + 1]) || u[i + 1] == u16::from(b'_'))
        {
            i += 1;
            i += match_digits_underscore(u, i, is_ascii_digit);
        }
        if i < u.len() && (u[i] == u16::from(b'e') || u[i] == u16::from(b'E')) {
            let mut exp = i + 1;
            if exp < u.len() && (u[exp] == u16::from(b'+') || u[exp] == u16::from(b'-')) {
                exp += 1;
            }
            let mut digits = exp;
            while digits < u.len() && is_ascii_digit(u[digits]) {
                digits += 1;
            }
            if digits > exp {
                i = digits;
            }
        }
        return i - at;
    }
    if u.get(body_start..body_start + 3)
        == Some(&[u16::from(b'n'), u16::from(b'a'), u16::from(b'n')])
        || u.get(body_start..body_start + 3)
            == Some(&[u16::from(b'i'), u16::from(b'n'), u16::from(b'f')])
    {
        return (body_start + 3) - at;
    }
    0
}

fn match_date_len(u: &[u16], at: usize) -> usize {
    if at + 10 > u.len() {
        return 0;
    }
    let d4 = (0..4).all(|k| is_ascii_digit(u[at + k]));
    let d2a = (5..7).all(|k| is_ascii_digit(u[at + k]));
    let d2b = (8..10).all(|k| is_ascii_digit(u[at + k]));
    if !(d4 && u[at + 4] == u16::from(b'-') && d2a && u[at + 7] == u16::from(b'-') && d2b) {
        return 0;
    }
    let mut end = at + 10;
    if end + 6 <= u.len()
        && matches!(u[end], 0x54 | 0x74 | 0x20)
        && is_ascii_digit(u[end + 1])
        && is_ascii_digit(u[end + 2])
        && u[end + 3] == u16::from(b':')
        && is_ascii_digit(u[end + 4])
        && is_ascii_digit(u[end + 5])
    {
        end += 6;
        if end + 3 <= u.len()
            && u[end] == u16::from(b':')
            && is_ascii_digit(u[end + 1])
            && is_ascii_digit(u[end + 2])
        {
            end += 3;
            if end + 1 < u.len() && u[end] == u16::from(b'.') && is_ascii_digit(u[end + 1]) {
                end += 2;
                while end < u.len() && is_ascii_digit(u[end]) {
                    end += 1;
                }
            }
        }
        if end < u.len() && (u[end] == u16::from(b'Z') || u[end] == u16::from(b'z')) {
            end += 1;
        } else if end + 6 <= u.len()
            && (u[end] == u16::from(b'+') || u[end] == u16::from(b'-'))
            && is_ascii_digit(u[end + 1])
            && is_ascii_digit(u[end + 2])
            && u[end + 3] == u16::from(b':')
            && is_ascii_digit(u[end + 4])
            && is_ascii_digit(u[end + 5])
        {
            end += 6;
        }
    }
    end - at
}

fn tokenize_toml(u: &[u16]) -> Vec<Vec<Value>> {
    let mut out = LineBuilder::new();
    let mut i = 0usize;
    let mut expect_key = true;
    let mut brace_depth = 0usize;
    let triple_dq = [u16::from(b'"'); 3];
    let triple_sq = [u16::from(b'\''); 3];

    while i < u.len() {
        let ch = u[i];
        if u16_eq_ascii(ch, b'\n') {
            out.newline();
            i += 1;
            if brace_depth == 0 {
                expect_key = true;
            }
            continue;
        }
        if u16_eq_ascii(ch, b' ') || u16_eq_ascii(ch, b'\t') || u16_eq_ascii(ch, b'\r') {
            out.push_units(TokenKind::Plain, &u[i..i + 1]);
            i += 1;
            continue;
        }
        if u16_eq_ascii(ch, b'#') {
            let end = next_unit_or(u, u16::from(b'\n'), i);
            out.push_units(TokenKind::Comment, &u[i..end]);
            i = end;
            continue;
        }
        if expect_key && u16_eq_ascii(ch, b'[') {
            let end = next_unit_or(u, u16::from(b'\n'), i);
            if let Some(close) = last_unit_before(u, u16::from(b']'), end)
                && close > i
            {
                out.push_units(TokenKind::Section, &u[i..close + 1]);
                i = close + 1;
                continue;
            }
        }
        if !expect_key {
            let multiline = if starts_with_units(u, i, &triple_dq) {
                Some(&triple_dq)
            } else if starts_with_units(u, i, &triple_sq) {
                Some(&triple_sq)
            } else {
                None
            };
            if let Some(delim) = multiline {
                let end = match utf16::index_of(u, delim, i + 3) {
                    Some(pos) => pos + 3,
                    None => u.len(),
                };
                out.push_units(TokenKind::String, &u[i..end]);
                i = end;
                continue;
            }
        }
        if u16_eq_ascii(ch, b'"') || u16_eq_ascii(ch, b'\'') {
            let len = read_quoted(u, i, u16_eq_ascii(ch, b'"'));
            out.push_units(
                if expect_key {
                    TokenKind::Key
                } else {
                    TokenKind::String
                },
                &u[i..i + len],
            );
            i += len;
            continue;
        }
        if u16_eq_ascii(ch, b'=') {
            out.push_str(TokenKind::Operator, "=");
            expect_key = false;
            i += 1;
            continue;
        }
        if u16_eq_ascii(ch, b'{') {
            out.push_str(TokenKind::Punct, "{");
            brace_depth += 1;
            expect_key = true;
            i += 1;
            continue;
        }
        if u16_eq_ascii(ch, b'}') {
            out.push_str(TokenKind::Punct, "}");
            brace_depth = brace_depth.saturating_sub(1);
            expect_key = false;
            i += 1;
            continue;
        }
        if u16_eq_ascii(ch, b'[') || u16_eq_ascii(ch, b']') {
            out.push_units(TokenKind::Punct, &u[i..i + 1]);
            i += 1;
            continue;
        }
        if u16_eq_ascii(ch, b',') {
            out.push_str(TokenKind::Operator, ",");
            if brace_depth > 0 {
                expect_key = true;
            }
            i += 1;
            continue;
        }
        if u16_eq_ascii(ch, b'.') {
            out.push_str(TokenKind::Operator, ".");
            i += 1;
            continue;
        }
        if expect_key {
            let len = match_bare_key_len(u, i);
            if len > 0 {
                out.push_units(TokenKind::Key, &u[i..i + len]);
                i += len;
                continue;
            }
            out.push_units(TokenKind::Plain, &u[i..i + 1]);
            i += 1;
            continue;
        }
        let date_len = match_date_len(u, i);
        if date_len > 0 {
            out.push_units(TokenKind::Date, &u[i..i + date_len]);
            i += date_len;
            continue;
        }
        if match_ascii_keyword(u, i, b"true") {
            out.push_str(TokenKind::Boolean, "true");
            i += 4;
            continue;
        }
        if match_ascii_keyword(u, i, b"false") {
            out.push_str(TokenKind::Boolean, "false");
            i += 5;
            continue;
        }
        if is_ascii_digit(ch) || u16_eq_ascii(ch, b'+') || u16_eq_ascii(ch, b'-') {
            let num_len = match_number_len(u, i);
            if num_len > 0 {
                out.push_units(TokenKind::Number, &u[i..i + num_len]);
                i += num_len;
                continue;
            }
        }
        let bare_len = match_bare_key_len(u, i);
        if bare_len > 0 {
            out.push_units(TokenKind::Plain, &u[i..i + bare_len]);
            i += bare_len;
            continue;
        }
        out.push_units(TokenKind::Plain, &u[i..i + 1]);
        i += 1;
    }

    out.into_json()
}

fn tokenize_json(u: &[u16]) -> Vec<Vec<Value>> {
    let mut out = LineBuilder::new();
    let mut i = 0usize;
    let close_block = [u16::from(b'*'), u16::from(b'/')];
    while i < u.len() {
        let ch = u[i];
        if u16_eq_ascii(ch, b'\n') {
            out.newline();
            i += 1;
            continue;
        }
        if u16_eq_ascii(ch, b'"') {
            let len = read_quoted(u, i, true);
            let mut probe = i + len;
            while probe < u.len() && is_js_ws_u16(u[probe]) {
                probe += 1;
            }
            let kind = if probe < u.len() && u16_eq_ascii(u[probe], b':') {
                TokenKind::Key
            } else {
                TokenKind::String
            };
            out.push_units(kind, &u[i..i + len]);
            i += len;
            continue;
        }
        if u16_eq_ascii(ch, b'/')
            && i + 1 < u.len()
            && (u16_eq_ascii(u[i + 1], b'/') || u16_eq_ascii(u[i + 1], b'*'))
        {
            let end = if u16_eq_ascii(u[i + 1], b'/') {
                next_unit_or(u, u16::from(b'\n'), i)
            } else {
                (index_of_units_or_end(u, &close_block, i + 2) + 2).min(u.len())
            };
            out.push_units(TokenKind::Comment, &u[i..end]);
            i = end;
            continue;
        }
        if u16_eq_ascii(ch, b'{')
            || u16_eq_ascii(ch, b'}')
            || u16_eq_ascii(ch, b'[')
            || u16_eq_ascii(ch, b']')
        {
            out.push_units(TokenKind::Punct, &u[i..i + 1]);
            i += 1;
            continue;
        }
        if u16_eq_ascii(ch, b':') || u16_eq_ascii(ch, b',') {
            out.push_units(TokenKind::Operator, &u[i..i + 1]);
            i += 1;
            continue;
        }
        if match_ascii_keyword(u, i, b"true") {
            out.push_str(TokenKind::Boolean, "true");
            i += 4;
            continue;
        }
        if match_ascii_keyword(u, i, b"false") {
            out.push_str(TokenKind::Boolean, "false");
            i += 5;
            continue;
        }
        if match_ascii_keyword(u, i, b"null") {
            out.push_str(TokenKind::Null, "null");
            i += 4;
            continue;
        }
        if is_ascii_digit(ch) || u16_eq_ascii(ch, b'+') || u16_eq_ascii(ch, b'-') {
            let num_len = match_number_len(u, i);
            if num_len > 0 {
                out.push_units(TokenKind::Number, &u[i..i + num_len]);
                i += num_len;
                continue;
            }
        }
        out.push_units(TokenKind::Plain, &u[i..i + 1]);
        i += 1;
    }
    out.into_json()
}

/// Port of `tokenizeCode` (`src/code-highlight.ts:34`).
pub fn tokenize_code(text: String, language: String) -> Result<Vec<Vec<Value>>, String> {
    let units = utf16::units(&text);
    if language == "json" {
        Ok(tokenize_json(&units))
    } else {
        Ok(tokenize_toml(&units))
    }
}
