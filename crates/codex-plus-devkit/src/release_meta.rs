//! Release version metadata & changelog section utilities.
//!
//! Direct pure-Rust port of `scripts/release-meta.mjs`.

use anyhow::{Context, Result, bail};
use regex::Regex;
use serde_json::Value;
use std::cmp::Ordering;
use std::fs;
use std::path::Path;

pub const VERSION_FILES: &[&str] = &[
    "Cargo.toml",
    "Cargo.lock",
    "apps/codex-plus-manager/package.json",
    "apps/codex-plus-manager/package-lock.json",
    "apps/codex-plus-manager/src-tauri/tauri.conf.json",
    "apps/codex-plus-manager/web-preview/tauri-web-shim.ts",
];

pub fn is_valid_version(v: &str) -> bool {
    let re = Regex::new(r"^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?$").expect("valid semver regex");
    re.is_match(v)
}

pub fn normalize_version(raw: &str) -> String {
    raw.strip_prefix('v')
        .or_else(|| raw.strip_prefix('V'))
        .unwrap_or(raw)
        .to_string()
}

fn is_workspace_lock_block(block: &str) -> bool {
    let has_name = block.lines().any(|line| {
        line.trim_end_matches('\r')
            .starts_with("name = \"codex-plus")
    });
    let has_source = block.contains("\nsource = ") || block.contains("\r\nsource = ");
    has_name && !has_source
}

fn extract_lock_version(block: &str) -> Option<&str> {
    for line in block.lines() {
        let trimmed = line.trim_end_matches('\r');
        if let Some(rest) = trimmed.strip_prefix("version = \"")
            && let Some(ver) = rest.strip_suffix('"')
        {
            return Some(ver);
        }
    }
    None
}

