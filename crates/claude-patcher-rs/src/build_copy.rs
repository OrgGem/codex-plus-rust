//! Build, localize, Computer-Use patch, and verify Claude Desktop copies (`build-copy.py` + `build-copy.ps1`).

use crate::asar::{
    asar_integrity, ensure_file_node, extract_blob, get_node, read_asar, walk_asar_files,
    write_asar_bytes,
};
use crate::fuses::{check_fuses_unlocked, patch_electron_fuses, root_for};
use anyhow::{Context, Result, bail, ensure};
use regex::{Regex, bytes::Regex as BytesRegex};
use serde_json::{Value, json};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub const LANG_OLD: &str =
    r#"["en-US","de-DE","fr-FR","ko-KR","ja-JP","es-419","es-ES","it-IT","hi-IN","pt-BR","id-ID"]"#;
pub const LANG_NEW: &str = r#"["en-US","de-DE","fr-FR","ko-KR","ja-JP","es-419","es-ES","it-IT","hi-IN","pt-BR","id-ID","zh-Hans"]"#;

const CATALOG_FILES: &[&str] = &[
    "zh-Hans.json",
    "dynamic/zh-Hans.json",
    "zh-Hans.overrides.json",
];

pub fn ion_for(app: &Path) -> PathBuf {
    let root = root_for(app);
    for c in [
        root.join("Contents/Resources/ion-dist"),
        root.join("resources/ion-dist"),
        root.join("ion-dist"),
    ] {
        if c.is_dir() {
            return c;
        }
    }
    root.join("Contents/Resources/ion-dist")
}

fn dir_has_js(dir: &Path) -> bool {
    let Ok(rd) = fs::read_dir(dir) else {
        return false;
    };
    for e in rd.flatten() {
        let p = e.path();
        if p.is_dir() {
            if dir_has_js(&p) {
                return true;
            }
        } else if p.is_file() && p.extension().and_then(|x| x.to_str()) == Some("js") {
            return true;
        }
    }
    false
}

pub fn ion_dir_has_ui(path: &Path) -> bool {
    path.join("index.html").is_file() && dir_has_js(&path.join("assets"))
}

pub fn ion_asar_for(app: &Path) -> Option<PathBuf> {
    let root = root_for(app);
    for c in [
        root.join("Contents/Resources/ion-dist"),
        root.join("resources/ion-dist"),
        root.join("ion-dist"),
    ] {
        if ion_dir_has_ui(&c) {
            return None;
        }
    }
    [
        root.join("Contents/Resources/ion-dist.asar"),
        root.join("resources/ion-dist.asar"),
    ]
    .into_iter()
    .find(|p| p.is_file())
}

pub fn app_asar(root: &Path) -> Option<PathBuf> {
    [
        root.join("Contents/Resources/app.asar"),
        root.join("resources/app.asar"),
    ]
    .into_iter()
    .find(|p| p.is_file())
}

pub fn cu_marker(app: &Path) -> PathBuf {
    let root = root_for(app);
    if root.join("Contents/Resources").is_dir() || !root.join("resources").is_dir() {
        root.join("Contents/Resources/.cu-patched")
    } else {
        root.join("resources/.cu-patched")
    }
}

