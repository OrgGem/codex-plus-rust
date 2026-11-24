//! Electron app.asar IPC method version reader ported from `ipc-versions.mjs`.

use anyhow::{Context, Result, bail, ensure};
use serde_json::Value;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

const NEEDLE: &str = "\"thread-follower-start-turn\":";

pub fn read_asar_file(asar_path: &Path, inner_path: &str) -> Result<String> {
    let mut f =
        File::open(asar_path).with_context(|| format!("failed to open {}", asar_path.display()))?;
    let file_len = f.metadata()?.len();
    ensure!(file_len >= 16, "asar file truncated (< 16 bytes)");
    let mut head = [0_u8; 16];
    f.read_exact(&mut head)?;
    let header_size = u64::from(u32::from_le_bytes([head[4], head[5], head[6], head[7]]));
    let json_size = u64::from(u32::from_le_bytes([head[12], head[13], head[14], head[15]]));
    let payload_start = 8_u64
        .checked_add(header_size)
        .context("asar header_size overflow")?;
    let json_end = 16_u64
        .checked_add(json_size)
        .context("asar json_size overflow")?;
    ensure!(
        payload_start <= file_len && json_end <= file_len && json_end <= payload_start,
        "asar header exceeds file bounds"
    );
    let json_len = usize::try_from(json_size).context("asar header too large")?;
    let mut json_buf = vec![0_u8; json_len];
    f.read_exact(&mut json_buf)?;
    let mut node: Value = serde_json::from_slice(&json_buf)?;
    for part in inner_path.split('/') {
        let Some(next) = node.get("files").and_then(|files| files.get(part)) else {
            bail!("asar 里没有 {inner_path}");
        };
        node = next.clone();
    }
    if node.get("unpacked").and_then(Value::as_bool) == Some(true) {
        bail!("{inner_path} 在 app.asar.unpacked 里");
    }
    let size_u64 = node
        .get("size")
        .and_then(Value::as_u64)
        .context("missing asar size")?;
    let offset = match node.get("offset") {
        Some(Value::String(s)) => s.parse::<u64>()?,
        Some(Value::Number(n)) => n.as_u64().context("invalid numeric offset")?,
        _ => bail!("missing asar offset"),
    };
    let start = payload_start
        .checked_add(offset)
        .context("asar entry offset overflow")?;
    let end = start
        .checked_add(size_u64)
        .context("asar entry size overflow")?;
    ensure!(end <= file_len, "asar entry exceeds file bounds");
    let size = usize::try_from(size_u64).context("asar entry size too large")?;
    f.seek(SeekFrom::Start(start))?;
    let mut data = vec![0_u8; size];
    f.read_exact(&mut data)?;
    Ok(String::from_utf8(data)?)
}

pub fn extract_ipc_versions(source: &str) -> Result<Value> {
    let bytes = source.as_bytes();
    let mut i = 0;
    while let Some(open) = bytes[i..].iter().position(|&b| b == b'{').map(|p| i + p) {
        let mut j = open + 1;
        while j < bytes.len() && bytes[j] != b'{' && bytes[j] != b'}' {
            j += 1;
        }
        if j < bytes.len() && bytes[j] == b'}' {
            let candidate = &source[open..=j];
            let has_key = candidate.find(NEEDLE).is_some_and(|p| {
                candidate
                    .as_bytes()
                    .get(p + NEEDLE.len())
                    .is_some_and(u8::is_ascii_digit)
            });
            if has_key {
                return Ok(serde_json::from_str::<Value>(candidate)?);
            }
        }
        i = j.max(open + 1);
    }
    bail!("没找到 IPC 版本表（Codex 改了实现）")
}
