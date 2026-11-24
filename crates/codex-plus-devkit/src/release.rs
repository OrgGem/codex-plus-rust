//! One-command release automation (`scripts/release.mjs`).

use crate::release_meta::{
    active_version_files, bump_version_in_text, changelog_section, compare_versions,
    is_valid_version, normalize_version, read_version_from_text, read_versions,
};
use anyhow::{Context, Result, bail, ensure};
use serde_json::Value;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

fn run_git(root: &Path, args: &[&str]) -> Result<String> {
    let out = Command::new("git")
        .args(args)
        .current_dir(root)
        .stdin(Stdio::null())
        .output()
        .with_context(|| format!("failed to execute git {}", args.join(" ")))?;
    ensure!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr).trim()
    );
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn try_cmd(root: &Path, prog: &str, args: &[&str]) -> Result<String, String> {
    let out = Command::new(prog)
        .args(args)
        .current_dir(root)
        .stdin(Stdio::null())
        .output()
        .map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
    } else {
        let msg = String::from_utf8_lossy(&out.stderr).trim().to_string();
        Err(msg)
    }
}

/// Parses NUL-delimited `git status --porcelain=v1 -z` output, handling two-path `R`/`C` entries
/// (`XY new_path\0orig_path\0`) and slicing at byte index 3 before UTF-8 decoding.
pub fn parse_porcelain_z_dirty_paths(raw: &[u8]) -> Vec<String> {
    let mut out = Vec::new();
    let mut fields = raw.split(|&b| b == 0);
    while let Some(entry) = fields.next() {
        if entry.len() < 4 || entry[2] != b' ' {
            continue;
        }
        let x = entry[0];
        let y = entry[1];
        out.push(String::from_utf8_lossy(&entry[3..]).into_owned());
        if (matches!(x, b'R' | b'C') || matches!(y, b'R' | b'C'))
            && let Some(orig) = fields.next()
            && !orig.is_empty()
        {
            out.push(String::from_utf8_lossy(orig).into_owned());
        }
    }
    out
}