pub fn catalog_meta_html(html: &str, langs: &[String]) -> Result<String> {
    let joined: Vec<String> = langs.iter().map(|l| format!("{l}:1.1.1")).collect();
    let meta = format!(
        "<meta name=\"i18n-catalogs\" data-i18n-catalogs=\"{}\">",
        joined.join(",")
    );
    let re = Regex::new(r#"<meta\b[^>]*name=["']i18n-catalogs["'][^>]*>"#)?;
    if re.is_match(html) {
        return Ok(re.replace_all(html, meta.as_str()).into_owned());
    }
    if let Some(pos) = html.find("</head>") {
        let mut out = String::with_capacity(html.len() + meta.len());
        out.push_str(&html[..pos]);
        out.push_str(&meta);
        out.push_str(&html[pos..]);
        return Ok(out);
    }
    bail!("本地界面的 HTML 格式已变化");
}

fn replace_bytes(haystack: &[u8], needle: &[u8], replacement: &[u8]) -> Vec<u8> {
    if needle.is_empty() {
        return haystack.to_vec();
    }
    let mut out = Vec::with_capacity(haystack.len());
    let mut i = 0usize;
    while i < haystack.len() {
        if haystack[i..].starts_with(needle) {
            out.extend_from_slice(replacement);
            i += needle.len();
        } else {
            out.push(haystack[i]);
            i += 1;
        }
    }
    out
}

pub fn localize_ion_asar(path: &Path, catalogs_dir: &Path) -> Result<()> {
    let archive = read_asar(path)?;
    let original = archive.header.clone();
    let mut header = archive.header;

    let mut entries = Vec::new();
    walk_asar_files(&header, "", &mut entries);

    let mut replaced: HashMap<String, Vec<u8>> = HashMap::new();
    let mut matches = 0usize;
    for (raw_name, meta) in &entries {
        let name = raw_name.trim_start_matches('/');
        if !(name.starts_with("assets/") && name.ends_with(".js")) {
            continue;
        }
        let blob = extract_blob(&archive.data, archive.payload_start, meta)?;
        if blob
            .windows(LANG_OLD.len())
            .any(|w| w == LANG_OLD.as_bytes())
        {
            replaced.insert(
                name.to_string(),
                replace_bytes(blob, LANG_OLD.as_bytes(), LANG_NEW.as_bytes()),
            );
            matches += 1;
        } else if blob
            .windows(LANG_NEW.len())
            .any(|w| w == LANG_NEW.as_bytes())
        {
            matches += 1;
        }
    }
    ensure!(matches > 0, "本地界面的语言协商格式已变化，旧副本已保留");

    for rel in CATALOG_FILES {
        let bytes = fs::read(catalogs_dir.join(rel))
            .with_context(|| format!("missing catalog file {rel}"))?;
        let key = format!("i18n/{rel}");
        let _ = ensure_file_node(&mut header, &key)?;
        replaced.insert(key, bytes);
    }

    let i18n_files = get_node(&header, "i18n")
        .and_then(|v| v.get("files"))
        .and_then(Value::as_object)
        .context("i18n node missing")?;
    let mut langs: Vec<String> = i18n_files
        .iter()
        .filter_map(|(n, child)| {
            if child.get("files").is_none() && n.ends_with(".json") {
                let stem = &n[..n.len() - 5];
                if !stem.contains('.') {
                    return Some(stem.to_string());
                }
            }
            None
        })
        .collect();
    langs.sort();

    let index_node =
        get_node(&original, "index.html").context("ion-dist.asar 里缺少 index.html")?;
    let html_bytes = extract_blob(&archive.data, archive.payload_start, index_node)?;
    let html = std::str::from_utf8(html_bytes)?;
    replaced.insert(
        "index.html".to_string(),
        catalog_meta_html(html, &langs)?.into_bytes(),
    );

    // Rebuild chunks and header in exact depth-first order matching _walk_asar.
    let mut final_paths = Vec::new();
    walk_asar_files(&header, "", &mut final_paths);

    let mut chunks = Vec::with_capacity(final_paths.len());
    let mut offset = 0usize;
    let mut new_blobs: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    for (raw_name, _) in &final_paths {
        let name = raw_name.trim_start_matches('/').to_string();
        let (blob, recompute_integrity) = if let Some(b) = replaced.get(&name) {
            (b.clone(), true)
        } else {
            let orig_meta = get_node(&original, &name).context("original node missing")?;
            (
                extract_blob(&archive.data, archive.payload_start, orig_meta)?.to_vec(),
                false,
            )
        };
        let node = ensure_file_node(&mut header, &name)?;
        let obj = node.as_object_mut().context("file node must be object")?;
        if recompute_integrity {
            obj.insert("integrity".into(), asar_integrity(&blob));
        }
        obj.insert("size".into(), json!(blob.len()));
        obj.insert("offset".into(), json!(offset.to_string()));
        offset += blob.len();
        new_blobs.insert(name, blob.clone());
        chunks.push(blob);
    }

    let out_bytes = write_asar_bytes(&header, &chunks)?;
    let tmp = path.with_file_name(format!(
        "{}.codex-plus-tmp",
        path.file_name().unwrap_or_default().to_string_lossy()
    ));
    fs::write(&tmp, out_bytes)?;
    fs::rename(&tmp, path)?;
    Ok(())
}

pub fn verify_ion_asar(path: &Path) -> Result<()> {
    let archive = read_asar(path)?;
    ensure!(
        get_node(&archive.header, "i18n/zh-Hans.json").is_some()
            && get_node(&archive.header, "i18n/dynamic/zh-Hans.json").is_some(),
        "中文词表缺失"
    );
    let index = get_node(&archive.header, "index.html").context("中文词表缺失：缺少 index.html")?;
    let html_bytes = extract_blob(&archive.data, archive.payload_start, index)?;
    ensure!(
        html_bytes.windows(8).any(|w| w == b"zh-Hans:"),
        "本地界面未注册中文词表"
    );
    Ok(())
}

pub fn install_ion_dir(ion: &Path, catalogs_dir: &Path) -> Result<()> {
    for rel in CATALOG_FILES {
        let target = ion.join("i18n").join(rel);
        if let Some(p) = target.parent() {
            fs::create_dir_all(p)?;
        }
        fs::copy(catalogs_dir.join(rel), target)?;
    }

    let mut js_files = Vec::new();
    fn collect_js(dir: &Path, out: &mut Vec<PathBuf>) {
        if let Ok(rd) = fs::read_dir(dir) {
            for e in rd.flatten() {
                let p = e.path();
                if p.is_dir() {
                    collect_js(&p, out);
                } else if p.is_file() && p.extension().and_then(|x| x.to_str()) == Some("js") {
                    out.push(p);
                }
            }
        }
    }
    collect_js(&ion.join("assets"), &mut js_files);

    let mut matches = 0usize;
    for p in js_files {
        let text = fs::read_to_string(&p)?;
        if text.contains(LANG_OLD) {
            // Write UTF-8 explicitly without BOM (fixing PowerShell 5.1 Set-Content -Encoding UTF8).
            fs::write(&p, text.replace(LANG_OLD, LANG_NEW).as_bytes())?;
            matches += 1;
        } else if text.contains(LANG_NEW) {
            matches += 1;
        }
    }
    ensure!(matches > 0, "本地界面的语言协商格式已变化，旧副本已保留");

    let mut langs = Vec::new();
    for entry in fs::read_dir(ion.join("i18n"))? {
        let entry = entry?;
        let p = entry.path();
        if p.is_file()
            && p.extension().and_then(|x| x.to_str()) == Some("json")
            && let Some(stem) = p.file_stem().and_then(|x| x.to_str())
            && !stem.contains('.')
        {
            langs.push(stem.to_string());
        }
    }
    langs.sort();

    let index = ion.join("index.html");
    let html = fs::read_to_string(&index)?;
    let updated = catalog_meta_html(&html, &langs)?;
    // Explicitly write raw UTF-8 without BOM.
    fs::write(&index, updated.as_bytes())?;
    Ok(())
}

fn match_paren(data: &[u8], open_idx: usize) -> Option<usize> {
    let mut depth = 0i32;
    let mut quote: Option<u8> = None;
    let mut i = open_idx;
    while i < data.len() {
        let ch = data[i];
        if let Some(q) = quote {
            if ch == b'\\' {
                i += 2;
                continue;
            }
            if ch == q {
                quote = None;
            }
        } else if matches!(ch, b'"' | b'\'' | b'`') {
            quote = Some(ch);
        } else if ch == b'(' {
            depth += 1;
        } else if ch == b')' {
            depth -= 1;
            if depth == 0 {
                return Some(i);
            }
        }
        i += 1;
    }
    None
}

pub fn patch_cu_source(src: &[u8]) -> Result<(Vec<u8>, Vec<String>)> {
    let mut hits = Vec::new();
    if !src.windows(10).any(|w| w == b"1291166712") {
        return Ok((src.to_vec(), hits));
    }
    let chicago_re = BytesRegex::new(r#"\b[A-Za-z_$][\w$]*\(\s*"chicagoEnabled"\s*\)"#)?;
    let mut cur = src.to_vec();
    let chicago_hits = chicago_re.find_iter(&cur).count();
    if chicago_hits > 0 {
        cur = chicago_re.replace_all(&cur, &b"!0"[..]).into_owned();
        hits.push(format!("chicagoEnabled x{chicago_hits}"));
    }

    let const_re = BytesRegex::new(r#"([A-Za-z_$][\w$]*)\s*=\s*"1291166712""#)?;
    if let Some(caps) = const_re.captures(&cur) {
        let const_name = std::str::from_utf8(&caps[1])?;
        let esc_const = regex::escape(const_name);
        for field in ["enabled", "appScopedEnabled"] {
            let pat = BytesRegex::new(&format!(
                r#"\b[A-Za-z_$][\w$]*\(\s*{esc_const}\s*,\s*"{field}"\s*,"#
            ))?;
            let mut count = 0usize;
            for _ in 0..8 {
                let Some(m) = pat.find(&cur) else {
                    break;
                };
                let open_rel = cur[m.start()..].iter().position(|&b| b == b'(');
                let Some(open_rel) = open_rel else {
                    break;
                };
                let Some(end) = match_paren(&cur, m.start() + open_rel) else {
                    break;
                };
                cur.splice(m.start()..=end, b"!0".iter().copied());
                count += 1;
            }
            if count > 0 {
                hits.push(format!("{field} x{count}"));
            }
        }
    }
    Ok((cur, hits))
}

pub fn patch_app_asar(copy: &Path) -> Result<Vec<String>> {
    let asar = app_asar(&root_for(copy)).context("副本缺少 app.asar")?;
    let archive = read_asar(&asar)?;
    let mut header = archive.header.clone();
    let mut raw_entries = Vec::new();
    walk_asar_files(&header, "", &mut raw_entries);

    let packed_paths: Vec<String> = raw_entries
        .into_iter()
        .filter(|(_, meta)| meta.get("unpacked").and_then(Value::as_bool) != Some(true))
        .map(|(p, _)| p)
        .collect();

    let mut changed = Vec::new();
    let mut chunks = Vec::with_capacity(packed_paths.len());
    let mut offset = 0usize;

    for raw_path in &packed_paths {
        let clean = raw_path.trim_start_matches('/');
        let orig_meta = get_node(&archive.header, clean).context("node missing")?;
        let raw_blob = extract_blob(&archive.data, archive.payload_start, orig_meta)?;
        let blob = if clean.ends_with(".js") && raw_blob.len() < 12_000_000 {
            let (patched, hits) = patch_cu_source(raw_blob)?;
            if !hits.is_empty() {
                let base = clean.rsplit('/').next().unwrap_or(clean);
                changed.push(format!("{base} ({})", hits.join(", ")));
                patched
            } else {
                raw_blob.to_vec()
            }
        } else {
            raw_blob.to_vec()
        };

        let node = ensure_file_node(&mut header, clean)?;
        let obj = node.as_object_mut().context("file node object")?;
        obj.remove("integrity");
        obj.insert("size".into(), json!(blob.len()));
        obj.insert("offset".into(), json!(offset.to_string()));
        offset += blob.len();
        chunks.push(blob);
    }

    ensure!(
        !changed.is_empty(),
        "没有找到可改写的 Computer Use 门禁，版本可能已变化"
    );

    let out = write_asar_bytes(&header, &chunks)?;
    fs::write(&asar, out)?;

    let marker = cu_marker(copy);
    if let Some(p) = marker.parent() {
        fs::create_dir_all(p)?;
    }
    fs::write(
        &marker,
        serde_json::to_string_pretty(&json!({ "patched": changed }))?,
    )?;
    Ok(changed)
}

pub fn check_copy(source: &Path, copy: &Path, computer_use: bool) -> Result<()> {
    let packed = ion_asar_for(copy);
    check_fuses_unlocked(copy, computer_use || packed.is_some())?;

    if !cfg!(windows) {
        let verify = Command::new("/usr/bin/codesign")
            .args(["--verify", "--deep", "--strict"])
            .arg(copy)
            .output()?;
        let verify_text = format!(
            "{}{}",
            String::from_utf8_lossy(&verify.stderr),
            String::from_utf8_lossy(&verify.stdout)
        );
        if !verify.status.success() && !verify_text.contains("CSSMERR_TP_NOT_TRUSTED") {
            bail!("副本签名校验失败：{}", verify_text.trim());
        }
    }

    let source_asar = app_asar(&root_for(source));
    let copy_asar = app_asar(&root_for(copy));
    if computer_use && cu_marker(copy).is_file() {
        ensure!(
            copy_asar.is_some_and(|p| p.is_file()),
            "CU 增强副本缺少 app.asar"
        );
    } else if let (Some(sa), Some(ca)) = (source_asar, copy_asar) {
        ensure!(fs::read(sa)? == fs::read(ca)?, "app.asar 已被改动");
    }

    if let Some(p) = packed {
        verify_ion_asar(&p)?;
    } else {
        let ion = ion_for(copy);
        ensure!(
            ion.join("i18n/zh-Hans.json").is_file()
                && ion.join("i18n/dynamic/zh-Hans.json").is_file(),
            "中文词表缺失"
        );
        let html = fs::read_to_string(ion.join("index.html"))?;
        ensure!(html.contains("zh-Hans:"), "本地界面未注册中文词表");
    }
    println!("副本签名、注入开关、中文词表与 app.asar 一致性检查通过");
    Ok(())
}

fn clear_readonly(path: &Path) -> Result<()> {
    let mut perm = fs::metadata(path)?.permissions();
    if perm.readonly() {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = perm.mode();
            perm.set_mode(mode | 0o200);
        }
        #[cfg(not(unix))]
        #[allow(clippy::permissions_set_readonly_false)]
        {
            perm.set_readonly(false);
        }
        fs::set_permissions(path, perm)?;
    }
    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        let meta = fs::symlink_metadata(&from)?;
        if meta.file_type().is_symlink() {
            #[cfg(unix)]
            {
                let target = fs::read_link(&from)?;
                std::os::unix::fs::symlink(target, &to)?;
            }
        } else if meta.is_dir() {
            copy_dir_recursive(&from, &to)?;
        } else {
            fs::copy(&from, &to)?;
            clear_readonly(&to)?;
        }
    }
    Ok(())
}

pub fn build_copy(
    source: &Path,
    copy: &Path,
    catalogs_dir: &Path,
    computer_use: bool,
) -> Result<()> {
    let source_root = root_for(source);
    let copy_root = root_for(copy);
    ensure!(
        source.exists() && source_root.exists(),
        "找不到官方 Claude 客户端: {}",
        source.display()
    );
    let same_root = match (
        fs::canonicalize(&source_root),
        std::path::absolute(&copy_root),
    ) {
        (Ok(s), Ok(c)) => {
            let s_clean = s
                .to_string_lossy()
                .trim_start_matches(r"\\?\")
                .replace('\\', "/")
                .to_lowercase();
            let c_clean = c
                .to_string_lossy()
                .trim_start_matches(r"\\?\")
                .replace('\\', "/")
                .to_lowercase();
            s_clean == c_clean
        }
        _ => false,
    };
    ensure!(
        !copy.exists()
            && fs::symlink_metadata(copy).is_err()
            && !copy_root.exists()
            && fs::symlink_metadata(&copy_root).is_err()
            && !same_root,
        "构建目标必须是新的暂存目录"
    );
    ensure!(
        ion_for(source).join("index.html").is_file() || ion_asar_for(source).is_some(),
        "此版本缺少 ion-dist 本地界面，暂不支持此版本汉化"
    );
    if !cfg!(windows) {
        let status = Command::new("/usr/bin/codesign")
            .args(["--verify", "--deep", "--strict"])
            .arg(source)
            .status()?;
        ensure!(
            status.success(),
            "Official source signature verification failed"
        );
    }

    if let Some(p) = copy_root.parent()
        && !p.as_os_str().is_empty()
    {
        fs::create_dir_all(p)?;
    }
    copy_dir_recursive(&source_root, &copy_root)?;

    let packed = ion_asar_for(copy);
    let _ = patch_electron_fuses(copy, computer_use || packed.is_some())?;
    if let Some(p) = &packed {
        localize_ion_asar(p, catalogs_dir)?;
    } else {
        install_ion_dir(&ion_for(copy), catalogs_dir)?;
    }
    if computer_use {
        let _ = patch_app_asar(copy)?;
    }
    check_copy(source, copy, computer_use)?;
    Ok(())
}
