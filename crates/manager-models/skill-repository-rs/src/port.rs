//! The Rust port of `src/skill-repository.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::is_git_hub_repository_homepage;
use serde_json::{Value, json};
use std::borrow::Cow;
use url::Url;

fn js_trim_ecma(s: &str) -> &str {
    s.trim_matches(|c: char| c != '\u{0085}' && (c.is_whitespace() || c == '\u{FEFF}'))
}

fn decode_uri_component(s: &str) -> Option<String> {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            let h1 = char::from(*bytes.get(i + 1)?).to_digit(16)?;
            let h2 = char::from(*bytes.get(i + 2)?).to_digit(16)?;
            out.push(u8::try_from((h1 << 4) | h2).ok()?);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8(out).ok()
}

/// Port of `parseSkillRepository` (`src/skill-repository.ts:5`).
pub fn parse_skill_repository(value: String) -> Result<Option<Value>, String> {
    let trimmed = js_trim_ecma(&value);
    let input = trimmed.strip_prefix("git+").unwrap_or(trimmed);
    let candidate = if input.contains("://") {
        Cow::Borrowed(input)
    } else {
        Cow::Owned(format!("https://github.com/{input}"))
    };
    let Ok(url) = Url::parse(&candidate) else {
        return Ok(None);
    };
    if !matches!(url.scheme(), "https" | "http")
        || !matches!(url.host_str(), Some("github.com" | "www.github.com"))
        || !url.username().is_empty()
        || url.password().is_some_and(|p| !p.is_empty())
        || url.port().is_some()
    {
        return Ok(None);
    }
    let path = url.path().trim_end_matches('/');
    let Some(segments) = path.strip_prefix('/') else {
        return Ok(None);
    };
    let Some(parts) = segments
        .split('/')
        .map(decode_uri_component)
        .collect::<Option<Vec<String>>>()
    else {
        return Ok(None);
    };
    if parts.len() < 2 {
        return Ok(None);
    }
    let owner = &parts[0];
    let raw_name = &parts[1];
    let name = raw_name.strip_suffix(".git").unwrap_or(raw_name);
    if owner.is_empty()
        || name.is_empty()
        || !is_git_hub_repository_homepage(&format!("https://github.com/{owner}/{name}"))
    {
        return Ok(None);
    }
    if parts.len() > 2 && (parts[2] != "tree" || parts.get(3).is_none_or(String::is_empty)) {
        return Ok(None);
    }
    let branch = parts
        .get(3)
        .filter(|b| !b.is_empty())
        .map_or("main", String::as_str);
    let subdir = if parts.len() > 4 { &parts[4..] } else { &[] };
    if subdir
        .iter()
        .any(|part| part.is_empty() || part == "." || part == ".." || part.contains('\\'))
    {
        return Ok(None);
    }
    Ok(Some(json!({
        "owner": owner,
        "name": name,
        "branch": branch,
        "subdir": subdir.join("/"),
        "enabled": true,
    })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_and_rejects_invalid_skill_repositories() {
        for input in [
            "openai/skills",
            "https://github.com/openai/skills.git",
            "https://www.github.com/openai/skills/?tab=readme#top",
            "  git+https://github.com/openai/skills.git  ",
        ] {
            assert_eq!(
                parse_skill_repository(input.into()),
                Ok(Some(json!({
                    "owner": "openai",
                    "name": "skills",
                    "branch": "main",
                    "subdir": "",
                    "enabled": true,
                }))),
                "{input}"
            );
        }
        assert_eq!(
            parse_skill_repository(
                "https://github.com/openai/skills/tree/dev/skills/.curated".into()
            ),
            Ok(Some(json!({
                "owner": "openai",
                "name": "skills",
                "branch": "dev",
                "subdir": "skills/.curated",
                "enabled": true,
            })))
        );
        for bad in [
            "https://evil.com/acme/kit",
            "https://github.com@evil.com/acme/kit",
            "https://github.com/acme/kit/issues",
            "https://github.com/acme/kit/tree",
            "acme",
            "https://github.com/topics/skills",
            "https://github.com/openai/skills/tree/main/%ZZ",
            "https://github.com/openai/skills/tree/main/a%5Cb",
        ] {
            assert_eq!(parse_skill_repository(bad.into()), Ok(None), "{bad}");
        }
    }
}