pub fn read_version_from_text(rel: &str, text: &str) -> Option<String> {
    match rel {
        "Cargo.toml" => {
            let re = Regex::new(r#"\[workspace\.package\][^\[]*?\r?\nversion = "([^"]+)""#).ok()?;
            re.captures(text).map(|c| c[1].to_string())
        }
        "Cargo.lock" => {
            let mut found: Option<String> = None;
            for block in text.split("\n[[package]]") {
                if is_workspace_lock_block(block) {
                    let v = extract_lock_version(block)?.to_string();
                    match &found {
                        Some(prev) if prev != &v => return None,
                        None => found = Some(v),
                        _ => {}
                    }
                }
            }
            found
        }
        "apps/codex-plus-manager/package.json"
        | "apps/codex-plus-manager/src-tauri/tauri.conf.json" => {
            let v: Value = serde_json::from_str(text).ok()?;
            v.get("version")?.as_str().map(ToString::to_string)
        }
        "apps/codex-plus-manager/package-lock.json" => {
            let v: Value = serde_json::from_str(text).ok()?;
            let top = v.get("version")?.as_str()?;
            let pkg = v.get("packages")?.get("")?.get("version")?.as_str()?;
            (top == pkg).then(|| top.to_string())
        }
        "apps/codex-plus-manager/web-preview/tauri-web-shim.ts" => {
            let re = Regex::new(r#"current_version:\s*"([^"]+)""#).ok()?;
            re.captures(text).map(|c| c[1].to_string())
        }
        _ => None,
    }
}

pub fn bump_version_in_text(rel: &str, text: &str, from: &str, to: &str) -> Result<String> {
    let out = match rel {
        "Cargo.toml" => {
            let re = Regex::new(r#"(\[workspace\.package\][^\[]*?\r?\nversion = ")[^"]+(")"#)?;
            re.replace(text, |caps: &regex::Captures| {
                format!("{}{}{}", &caps[1], to, &caps[2])
            })
            .into_owned()
        }
        "Cargo.lock" => {
            let sep = if text.contains("\r\n[[package]]\r\n") {
                "\r\n[[package]]\r\n"
            } else {
                "\n[[package]]\n"
            };
            let parts: Vec<String> = text
                .split(sep)
                .map(|block| {
                    if is_workspace_lock_block(block) {
                        block.replacen(
                            &format!("version = \"{from}\""),
                            &format!("version = \"{to}\""),
                            1,
                        )
                    } else {
                        block.to_string()
                    }
                })
                .collect();
            parts.join(sep)
        }
        "apps/codex-plus-manager/package.json"
        | "apps/codex-plus-manager/src-tauri/tauri.conf.json" => text.replacen(
            &format!("\"version\": \"{from}\""),
            &format!("\"version\": \"{to}\""),
            1,
        ),
        "apps/codex-plus-manager/package-lock.json" => {
            let esc = regex::escape(from);
            let re1 = Regex::new(&format!(r#"(?m)^(  "version": )"{esc}""#))?;
            let step1 = re1.replace(text, |c: &regex::Captures| format!("{}\"{to}\"", &c[1]));
            let re2 = Regex::new(&format!(
                r#"("": \{{\r?\n\s+"name": "[^"]+",\r?\n\s+"version": )"{esc}""#
            ))?;
            re2.replace(&step1, |c: &regex::Captures| format!("{}\"{to}\"", &c[1]))
                .into_owned()
        }
        "apps/codex-plus-manager/web-preview/tauri-web-shim.ts" => {
            text.replace(&format!("\"{from}\""), &format!("\"{to}\""))
        }
        other => bail!("Unknown version file: {other}"),
    };
    Ok(out)
}

pub fn read_versions(root: &Path) -> Result<Vec<(&'static str, Option<String>)>> {
    let mut out = Vec::with_capacity(VERSION_FILES.len());
    for &rel in VERSION_FILES {
        let text =
            fs::read_to_string(root.join(rel)).with_context(|| format!("failed to read {rel}"))?;
        let ver = read_version_from_text(rel, &text);
        out.push((rel, ver));
    }
    Ok(out)
}

pub fn changelog_section(version: &str, text: &str) -> String {
    let lines: Vec<&str> = text.split('\n').map(|l| l.trim_end_matches('\r')).collect();
    let heading_re = Regex::new(&format!(r"^## \[?v?{}\]?(?:\s|$)", regex::escape(version)))
        .expect("valid changelog heading regex");
    let Some(start) = lines.iter().position(|line| heading_re.is_match(line)) else {
        return String::new();
    };
    let end = lines
        .iter()
        .enumerate()
        .skip(start + 1)
        .find(|(_, line)| line.starts_with("## "))
        .map_or(lines.len(), |(idx, _)| idx);
    lines[start + 1..end].join("\n").trim().to_string()
}

pub fn compare_versions(a: &str, b: &str) -> i32 {
    fn parse(v: &str) -> ([i64; 3], &str) {
        let s = normalize_version_slice(v);
        let (core, pre) = match s.split_once('-') {
            Some((c, p)) => (c, p),
            None => (s, ""),
        };
        let mut parts = [0i64; 3];
        for (i, seg) in core.split('.').take(3).enumerate() {
            parts[i] = seg.parse::<i64>().unwrap_or(0);
        }
        (parts, pre)
    }
    fn normalize_version_slice(v: &str) -> &str {
        v.strip_prefix('v')
            .or_else(|| v.strip_prefix('V'))
            .unwrap_or(v)
    }
    let (ap, apre) = parse(a);
    let (bp, bpre) = parse(b);
    for i in 0..3 {
        if ap[i] != bp[i] {
            return if ap[i] > bp[i] { 1 } else { -1 };
        }
    }
    match (apre.is_empty(), bpre.is_empty()) {
        (true, true) => 0,
        (true, false) => 1,
        (false, true) => -1,
        (false, false) => match apre.encode_utf16().cmp(bpre.encode_utf16()) {
            Ordering::Less => -1,
            Ordering::Greater => 1,
            Ordering::Equal => 0,
        },
    }
}

pub fn run_check(root: &Path, raw_version: &str) -> Result<i32> {
    let version = normalize_version(raw_version);
    if !is_valid_version(&version) {
        eprintln!("Usage: release-meta check <version>");
        return Ok(2);
    }
    let versions = read_versions(root)?;
    let mut mismatches = 0usize;
    for (file, found) in versions {
        if found.as_deref() != Some(version.as_str()) {
            eprintln!(
                "Version mismatch: {file} is {}, expected {version}",
                found.as_deref().unwrap_or("(unreadable)")
            );
            mismatches += 1;
        }
    }
    if mismatches > 0 {
        return Ok(1);
    }
    eprintln!("Versions match: {version}");
    Ok(0)
}

pub fn run_notes(root: &Path, raw_version: &str) -> Result<i32> {
    let version = normalize_version(raw_version);
    if !is_valid_version(&version) {
        eprintln!("Usage: release-meta notes <version>");
        return Ok(2);
    }
    let text =
        fs::read_to_string(root.join("CHANGELOG.md")).context("failed to read CHANGELOG.md")?;
    let notes = changelog_section(&version, &text);
    if notes.is_empty() {
        eprintln!("CHANGELOG.md has no section for {version}");
        return Ok(1);
    }
    println!("{notes}");
    Ok(0)
}
