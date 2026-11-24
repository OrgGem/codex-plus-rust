//! Assemble and split `assets/inject/renderer-inject` fragments with `oxc_parser` syntax verification.

use anyhow::{Context, Result, bail, ensure};
use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_span::SourceType;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FragmentEntry {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub fragments: Vec<FragmentEntry>,
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn validate_js_syntax(source: &str) -> Result<()> {
    let alloc = Allocator::default();
    // Same role as `new Function(assembled)` in `assemble-renderer-inject.mjs`: validate parseable JS syntax inside an IIFE/function body without executing.
    let wrapped = format!("function _codex_plus_validate_() {{\n{source}\n}}");
    let ret = Parser::new(&alloc, &wrapped, SourceType::cjs()).parse();
    if !ret.diagnostics.is_empty() || ret.fatal_error {
        let msg = ret
            .diagnostics
            .first()
            .map_or_else(|| "SyntaxError".to_string(), ToString::to_string);
        bail!("Assembled JS syntax error: {msg}");
    }
    Ok(())
}

fn is_valid_shard_filename(name: &str) -> bool {
    !name.contains('/')
        && !name.contains('\\')
        && !name.contains("..")
        && name.ends_with(".js")
        && name.len() > 3
}

pub fn assemble_renderer_inject(root: &Path, check_only: bool) -> Result<String> {
    let fragment_dir = root.join("assets/inject/renderer-inject");
    let artifact_path = root.join("assets/inject/renderer-inject.js");
    let manifest_path = fragment_dir.join("manifest.json");

    let current_manifest_text =
        fs::read_to_string(&manifest_path).context("failed to read manifest.json")?;
    let mut manifest: Manifest = serde_json::from_str(&current_manifest_text)?;
    ensure!(
        !manifest.fragments.is_empty(),
        "manifest.json has no fragments"
    );

    let mut seen = HashSet::new();
    let mut fragment_names = Vec::with_capacity(manifest.fragments.len());
    for f in &manifest.fragments {
        ensure!(
            is_valid_shard_filename(&f.name),
            "Invalid fragment filename in manifest.json: {}",
            f.name
        );
        ensure!(
            seen.insert(f.name.clone()),
            "Duplicate fragment in manifest.json: {}",
            f.name
        );
        fragment_names.push(f.name.clone());
    }

    let mut disk_names = Vec::new();
    for entry in fs::read_dir(&fragment_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_file()
            && let Some(name) = entry.file_name().to_str()
            && name.ends_with(".js")
        {
            disk_names.push(name.to_string());
        }
    }
    let disk_set: HashSet<String> = disk_names.iter().cloned().collect();
    let mut unlisted: Vec<String> = disk_names
        .into_iter()
        .filter(|n| !seen.contains(n))
        .collect();
    let mut missing: Vec<String> = fragment_names
        .iter()
        .filter(|n| !disk_set.contains(n.as_str()))
        .cloned()
        .collect();
    unlisted.sort();
    missing.sort();
    ensure!(
        unlisted.is_empty() && missing.is_empty(),
        "manifest.json mismatch with directory (unlisted: {:?}, missing: {:?})",
        unlisted,
        missing
    );

    let mut assembled = String::new();
    let mut line_counts: BTreeMap<String, usize> = BTreeMap::new();
    for frag in &manifest.fragments {
        let body = fs::read_to_string(fragment_dir.join(&frag.name))
            .with_context(|| format!("failed to read {}", frag.name))?;
        ensure!(
            body.ends_with('\n'),
            "Fragment {} does not end with newline",
            frag.name
        );
        let lines = body.split('\n').count() - 1;
        line_counts.insert(frag.name.clone(), lines);
        assembled.push_str(&body);
    }

    let mut manifest_changed = false;
    for frag in &mut manifest.fragments {
        if let Some(&lines) = line_counts.get(&frag.name)
            && frag.lines != Some(lines)
        {
            frag.lines = Some(lines);
            manifest_changed = true;
        }
    }
    let manifest_text = format!("{}\n", serde_json::to_string_pretty(&manifest)?);

    ensure!(
        assembled.starts_with("(() => {\n"),
        "Assembled bundle does not start with IIFE"
    );
    ensure!(
        assembled.contains("\n})();\n"),
        "Assembled bundle missing closing `}})();`"
    );
    validate_js_syntax(&assembled)?;

    let hash = sha256_hex(assembled.as_bytes());
    let current = fs::read_to_string(&artifact_path).ok();

    if check_only {
        ensure!(
            !manifest_changed,
            "manifest.json fragment line counts differ from disk"
        );
        let cur = current.context("renderer-inject.js does not exist")?;
        if cur != assembled {
            let cur_hash = sha256_hex(cur.as_bytes());
            bail!("Artifact drift: artifact={cur_hash}, fragments={hash}");
        }
        println!("产物与分片一致 ({hash})");
    } else if current.as_deref() == Some(assembled.as_str()) {
        if manifest_changed {
            fs::write(&manifest_path, manifest_text)?;
            println!("已同步 manifest.json 的分片行数");
        }
        println!("产物已是最新 ({hash})");
    } else {
        fs::write(&artifact_path, &assembled)?;
        if manifest_changed {
            fs::write(&manifest_path, manifest_text)?;
        }
        println!(
            "已写入产物 {} 个分片 ({hash})",
            manifest.fragments.length_hint()
        );
    }
    Ok(hash)
}

trait LenHint {
    fn length_hint(&self) -> usize;
}
impl<T> LenHint for Vec<T> {
    fn length_hint(&self) -> usize {
        self.len()
    }
}

pub fn split_renderer_inject(root: &Path) -> Result<String> {
    let source_path = root.join("assets/inject/renderer-inject.js");
    let out_dir = root.join("assets/inject/renderer-inject");
    let manifest_path = out_dir.join("manifest.json");

    let manifest_source: Manifest = serde_json::from_str(&fs::read_to_string(&manifest_path)?)?;
    ensure!(
        !manifest_source.fragments.is_empty(),
        "manifest.json has no fragments"
    );
    let mut seen = HashSet::new();
    let mut plan = Vec::with_capacity(manifest_source.fragments.len());
    for f in &manifest_source.fragments {
        ensure!(
            is_valid_shard_filename(&f.name),
            "Invalid fragment filename in manifest.json: {}",
            f.name
        );
        ensure!(
            seen.insert(f.name.clone()),
            "Duplicate fragment in manifest.json: {}",
            f.name
        );
        let lines = f.lines.filter(|&n| n > 0).with_context(|| {
            format!("Invalid fragment lines count in manifest.json: {}", f.name)
        })?;
        plan.push((f.name.clone(), f.description.clone(), lines));
    }

    let raw = fs::read_to_string(&source_path)?;
    let mut lines: Vec<&str> = raw.split('\n').collect();
    if lines.last() == Some(&"") {
        lines.pop();
    }
    let expected_lines: usize = plan.iter().map(|(_, _, c)| *c).sum();
    ensure!(
        expected_lines == lines.len(),
        "manifest.json line sum ({expected_lines}) != artifact ({})",
        lines.len()
    );

    let mut fragments = Vec::new();
    let mut offset = 0usize;
    for (name, description, count) in plan {
        let body_lines = &lines[offset..offset + count];
        let body = format!("{}\n", body_lines.join("\n"));
        ensure!(!body_lines.is_empty(), "Fragment {name} is empty");
        fragments.push((name, description, count, body));
        offset += count;
    }

    for (name, _, _, body) in &fragments {
        if name == "99-tail.js" || name == "zz-paste-fix.js" {
            continue;
        }
        let first = body.split('\n').next().unwrap_or("");
        ensure!(
            first.starts_with("  ") || first == "(() => {",
            "Fragment {name} does not start at top-level declaration: {first}"
        );
    }

    let rejoined: String = fragments.iter().map(|(_, _, _, b)| b.as_str()).collect();
    let orig_hash = sha256_hex(raw.as_bytes());
    let re_hash = sha256_hex(rejoined.as_bytes());
    ensure!(
        orig_hash == re_hash,
        "Rejoined hash mismatch: orig={orig_hash}, rejoined={re_hash}"
    );

    fs::create_dir_all(&out_dir)?;
    let mut manifest_entries = Vec::with_capacity(fragments.len());
    for (name, description, line_count, body) in &fragments {
        fs::write(out_dir.join(name), body)?;
        manifest_entries.push(FragmentEntry {
            name: name.clone(),
            description: description.clone(),
            lines: Some(*line_count),
        });
    }
    let out_manifest = format!(
        "{}\n",
        serde_json::to_string_pretty(&Manifest {
            fragments: manifest_entries
        })?
    );
    fs::write(&manifest_path, out_manifest)?;
    println!(
        "已切分 {} 个分片，sha256 一致: {orig_hash}",
        fragments.len()
    );
    Ok(orig_hash)
}
