//! Prebuilt sherpa-onnx cache validator & safe quarantine (`scripts/prepare-sherpa-cache.mjs`).

use anyhow::{Context, Result, bail, ensure};
use chrono::Utc;
use regex::Regex;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

pub const REQUIRED_LIBS: &[&str] = &[
    "sherpa-onnx-c-api",
    "sherpa-onnx-core",
    "kaldi-decoder-core",
    "sherpa-onnx-kaldifst-core",
    "sherpa-onnx-fstfar",
    "sherpa-onnx-fst",
    "kaldi-native-fbank-core",
    "kissfft-float",
    "piper_phonemize",
    "espeak-ng",
    "ucd",
    "onnxruntime",
    "ssentencepiece_core",
];

fn is_reparse_or_symlink(path: &Path) -> Result<bool> {
    let meta = fs::symlink_metadata(path)?;
    if meta.file_type().is_symlink() {
        return Ok(true);
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0400;
        if (meta.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT) != 0 {
            return Ok(true);
        }
    }
    Ok(false)
}

fn create_unique_quarantine_dir(temp_dir: &Path) -> Result<PathBuf> {
    fs::create_dir_all(temp_dir)?;
    let ts = Utc::now().timestamp_millis();
    let pid = std::process::id();
    for seq in 0u32..1000 {
        let cand = temp_dir.join(format!("codex-sherpa-cache-{pid}-{ts}-{seq}"));
        match fs::create_dir(&cand) {
            Ok(()) => return Ok(fs::canonicalize(&cand)?),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e.into()),
        }
    }
    bail!("Failed to allocate unique quarantine directory")
}

fn is_cross_device_error(err: &std::io::Error) -> bool {
    err.kind() == std::io::ErrorKind::CrossesDevices
        || err.raw_os_error() == Some(17) // Windows ERROR_NOT_SAME_DEVICE
        || err.raw_os_error() == Some(18) // POSIX EXDEV
}

fn copy_dir_no_symlinks(src: &Path, dst: &Path) -> Result<()> {
    ensure!(
        !is_reparse_or_symlink(src)?,
        "Refusing to copy symlinked sherpa directory: {}",
        src.display()
    );
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let from = entry.path();
        ensure!(
            !is_reparse_or_symlink(&from)?,
            "Refusing symlink inside sherpa package: {}",
            from.display()
        );
        let to = dst.join(entry.file_name());
        let meta = fs::symlink_metadata(&from)?;
        if meta.is_dir() {
            copy_dir_no_symlinks(&from, &to)?;
        } else {
            fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

fn remove_dir_no_symlinks(dir: &Path) -> Result<()> {
    ensure!(
        !is_reparse_or_symlink(dir)?,
        "Refusing to remove symlinked sherpa directory: {}",
        dir.display()
    );
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let p = entry.path();
        ensure!(
            !is_reparse_or_symlink(&p)?,
            "Refusing symlink inside sherpa package: {}",
            p.display()
        );
        let meta = fs::symlink_metadata(&p)?;
        if meta.is_dir() {
            remove_dir_no_symlinks(&p)?;
        } else {
            fs::remove_file(&p)?;
        }
    }
    fs::remove_dir(dir)?;
    Ok(())
}

pub fn move_to_quarantine(src: &Path, dst: &Path) -> Result<()> {
    ensure!(
        !is_reparse_or_symlink(src)?,
        "Refusing a symlinked sherpa path: {}",
        src.display()
    );
    match fs::rename(src, dst) {
        Ok(()) => Ok(()),
        Err(e) if is_cross_device_error(&e) => {
            let meta = fs::symlink_metadata(src)?;
            if meta.is_dir() {
                copy_dir_no_symlinks(src, dst)?;
                remove_dir_no_symlinks(src)?;
            } else {
                fs::copy(src, dst)?;
                fs::remove_file(src)?;
            }
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}

/// Validates `<target_dir>/sherpa-onnx-prebuilt` and moves incomplete packages to quarantine.
pub fn prepare_sherpa_cache(target_dir: &Path, temp_dir: &Path) -> Result<Vec<String>> {
    let pkg_re = Regex::new(
        r"^sherpa-onnx-v[\d.]+-(win-(?:x64|arm64)-static-MT-Release|linux-(?:x64|aarch64)-static|osx-(?:x64|arm64)-static)-lib$",
    )?;
    if fs::symlink_metadata(target_dir).is_ok() && is_reparse_or_symlink(target_dir)? {
        bail!("Refusing a symlinked sherpa cache");
    }
    let raw_cache = target_dir.join("sherpa-onnx-prebuilt");
    if fs::symlink_metadata(&raw_cache).is_ok() {
        if is_reparse_or_symlink(&raw_cache)? {
            bail!("Refusing a symlinked sherpa cache");
        }
    } else {
        return Ok(Vec::new());
    }
    if !raw_cache.exists() {
        bail!("Refusing a symlinked sherpa cache");
    }
    let target = fs::canonicalize(target_dir)?;
    let cache = target.join("sherpa-onnx-prebuilt");
    if is_reparse_or_symlink(&cache)? {
        bail!("Refusing a symlinked sherpa cache");
    }
    let root = fs::canonicalize(&cache)?;
    let mut moved = Vec::new();
    let mut quarantine: Option<PathBuf> = None;

    for entry in fs::read_dir(&root)? {
        let entry = entry?;
        let name = entry.file_name();
        let Some(name_str) = name.to_str() else {
            continue;
        };
        if !pkg_re.is_match(name_str) {
            continue;
        }
        let extracted = root.join(name_str);
        if is_reparse_or_symlink(&extracted)?
            || fs::canonicalize(&extracted).ok().as_ref() != Some(&extracted)
        {
            bail!(
                "Refusing a symlinked sherpa package: {}",
                extracted.display()
            );
        }
        let meta = fs::symlink_metadata(&extracted)?;
        if !meta.is_dir() {
            continue;
        }
        let windows = name_str.contains("-win-");
        let complete = REQUIRED_LIBS.iter().all(|lib| {
            let file = if windows {
                format!("{lib}.lib")
            } else {
                format!("lib{lib}.a")
            };
            let lib_path = extracted.join("lib").join(file);
            fs::symlink_metadata(&lib_path)
                .map(|m| m.is_file() && !m.file_type().is_symlink() && m.len() > 0)
                .unwrap_or(false)
        });
        if complete {
            continue;
        }

        if quarantine.is_none() {
            let q = create_unique_quarantine_dir(temp_dir)?;
            if q == root || q.starts_with(&root) {
                bail!("Sherpa quarantine must be outside the prebuilt cache");
            }
            quarantine = Some(q);
        }
        let q = quarantine.as_ref().context("quarantine initialized")?;
        if extracted.parent() != Some(root.as_path()) {
            bail!("Sherpa package escaped the cache");
        }
        move_to_quarantine(&extracted, &q.join(name_str))?;

        let mut archive_os = OsString::from(&extracted);
        archive_os.push(".tar.bz2");
        let archive = PathBuf::from(archive_os);
        if fs::symlink_metadata(&archive).is_ok() {
            move_to_quarantine(&archive, &q.join(format!("{name_str}.tar.bz2")))?;
        }

        moved.push(name_str.to_string());
        println!("Quarantined incomplete sherpa package: {name_str}");
    }
    Ok(moved)
}
