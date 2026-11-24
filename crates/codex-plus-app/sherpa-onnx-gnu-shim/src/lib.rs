//! Safe Pure-Rust GNU-compatible shim for `sherpa-onnx` (`1.13.8`).
//!
//! Avoids linking MSVC `/MT` prebuilt C++ `.lib` archives when compiling `codex-plus-core`
//! under `x86_64-pc-windows-gnu`. Local SenseVoice fallback cleanly reports unavailable
//! unless `CODEX_PLUS_SHERPA_MOCK_TEXT` is set for test environments.

#![forbid(unsafe_code)]

use std::sync::Mutex;

#[derive(Debug, Clone, Default)]
pub struct OfflineSenseVoiceModelConfig {
    pub model: Option<String>,
    pub language: Option<String>,
    pub use_itn: bool,
}

#[derive(Debug, Clone, Default)]
pub struct OfflineModelConfig {
    pub sense_voice: OfflineSenseVoiceModelConfig,
    pub tokens: Option<String>,
    pub provider: Option<String>,
    pub num_threads: i32,
    pub debug: bool,
    pub model_type: Option<String>,
    pub modeling_unit: Option<String>,
    pub bpe_vocab: Option<String>,
    pub telespeech_ctc: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct OfflineRecognizerConfig {
    pub model_config: OfflineModelConfig,
    pub decoding_method: Option<String>,
    pub max_active_paths: i32,
    pub hotwords_file: Option<String>,
    pub hotwords_score: f32,
    pub blank_penalty: f32,
    pub rule_fsts: Option<String>,
    pub rule_far: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct OfflineRecognizerResult {
    pub text: String,
    pub tokens: Vec<String>,
    pub timestamps: Vec<f32>,
    pub lang: String,
    pub emotion: String,
    pub event: String,
}

#[derive(Debug, Default)]
pub struct OfflineStream {
    text: Mutex<Option<String>>,
}

impl OfflineStream {
    pub fn accept_waveform(&self, _sample_rate: i32, _samples: &[f32]) {}

    pub fn get_result(&self) -> Option<OfflineRecognizerResult> {
        let guard = self.text.lock().unwrap_or_else(|e| e.into_inner());
        guard.as_ref().map(|t| OfflineRecognizerResult {
            text: t.clone(),
            ..OfflineRecognizerResult::default()
        })
    }
}

#[derive(Debug, Default)]
pub struct OfflineRecognizer {
    mock_text: String,
}

impl OfflineRecognizer {
    pub fn create(_config: &OfflineRecognizerConfig) -> Option<Self> {
        if !cfg!(debug_assertions) {
            return None;
        }
        let text = std::env::var("CODEX_PLUS_SHERPA_MOCK_TEXT").ok()?;
        Some(Self { mock_text: text })
    }

    pub fn create_stream(&self) -> OfflineStream {
        OfflineStream {
            text: Mutex::new(None),
        }
    }

    pub fn decode(&self, stream: &OfflineStream) {
        let mut guard = stream.text.lock().unwrap_or_else(|e| e.into_inner());
        *guard = Some(self.mock_text.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gnu_shim_handles_missing_mock_cleanly() {
        if std::env::var_os("CODEX_PLUS_SHERPA_MOCK_TEXT").is_none() {
            let cfg = OfflineRecognizerConfig::default();
            assert!(OfflineRecognizer::create(&cfg).is_none());
        }
    }
}
