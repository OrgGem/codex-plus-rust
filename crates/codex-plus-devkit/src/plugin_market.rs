//! On-demand plugin market index builder (`scripts/build-plugin-market-index.mjs`).

use anyhow::{Context, Result, bail, ensure};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

fn run_git_raw(root: &Path, args: &[&str]) -> Result<String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .stdin(Stdio::null())
        .output()
        .with_context(|| format!("failed to invoke git {:?}", args))?;
    ensure!(
        out.status.success(),
        "git {:?} failed: {}",
        args,
        String::from_utf8_lossy(&out.stderr).trim()
    );
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

fn slice_utf16(s: &str, max_units: usize) -> String {
    let mut count = 0usize;
    let mut end = s.len();
    for (idx, ch) in s.char_indices() {
        let next = count + ch.len_utf16();
        if next > max_units {
            end = idx;
            break;
        }
        count = next;
    }
    s[..end].to_string()
}

fn text_opt(v: Option<&Value>, max: usize) -> String {
    match v.and_then(Value::as_str) {
        Some(s) => slice_utf16(s, max),
        None => String::new(),
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginEntry {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub tags: Vec<String>,
    pub license: String,
    pub path: String,
    pub tree: String,
    pub bytes: u64,
    pub skills: u64,
    pub requires_auth: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketIndex {
    pub schema_version: u64,
    pub repository: String,
    pub revision: String,
    pub updated_at: Value,
    pub plugins: Vec<PluginEntry>,
}

fn locale_cmp_fallback(a: &str, b: &str) -> std::cmp::Ordering {
    let al = a.to_lowercase();
    let bl = b.to_lowercase();
    al.cmp(&bl).then_with(|| a.cmp(b))
}

pub fn build_plugin_market_index(cache_repo: &Path, source: &str, output: &Path) -> Result<String> {
    if source != "public" && source != "full" {
        bail!("Invalid source mode '{source}': expected 'public' or 'full'");
    }
    let root = fs::canonicalize(cache_repo)?;
    let repository = if source == "full" {
        "BigPizzaV3/CodexPlusPlusFullPluginCache"
    } else {
        "BigPizzaV3/CodexPlusPlusPluginCache"
    };

    let revision = run_git_raw(&root, &["rev-parse", "HEAD"])?
        .trim()
        .to_string();

    let mut trees: HashMap<String, String> = HashMap::new();
    let tree_out = run_git_raw(&root, &["ls-tree", "-r", "-d", &revision])?;
    for line in tree_out.trim().lines() {
        let Some((meta, name)) = line.split_once('\t') else {
            continue;
        };
        if let Some(hash) = meta.split(' ').nth(2) {
            trees.insert(name.to_string(), hash.to_string());
        }
    }

    let mut sizes: HashMap<String, u64> = HashMap::new();
    let mut skill_counts: HashMap<String, u64> = HashMap::new();
    let mut auth_packages: HashSet<String> = HashSet::new();
    let mut tracked_files: HashSet<String> = HashSet::new();

    let files_out = run_git_raw(&root, &["ls-tree", "-r", "-l", &revision])?;
    let prefix = if source == "full" {
        "markets/"
    } else {
        "plugins/"
    };
    let depth = if source == "full" { 4 } else { 2 };

    for line in files_out.trim().lines() {
        let Some((meta, file)) = line.split_once('\t') else {
            continue;
        };
        tracked_files.insert(file.to_string());
        let parts: Vec<&str> = file.split('/').collect();
        let key = parts
            .iter()
            .take(depth)
            .copied()
            .collect::<Vec<_>>()
            .join("/");
        if !key.starts_with(prefix) {
            continue;
        }
        if let Some(sz_str) = meta.split_whitespace().nth(3)
            && let Ok(bytes) = sz_str.parse::<u64>()
        {
            *sizes.entry(key.clone()).or_default() += bytes;
        }
        if file.ends_with("/SKILL.md") {
            *skill_counts.entry(key.clone()).or_default() += 1;
        }
        if matches!(parts.last(), Some(&".app.json" | &".mcp.json")) {
            auth_packages.insert(key);
        }
    }

    let catalog_file = if source == "full" {
        "directory.json"
    } else {
        "catalog.json"
    };
    let catalog: Value = serde_json::from_str(&fs::read_to_string(root.join(catalog_file))?)?;
    let raw_entries: Vec<&Value> = if source == "full" {
        catalog
            .get("entries")
            .and_then(Value::as_array)
            .context("directory.json missing entries")?
            .iter()
            .filter(|it| it.get("marketState").and_then(Value::as_str) == Some("ready"))
            .collect()
    } else {
        catalog
            .get("plugins")
            .and_then(Value::as_array)
            .context("catalog.json missing plugins")?
            .iter()
            .collect()
    };

    let mut plugins = Vec::with_capacity(raw_entries.len());
    for entry in raw_entries {
        let name = if source == "full" {
            entry.get("manifestName").and_then(Value::as_str)
        } else {
            entry.get("name").and_then(Value::as_str)
        }
        .context("catalog entry missing name")?
        .to_string();

        let id = entry
            .get("id")
            .and_then(Value::as_str)
            .or_else(|| entry.get("remotePluginId").and_then(Value::as_str))
            .unwrap_or(&name)
            .to_string();

        let plugin_path = if source == "full" {
            let mkt_root = entry
                .get("marketplaceRoot")
                .and_then(Value::as_str)
                .context("full entry missing marketplaceRoot")?;
            let mkt_base = Path::new(mkt_root)
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            let archive = entry
                .get("archiveFile")
                .and_then(Value::as_str)
                .context("full entry missing archiveFile")?;
            format!("markets/{mkt_base}/plugins/{}", slice_utf16(archive, 16))
        } else {
            format!("plugins/{name}")
        };

        let Some(tree) = trees.get(&plugin_path).cloned() else {
            bail!("Plugin directory missing in git commit: {plugin_path}");
        };

        let mut manifest: Option<Value> = None;
        for rel in [
            ".codex-plugin/plugin.json",
            ".claude-plugin/plugin.json",
            "plugin.json",
        ] {
            let spec = format!("{plugin_path}/{rel}");
            if !tracked_files.contains(&spec) {
                continue;
            }
            if let Ok(raw) = run_git_raw(&root, &["show", &format!("{revision}:{spec}")])
                && let Ok(parsed) = serde_json::from_str::<Value>(&raw)
            {
                manifest = Some(parsed);
                break;
            }
        }
        let manifest = manifest
            .filter(|m| m.get("name").and_then(Value::as_str) == Some(&name))
            .with_context(|| format!("Plugin manifest mismatch at {plugin_path}"))?;

        let display_val = manifest
            .get("interface")
            .and_then(|i| i.get("displayName"))
            .or_else(|| manifest.get("displayName"));
        let display_name = if let Some(Value::String(_)) = display_val {
            text_opt(display_val, 200)
        } else if display_val.is_some_and(|v| !v.is_null()) {
            String::new()
        } else {
            slice_utf16(&name, 200)
        };

        let version_val = manifest
            .get("version")
            .or_else(|| entry.get("manifestVersion"))
            .or_else(|| entry.get("version"))
            .or_else(|| entry.get("directoryVersion"));
        let version = match version_val {
            Some(Value::String(s)) => slice_utf16(s, 128),
            Some(v) if !v.is_null() => String::new(),
            _ => "0.0.0".to_string(),
        };

        let author_val = manifest.get("author").or_else(|| entry.get("author"));
        let author = match author_val {
            Some(Value::String(s)) => slice_utf16(s, 200),
            Some(Value::Object(o)) => text_opt(o.get("name"), 200),
            _ => String::new(),
        };

        let tags: Vec<String> = manifest
            .get("keywords")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(Value::as_str)
                    .take(20)
                    .map(ToString::to_string)
                    .collect()
            })
            .unwrap_or_default();

        let license = match manifest.get("license") {
            Some(Value::String(s)) => slice_utf16(s, 300),
            _ => text_opt(entry.get("declaredLicense"), 300),
        };

        plugins.push(PluginEntry {
            id,
            name,
            display_name,
            description: text_opt(manifest.get("description"), 2000),
            version,
            author,
            tags,
            license,
            bytes: sizes.get(&plugin_path).copied().unwrap_or(0),
            skills: skill_counts.get(&plugin_path).copied().unwrap_or(0),
            requires_auth: auth_packages.contains(&plugin_path),
            path: plugin_path,
            tree,
        });
    }

    plugins.sort_by(|a, b| {
        locale_cmp_fallback(&a.display_name, &b.display_name)
            .then_with(|| locale_cmp_fallback(&a.id, &b.id))
    });

    let index = MarketIndex {
        schema_version: 1,
        repository: repository.to_string(),
        revision: revision.clone(),
        updated_at: catalog.get("snapshotDate").cloned().unwrap_or(Value::Null),
        plugins,
    };
    let contents = format!("{}\n", serde_json::to_string(&index)?);
    if let Some(parent) = output.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }
    fs::write(output, &contents)?;
    let summary = serde_json::json!({
        "source": source,
        "repository": repository,
        "revision": revision,
        "plugins": index.plugins.len(),
        "indexBytes": contents.len(),
        "output": output.to_string_lossy(),
    });
    println!("{summary}");
    Ok(contents)
}
