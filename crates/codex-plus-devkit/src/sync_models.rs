//! Sync official model metadata into `assets/codex-models.json` (`scripts/sync_official_models.py`).

use anyhow::{Context, Result, ensure};
use serde_json::{Map, Value, json};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::time::Duration;

pub const DEFAULT_SOURCE_URL: &str =
    "https://raw.githubusercontent.com/openai/codex/main/codex-rs/models-manager/models.json";

const SKIP_SLUGS: &[&str] = &[
    "gpt-reserve",
    "codex-auto-review",
    "gpt-daybreak-blue-latest",
    "gpt-daybreak-red-latest",
];

const LEGACY_SLUGS: &[&str] = &["gpt-5.4", "gpt-5.4-mini", "gpt-5.3-codex", "gpt-5.2"];

fn fetch_models_json(source: &str) -> Result<Value> {
    if source.starts_with("http://") || source.starts_with("https://") {
        let resp = ureq::get(source).timeout(Duration::from_secs(60)).call()?;
        let body = resp.into_string()?;
        return Ok(serde_json::from_str(&body)?);
    }
    let text = fs::read_to_string(source).with_context(|| format!("failed to read {source}"))?;
    Ok(serde_json::from_str(&text)?)
}

/// Synchronizes missing non-hidden upstream models into `asset_path`.
/// Returns `Ok(true)` if up-to-date (or applied when `!check_only`), `Ok(false)` when `--check` detects missing additions.
pub fn sync_official_models(asset_path: &Path, source: &str, check_only: bool) -> Result<bool> {
    let official_val = fetch_models_json(source)?;
    let official_arr = official_val
        .get("models")
        .and_then(Value::as_array)
        .context("Official models.json missing models array")?;

    let mut ordered_official: Vec<(&str, &Map<String, Value>)> = Vec::new();
    let mut seen_slugs = HashSet::new();
    for item in official_arr {
        if let Some(obj) = item.as_object()
            && let Some(slug) = obj.get("slug").and_then(Value::as_str)
            && !slug.is_empty()
            && seen_slugs.insert(slug.to_string())
        {
            ordered_official.push((slug, obj));
        }
    }

    let asset_val: Value = serde_json::from_str(
        &fs::read_to_string(asset_path)
            .with_context(|| format!("failed to read {}", asset_path.display()))?,
    )?;
    let mut asset_models = asset_val
        .get("models")
        .and_then(Value::as_array)
        .cloned()
        .context("Asset JSON missing models array")?;

    let existing: HashSet<String> = asset_models
        .iter()
        .filter_map(|m| {
            m.get("slug")
                .and_then(Value::as_str)
                .map(ToString::to_string)
        })
        .collect();
    let skip: HashSet<&str> = SKIP_SLUGS.iter().copied().collect();

    let additions: Vec<(&str, &Map<String, Value>)> = ordered_official
        .into_iter()
        .filter(|(slug, _)| !existing.contains(*slug) && !skip.contains(*slug))
        .collect();

    if additions.is_empty() {
        println!("静态资产已覆盖官方仓库全部可见模型，无需补入。");
        return Ok(true);
    }

    let count = additions.len();
    for (slug, obj) in additions {
        let mut entry = obj.clone();
        entry.shift_remove("comp_hash");
        let legacy_first = asset_models
            .iter()
            .position(|m| {
                m.get("slug")
                    .and_then(Value::as_str)
                    .is_some_and(|s| LEGACY_SLUGS.contains(&s))
            })
            .unwrap_or(asset_models.len());
        let win = entry
            .get("max_context_window")
            .map_or_else(|| "None".to_string(), ToString::to_string);
        asset_models.insert(legacy_first, Value::Object(entry));
        println!("补入 {slug} (max_context_window={win})");
    }

    if check_only {
        println!("--check 模式：发现 {count} 条可补入，未写入。");
        return Ok(false);
    }

    let out = format!(
        "{}\n",
        serde_json::to_string_pretty(&json!({ "models": asset_models }))?
    );
    ensure!(!out.is_empty(), "serialized output is empty");
    fs::write(asset_path, out)?;
    println!(
        "已写入 {}（现 {} 条）。",
        asset_path.display(),
        asset_models.len()
    );
    Ok(true)
}
