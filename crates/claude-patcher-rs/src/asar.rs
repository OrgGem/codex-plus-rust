//! Pure-Rust Chromium Pickle (`<IIII`) ASAR reader and writer with 4 MiB SHA-256 block integrity.

use anyhow::{Context, Result, bail, ensure};
use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub const ASAR_BLOCK_SIZE: usize = 4 * 1024 * 1024;

#[derive(Clone, Debug)]
pub struct AsarArchive {
    pub header: Value,
    pub data: Vec<u8>,
    pub payload_start: usize,
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub fn asar_integrity(blob: &[u8]) -> Value {
    let blocks: Vec<Value> = blob
        .chunks(ASAR_BLOCK_SIZE)
        .map(|chunk| Value::String(sha256_hex(chunk)))
        .collect();
    json!({
        "algorithm": "SHA256",
        "hash": sha256_hex(blob),
        "blockSize": ASAR_BLOCK_SIZE,
        "blocks": blocks,
    })
}

pub fn read_asar(path: &Path) -> Result<AsarArchive> {
    let data =
        fs::read(path).with_context(|| format!("failed to read ASAR: {}", path.display()))?;
    ensure!(data.len() >= 16, "ASAR file truncated (< 16 bytes)");
    let hdr_size = u32::from_le_bytes(data[4..8].try_into()?) as usize;
    let json_str_size = u32::from_le_bytes(data[12..16].try_into()?) as usize;
    ensure!(
        data.len() >= 16 + json_str_size && data.len() >= 8 + hdr_size,
        "ASAR header out of bounds"
    );
    let header_str = std::str::from_utf8(&data[16..16 + json_str_size])?;
    let header: Value = serde_json::from_str(header_str)?;
    Ok(AsarArchive {
        header,
        data,
        payload_start: 8 + hdr_size,
    })
}

pub fn get_node<'a>(header: &'a Value, rel: &str) -> Option<&'a Value> {
    let mut node = header;
    for part in rel.trim_start_matches('/').split('/') {
        node = node.get("files")?.get(part)?;
    }
    Some(node)
}

pub fn ensure_file_node<'a>(header: &'a mut Value, rel: &str) -> Result<&'a mut Value> {
    let mut node = header;
    for part in rel.trim_start_matches('/').split('/') {
        let obj = node
            .as_object_mut()
            .context("expected directory object in ASAR header")?;
        if !obj.contains_key("files") {
            obj.insert("files".to_string(), Value::Object(Map::new()));
        }
        let files = obj
            .get_mut("files")
            .and_then(Value::as_object_mut)
            .context("ASAR files must be object")?;
        if !files.contains_key(part) {
            files.insert(part.to_string(), Value::Object(Map::new()));
        }
        node = files.get_mut(part).context("missing inserted node")?;
    }
    Ok(node)
}

pub fn walk_asar_files(node: &Value, prefix: &str, out: &mut Vec<(String, Value)>) {
    let Some(files) = node.get("files").and_then(Value::as_object) else {
        return;
    };
    for (name, child) in files {
        let path = format!("{prefix}/{name}");
        if child.get("files").is_some() {
            walk_asar_files(child, &path, out);
        } else {
            out.push((path, child.clone()));
        }
    }
}

pub fn extract_blob<'a>(data: &'a [u8], payload_start: usize, meta: &Value) -> Result<&'a [u8]> {
    if meta.get("unpacked").and_then(Value::as_bool) == Some(true) {
        bail!("暂不支持带 unpacked 文件的 ion-dist.asar");
    }
    let off = meta
        .get("offset")
        .and_then(Value::as_str)
        .and_then(|s| s.parse::<usize>().ok())
        .or_else(|| {
            meta.get("offset")
                .and_then(Value::as_u64)
                .map(|u| u as usize)
        })
        .context("ASAR entry missing offset")?;
    let size = meta
        .get("size")
        .and_then(Value::as_u64)
        .context("ASAR entry missing size")? as usize;
    let abs = payload_start.checked_add(off).context("offset overflow")?;
    let end = abs.checked_add(size).context("size overflow")?;
    ensure!(end <= data.len(), "ASAR blob exceeds archive bounds");
    Ok(&data[abs..end])
}

/// Serializes `header` using exact Chromium Pickle `<IIII` framing and appends payload chunks.
pub fn write_asar_bytes(header: &Value, chunks: &[Vec<u8>]) -> Result<Vec<u8>> {
    let text = serde_json::to_vec(header)?;
    let pad = (4 - (text.len() % 4)) % 4;
    let json_size = text.len() + pad + 4;
    let hdr_size = json_size + 4;
    let mut out =
        Vec::with_capacity(16 + text.len() + pad + chunks.iter().map(Vec::len).sum::<usize>());
    out.extend_from_slice(&4u32.to_le_bytes());
    out.extend_from_slice(&(hdr_size as u32).to_le_bytes());
    out.extend_from_slice(&(json_size as u32).to_le_bytes());
    out.extend_from_slice(&(text.len() as u32).to_le_bytes());
    out.extend_from_slice(&text);
    out.resize(out.len() + pad, 0u8);
    for c in chunks {
        out.extend_from_slice(c);
    }
    Ok(out)
}

/// Builds a fresh ASAR archive bytes buffer from an ordered `(path, bytes)` map (test/pack helper).
pub fn pack_files(entries: &BTreeMap<String, Vec<u8>>) -> Result<Vec<u8>> {
    let mut header = json!({ "files": {} });
    let mut chunks = Vec::new();
    let mut offset = 0usize;
    for (rel, bytes) in entries {
        let clean = rel.trim_start_matches('/');
        let node = ensure_file_node(&mut header, clean)?;
        let obj = node.as_object_mut().context("file node object")?;
        obj.insert("size".into(), json!(bytes.len()));
        obj.insert("offset".into(), json!(offset.to_string()));
        obj.insert("integrity".into(), asar_integrity(bytes));
        offset += bytes.len();
        chunks.push(bytes.clone());
    }
    write_asar_bytes(&header, &chunks)
}
