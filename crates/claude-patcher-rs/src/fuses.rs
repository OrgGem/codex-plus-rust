//! Electron fuse scanner and patcher matching `assets/claude-localization/build-copy.py`.

use anyhow::{Result, bail, ensure};
use std::fs::{self, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

pub const WIRE: &[u8] = b"dL7pKGdnNz796PbbjQWNKmHXBZaB9tsX";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FuseSite {
    pub binary: PathBuf,
    pub flags_offset: usize,
    pub flags: Vec<u8>,
}

pub fn root_for(app: &Path) -> PathBuf {
    let is_exe_path = app
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("exe"));
    if app.is_file() || is_exe_path {
        match app.parent() {
            Some(p) if !p.as_os_str().is_empty() => p.to_path_buf(),
            Some(_) => {
                if let Some(s) = app.to_str()
                    && let Some(idx) = s.rfind('\\')
                    && idx > 0
                {
                    return PathBuf::from(&s[..idx]);
                }
                PathBuf::from(".")
            }
            None => app.to_path_buf(),
        }
    } else {
        app.to_path_buf()
    }
}

fn collect_exe_recursive(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else {
        return;
    };
    for entry in rd.flatten() {
        let p = entry.path();
        let Ok(meta) = fs::symlink_metadata(&p) else {
            continue;
        };
        if meta.file_type().is_symlink() {
            continue;
        }
        if meta.is_dir() {
            collect_exe_recursive(&p, out);
        } else if meta.is_file()
            && p.extension()
                .and_then(|e| e.to_str())
                .is_some_and(|e| e.eq_ignore_ascii_case("exe"))
        {
            out.push(p);
        }
    }
}

pub fn find_fuse_sites_in_bytes(binary: &Path, data: &[u8]) -> Vec<FuseSite> {
    let mut found = Vec::new();
    let mut offset = 0usize;
    while offset + WIRE.len() + 2 <= data.len() {
        let Some(rel) = data[offset..].windows(WIRE.len()).position(|w| w == WIRE) else {
            break;
        };
        let index = offset + rel;
        let start = index + WIRE.len();
        offset = start;
        if start + 2 > data.len() {
            break;
        }
        let version = data[start + 1] as usize;
        let flags_start = start + 2;
        if version == 0 || version > 32 || flags_start + version > data.len() {
            continue;
        }
        let flags = &data[flags_start..flags_start + version];
        if flags.iter().all(|&b| matches!(b, b'0' | b'1' | b'r')) {
            found.push(FuseSite {
                binary: binary.to_path_buf(),
                flags_offset: flags_start,
                flags: flags.to_vec(),
            });
        }
    }
    found
}

pub fn find_fuses(app: &Path) -> Result<Vec<FuseSite>> {
    let root = root_for(app);
    let mut candidates = Vec::new();
    if !cfg!(windows) {
        candidates.push(root.join(
            "Contents/Frameworks/Electron Framework.framework/Versions/A/Electron Framework",
        ));
    }
    candidates.push(root.clone());
    candidates.push(root.join("Claude.exe"));
    candidates.push(root.join("resources/electron.exe"));
    collect_exe_recursive(&root, &mut candidates);

    for bin in candidates {
        if !bin.is_file() {
            continue;
        }
        let mut f = fs::File::open(&bin)?;
        let mut data = Vec::new();
        f.read_to_end(&mut data)?;
        let sites = find_fuse_sites_in_bytes(&bin, &data);
        if !sites.is_empty() {
            return Ok(sites);
        }
    }
    bail!("Electron fuse 格式已变化，无法安全汉化此版本");
}

pub fn patch_electron_fuses(app: &Path, disable_asar_integrity: bool) -> Result<Vec<FuseSite>> {
    let sites = find_fuses(app)?;
    for site in &sites {
        ensure!(
            site.flags.len() > 3 && matches!(site.flags[3], b'0' | b'1'),
            "此版本禁用了 Electron inspect fuse，无法汉化"
        );
        if disable_asar_integrity {
            ensure!(
                site.flags.len() > 4 && matches!(site.flags[4], b'0' | b'1'),
                "此版本缺少 asar 完整性 fuse，无法汉化"
            );
        }
    }
    for site in &sites {
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&site.binary)?;
        f.seek(SeekFrom::Start((site.flags_offset + 3) as u64))?;
        f.write_all(b"1")?;
        if disable_asar_integrity {
            f.seek(SeekFrom::Start((site.flags_offset + 4) as u64))?;
            f.write_all(b"0")?;
        }
    }
    Ok(sites)
}

pub fn check_fuses_unlocked(app: &Path, require_asar_disabled: bool) -> Result<()> {
    let sites = find_fuses(app)?;
    let flags = &sites[0].flags;
    ensure!(
        flags.get(3) == Some(&b'1'),
        "副本未启用启动注入 (EnableNodeCliInspectArguments != 1)"
    );
    if require_asar_disabled {
        ensure!(
            flags.get(4) == Some(&b'0'),
            "副本改写了 asar，但 EnableEmbeddedAsarIntegrityValidation 仍为 1"
        );
    }
    Ok(())
}
