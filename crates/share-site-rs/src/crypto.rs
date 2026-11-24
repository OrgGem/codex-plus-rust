//! AES-256-GCM base64url encryption/decryption matching `services/share-site/app.js`.

use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncryptedPayload {
    pub v: u8,
    pub iv: String,
    pub ciphertext: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncryptedShare {
    pub key: String,
    pub payload: EncryptedPayload,
}

pub fn to_base64_url(bytes: &[u8]) -> String {
    URL_SAFE_NO_PAD.encode(bytes)
}

pub fn from_base64_url(value: &str) -> Result<Vec<u8>, String> {
    let unpadded = if let Some(stripped) = value.strip_suffix("==") {
        if !value.len().is_multiple_of(4) || stripped.contains('=') {
            return Err("Invalid base64url padding".into());
        }
        stripped
    } else if let Some(stripped) = value.strip_suffix('=') {
        if !value.len().is_multiple_of(4) || stripped.contains('=') {
            return Err("Invalid base64url padding".into());
        }
        stripped
    } else {
        value
    };
    URL_SAFE_NO_PAD
        .decode(unpadded)
        .map_err(|e| format!("Invalid base64url input: {e}"))
}

pub fn sha256_b64u(value: &[u8]) -> String {
    to_base64_url(&Sha256::digest(value))
}

pub fn random_token_b64u(byte_length: usize) -> Result<String, String> {
    let mut buf = vec![0_u8; byte_length];
    getrandom::getrandom(&mut buf).map_err(|e| format!("RNG error: {e}"))?;
    Ok(to_base64_url(&buf))
}

pub fn timing_safe_eq_str(left: &str, right: &str) -> bool {
    let left_hash = Sha256::digest(left.as_bytes());
    let right_hash = Sha256::digest(right.as_bytes());
    left_hash.as_slice().ct_eq(right_hash.as_slice()).into()
}

pub fn encrypt_text_with_material(
    value: &str,
    key_bytes: &[u8; 32],
    iv_bytes: &[u8; 12],
) -> Result<EncryptedShare, String> {
    let cipher = Aes256Gcm::new_from_slice(key_bytes).map_err(|e| format!("Key error: {e}"))?;
    let ct = cipher
        .encrypt(Nonce::from_slice(iv_bytes), value.as_bytes())
        .map_err(|e| format!("Encrypt error: {e}"))?;
    Ok(EncryptedShare {
        key: to_base64_url(key_bytes),
        payload: EncryptedPayload {
            v: 1,
            iv: to_base64_url(iv_bytes),
            ciphertext: to_base64_url(&ct),
        },
    })
}

pub fn encrypt_text(value: &str) -> Result<EncryptedShare, String> {
    let mut key_bytes = [0_u8; 32];
    let mut iv_bytes = [0_u8; 12];
    getrandom::getrandom(&mut key_bytes).map_err(|e| format!("RNG error: {e}"))?;
    getrandom::getrandom(&mut iv_bytes).map_err(|e| format!("RNG error: {e}"))?;
    encrypt_text_with_material(value, &key_bytes, &iv_bytes)
}

pub fn decrypt_text(payload: &EncryptedPayload, key_value: &str) -> Result<String, String> {
    if payload.v != 1 {
        return Err("不支持此分享的数据格式。".into());
    }
    if payload.ciphertext.len() > crate::worker::MAX_CIPHERTEXT_LENGTH {
        return Err("分享内容超过大小限制。".into());
    }
    let key_bytes = from_base64_url(key_value)?;
    let cipher = Aes256Gcm::new_from_slice(&key_bytes).map_err(|_| "解密失败。".to_string())?;
    let iv_bytes = from_base64_url(&payload.iv)?;
    if iv_bytes.len() != 12 {
        return Err("解密失败。".into());
    }
    let ct = from_base64_url(&payload.ciphertext)?;
    let pt = cipher
        .decrypt(Nonce::from_slice(&iv_bytes), ct.as_slice())
        .map_err(|_| "解密失败。".to_string())?;
    String::from_utf8(pt).map_err(|e| format!("UTF-8 decode error: {e}"))
}
