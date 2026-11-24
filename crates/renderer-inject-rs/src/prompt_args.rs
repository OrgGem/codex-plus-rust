//! 1:1 port of pure computation logic in `assets/inject/renderer-inject/96-prompt-args.js`.

#![forbid(unsafe_code)]

use fancy_regex::Regex as FancyRegex;
use indexmap::IndexMap;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptPlaceholders {
    pub positional: bool,
    pub named: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptExpandResult {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

fn command_pattern() -> &'static FancyRegex {
    static RE: OnceLock<FancyRegex> = OnceLock::new();
    RE.get_or_init(|| {
        FancyRegex::new(r"^/prompts:([^\s/]+)\s+([\s\S]*\S)\s*$")
            .expect("prompt command regex must compile")
    })
}

fn named_pattern() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^[A-Z][A-Z0-9_]*").expect("named regex must compile"))
}

/// Parses `/prompts:<id> <args>` command lines (requires non-empty trailing args).
pub fn prompt_args_parse_command(input: &str) -> Option<(String, String)> {
    let trimmed = input.trim();
    let caps = command_pattern().captures(trimmed).ok().flatten()?;
    let id = caps.get(1)?.as_str().to_string();
    let args = caps.get(2)?.as_str().to_string();
    Some((id, args))
}

/// Splits argument string by whitespace while respecting `'...'`, `"..."`, and `\"` / `\\` escapes.
pub fn prompt_args_split(text: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    let mut started = false;
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if let Some(q) = quote {
            if ch == q {
                quote = None;
            } else if q == '"' && ch == '\\' {
                if let Some(&next) = chars.peek()
                    && (next == '"' || next == '\\')
                {
                    current.push(next);
                    chars.next();
                } else {
                    current.push(ch);
                }
            } else {
                current.push(ch);
            }
        } else if ch == '"' || ch == '\'' {
            quote = Some(ch);
            started = true;
        } else if ch.is_whitespace() {
            if started {
                args.push(std::mem::take(&mut current));
            }
            current.clear();
            started = false;
        } else {
            current.push(ch);
            started = true;
        }
    }
    if started {
        args.push(current);
    }
    args
}

/// Extracts positional (`$1..$9`, `$ARGUMENTS`) and unique ordered named placeholders (`$NAME`).
pub fn prompt_args_placeholders(content: &str) -> PromptPlaceholders {
    let mut named: Vec<String> = Vec::new();
    let mut positional = false;
    let bytes = content.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] != b'$' {
            let ch_len = content[i..].chars().next().map_or(1, char::len_utf8);
            i += ch_len;
            continue;
        }
        if i + 1 >= bytes.len() {
            break;
        }
        let next_b = bytes[i + 1];
        if next_b == b'$' {
            i += 2;
            continue;
        }
        if (b'1'..=b'9').contains(&next_b) {
            positional = true;
            i += 2;
            continue;
        }
        if let Some(m) = named_pattern().find(&content[i + 1..]) {
            let name = m.as_str();
            if name == "ARGUMENTS" {
                positional = true;
            } else if !named.iter().any(|existing| existing == name) {
                named.push(name.to_string());
            }
            i += 1 + name.len();
        } else {
            i += 1;
        }
    }

    PromptPlaceholders { positional, named }
}

/// Performs single-pass placeholder substitution matching JS `codexPlusPromptArgsSubstitute`.
pub fn prompt_args_substitute(
    content: &str,
    args: Option<&[String]>,
    values: Option<&IndexMap<String, String>>,
) -> String {
    let mut out = String::with_capacity(content.len());
    let bytes = content.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] != b'$' {
            let ch = content[i..].chars().next().unwrap_or_default();
            out.push(ch);
            i += ch.len_utf8();
            continue;
        }
        if i + 1 >= bytes.len() {
            out.push('$');
            break;
        }
        let next_b = bytes[i + 1];
        if next_b == b'$' {
            out.push('$');
            i += 2;
            continue;
        }
        if let Some(arg_slice) = args
            && (b'1'..=b'9').contains(&next_b)
        {
            let idx = (next_b - b'1') as usize;
            if let Some(val) = arg_slice.get(idx) {
                out.push_str(val);
            }
            i += 2;
            continue;
        }
        if let Some(m) = named_pattern().find(&content[i + 1..]) {
            let name = m.as_str();
            if name == "ARGUMENTS"
                && let Some(arg_slice) = args
            {
                out.push_str(&arg_slice.join(" "));
                i += 1 + name.len();
                continue;
            }
            if let Some(val_map) = values
                && let Some(val) = val_map.get(name)
            {
                out.push_str(val);
                i += 1 + name.len();
                continue;
            }
        }
        out.push('$');
        i += 1;
    }

    out
}

/// Expands a custom prompt template against raw argument text following CLI placeholder rules.
pub fn prompt_args_expand(prompt_id: &str, content: &str, arg_text: &str) -> PromptExpandResult {
    let args = prompt_args_split(arg_text);
    let PromptPlaceholders { positional, named } = prompt_args_placeholders(content);
    let pairs: Vec<Option<(String, String)>> = args
        .iter()
        .map(|arg| {
            arg.find('=')
                .filter(|&at| at > 0)
                .map(|at| (arg[..at].to_string(), arg[at + 1..].to_string()))
        })
        .collect();

    let named_mode = !named.is_empty()
        && pairs
            .iter()
            .flatten()
            .any(|(k, _)| named.iter().any(|n| n == k));

    if named_mode {
        let bad: Vec<&str> = args
            .iter()
            .enumerate()
            .filter_map(|(idx, arg)| pairs[idx].is_none().then_some(arg.as_str()))
            .collect();
        if !bad.is_empty() {
            return PromptExpandResult {
                ok: false,
                text: None,
                hint: None,
                error: Some(format!(
                    "/prompts:{prompt_id} 的参数要写成 名字=值（值里有空格就加引号），这些没写对：{}",
                    bad.join("、")
                )),
            };
        }
        let mut values = IndexMap::new();
        for (k, v) in pairs.into_iter().flatten() {
            values.insert(k, v);
        }
        let missing: Vec<&str> = named
            .iter()
            .filter(|name| !values.contains_key(name.as_str()))
            .map(String::as_str)
            .collect();
        if !missing.is_empty() {
            return PromptExpandResult {
                ok: false,
                text: None,
                hint: None,
                error: Some(format!(
                    "/prompts:{prompt_id} 缺少参数：{}（写成 {}=值）",
                    missing.join("、"),
                    missing[0]
                )),
            };
        }
        return PromptExpandResult {
            ok: true,
            text: Some(
                prompt_args_substitute(content, None, Some(&values))
                    .trim()
                    .to_string(),
            ),
            hint: None,
            error: None,
        };
    }

    let hint = if named.is_empty() {
        String::new()
    } else {
        let dollars: Vec<String> = named.iter().map(|n| format!("${n}")).collect();
        format!(
            "/prompts:{prompt_id} 里有 {}，要替换请写成 {}=值",
            dollars.join("、"),
            named[0]
        )
    };

    if positional {
        return PromptExpandResult {
            ok: true,
            text: Some(
                prompt_args_substitute(content, Some(&args), None)
                    .trim()
                    .to_string(),
            ),
            hint: Some(hint),
            error: None,
        };
    }

    let combined = format!("{content} {}", arg_text.trim()).trim().to_string();
    PromptExpandResult {
        ok: true,
        text: Some(combined),
        hint: Some(hint),
        error: None,
    }
}