pub fn run_release(root: &Path, raw_version: &str, dry_run: bool) -> Result<()> {
    let version = normalize_version(raw_version);
    ensure!(
        is_valid_version(&version),
        "Usage: release <version, e.g. 2.2.2> [--dry-run]"
    );
    let tag = format!("v{version}");

    // 1. Working tree must be on main and clean except CHANGELOG.md.
    let branch = run_git(root, &["rev-parse", "--abbrev-ref", "HEAD"])?;
    ensure!(
        branch == "main",
        "Current branch is {branch}; release only from main"
    );

    let status_raw = Command::new("git")
        .args(["status", "--porcelain", "-z", "--untracked-files=no"])
        .current_dir(root)
        .output()?;
    ensure!(status_raw.status.success(), "git status failed");
    let dirty: Vec<String> = parse_porcelain_z_dirty_paths(&status_raw.stdout)
        .into_iter()
        .filter(|f| f != "CHANGELOG.md")
        .collect();
    ensure!(
        dirty.is_empty(),
        "Uncommitted changes present; commit or revert first:\n  {}",
        dirty.join("\n  ")
    );
    println!("• 工作区干净（CHANGELOG.md 除外）");

    // 2. Fetch and ensure main is not behind origin/main.
    let _ = run_git(root, &["fetch", "origin", "main", "--tags", "--quiet"])?;
    if try_cmd(
        root,
        "git",
        &["merge-base", "--is-ancestor", "origin/main", "HEAD"],
    )
    .is_err()
    {
        bail!("Local main is behind origin/main; run git pull --rebase first");
    }
    println!("• 本地 main 不落后 origin/main");

    // 3. Tag must not exist locally or remotely.
    let tag_ref = format!("refs/tags/{tag}");
    ensure!(
        try_cmd(root, "git", &["rev-parse", "-q", "--verify", &tag_ref]).is_err(),
        "Tag {tag} already exists locally"
    );
    let remote_tag = run_git(root, &["ls-remote", "--tags", "origin", &tag_ref])?;
    ensure!(remote_tag.is_empty(), "Tag {tag} already exists on origin");
    println!("• tag {tag} 尚未使用");

    // 4. Warn or abort on unrelated draft releases via gh if available.
    match try_cmd(
        root,
        "gh",
        &[
            "release",
            "list",
            "--limit",
            "30",
            "--json",
            "tagName,isDraft",
        ],
    ) {
        Ok(json_out) => {
            let releases: Vec<Value> =
                serde_json::from_str(if json_out.is_empty() { "[]" } else { &json_out })?;
            let other_drafts: Vec<String> = releases
                .iter()
                .filter(|r| r.get("isDraft").and_then(Value::as_bool) == Some(true))
                .filter_map(|r| {
                    r.get("tagName")
                        .and_then(Value::as_str)
                        .map(ToString::to_string)
                })
                .filter(|t| t != &tag)
                .collect();
            ensure!(
                other_drafts.is_empty(),
                "Other unpublished draft releases exist: {}",
                other_drafts.join(", ")
            );
            println!("• 没有其他版本的草稿 Release");
        }
        Err(err) => {
            let first = err.lines().next().unwrap_or_default();
            eprintln!("! 查不到 Release 列表（gh 不可用或未登录），跳过草稿检查：{first}");
        }
    }

    // 5. CHANGELOG.md section must exist.
    let changelog_text =
        fs::read_to_string(root.join("CHANGELOG.md")).context("failed to read CHANGELOG.md")?;
    let notes = changelog_section(&version, &changelog_text);
    ensure!(
        !notes.is_empty(),
        "CHANGELOG.md has no \"## {version}\" section"
    );
    println!(
        "• CHANGELOG.md 有 {version} 的说明（{} 行）",
        notes.lines().count()
    );

    // 6. Read, verify uniformity, and bump all 6 version files.
    let current = read_versions(root)?;
    let unreadable: Vec<&str> = current
        .iter()
        .filter_map(|(f, v)| v.is_none().then_some(*f))
        .collect();
    ensure!(
        unreadable.is_empty(),
        "Unable to read version from: {}",
        unreadable.join(", ")
    );
    let distinct: BTreeSet<String> = current.iter().filter_map(|(_, v)| v.clone()).collect();
    ensure!(
        distinct.len() == 1,
        "Mismatched versions across files:\n  {}",
        current
            .iter()
            .map(|(f, v)| format!("{f}: {}", v.as_deref().unwrap_or("")))
            .collect::<Vec<_>>()
            .join("\n  ")
    );
    let from = distinct.into_iter().next().context("empty version set")?;
    ensure!(
        compare_versions(&version, &from) >= 0,
        "Target version {version} is older than current {from}"
    );

    let mut changed_files = Vec::new();
    if from == version {
        println!("• 版本号已经是 {version}，跳过改号");
    } else {
        for &file in active_version_files(root) {
            let path = root.join(file);
            let before = fs::read_to_string(&path)?;
            let after = bump_version_in_text(file, &before, &from, &version)?;
            ensure!(
                read_version_from_text(file, &after).as_deref() == Some(version.as_str()),
                "Failed to bump version in {file}"
            );
            if !dry_run {
                fs::write(&path, after)?;
            }
            changed_files.push(file.to_string());
        }
        println!(
            "• 版本号 {from} → {version}：{} 个文件",
            changed_files.len()
        );
    }

    let changelog_dirty = !run_git(
        root,
        &[
            "status",
            "--porcelain",
            "--untracked-files=no",
            "--",
            "CHANGELOG.md",
        ],
    )?
    .is_empty();
    let mut to_commit = changed_files;
    if changelog_dirty {
        to_commit.push("CHANGELOG.md".to_string());
    }

    if dry_run {
        let preview = if to_commit.is_empty() {
            "（无改动，直接给 HEAD 打 tag）".to_string()
        } else {
            to_commit.join("、")
        };
        println!("\n[dry-run] 将提交：{preview}");
        println!("[dry-run] 将创建并推送 tag {tag}，CI 随后建草稿 Release、构建附件、自动发布");
        println!("\n—— Release 说明预览 ——\n{notes}");
        return Ok(());
    }

    // 7. Commit, tag, atomic push.
    if !to_commit.is_empty() {
        let mut add_args = vec!["add", "--"];
        let refs: Vec<&str> = to_commit.iter().map(String::as_str).collect();
        add_args.extend(refs);
        run_git(root, &add_args)?;
        run_git(root, &["commit", "-m", &format!("release: {tag}")])?;
        println!("• 已提交 release: {tag}");
    }
    run_git(root, &["tag", "-a", &tag, "-m", &format!("Codex++ {tag}")])?;
    let short_sha = run_git(root, &["rev-parse", "--short", "HEAD"])?;
    println!("• 已打 tag {tag} → {short_sha}");
    run_git(
        root,
        &[
            "push",
            "--atomic",
            "origin",
            "HEAD:refs/heads/main",
            &tag_ref,
        ],
    )?;

    let slug = try_cmd(
        root,
        "gh",
        &[
            "repo",
            "view",
            "--json",
            "nameWithOwner",
            "-q",
            ".nameWithOwner",
        ],
    )
    .unwrap_or_else(|_| "BigPizzaV3/CodexPlusPlus".to_string());
    println!("\n✓ 已推送 {tag}。CI 会建草稿 Release → 构建附件 → 附件齐了自动发布（约 10 分钟）：");
    println!("  https://github.com/{slug}/actions/workflows/release-assets.yml");
    Ok(())
}
