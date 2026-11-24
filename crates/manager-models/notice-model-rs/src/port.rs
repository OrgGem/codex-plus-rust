//! The Rust port of `src/notice-model.ts`.

#![forbid(unsafe_code)]

/// Port of `noticeTone` (`src/notice-model.ts:8`).
pub fn notice_tone(status: Option<String>) -> Result<String, String> {
    let value = status.as_deref().unwrap_or("ok").to_lowercase();
    let tone = match value.as_str() {
        "failed" | "error" | "crashed" => "error",
        "warning" | "partial" | "ready_degraded" | "not_implemented" | "wait" => "warning",
        "info" | "not_checked" | "starting" | "running" => "info",
        _ => "success",
    };
    Ok(tone.to_string())
}

/// Port of `noticeDuration` (`src/notice-model.ts:17`).
pub fn notice_duration(tone: String, message: String) -> Result<f64, String> {
    let base = match tone.as_str() {
        "error" => 9000.0,
        "warning" => 6500.0,
        _ => 4200.0,
    };
    let extra = (message.encode_utf16().count().saturating_sub(60) as f64) * 30.0;
    Ok((base + extra).min(15_000.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_tone_and_duration() {
        assert_eq!(notice_tone(None).unwrap(), "success");
        assert_eq!(notice_tone(Some("FAILED".into())).unwrap(), "error");
        assert_eq!(notice_tone(Some("partial".into())).unwrap(), "warning");
        assert_eq!(notice_tone(Some("not_checked".into())).unwrap(), "info");
        assert_eq!(
            notice_duration("success".into(), "短消息".into()).unwrap(),
            4200.0
        );
        assert_eq!(
            notice_duration("error".into(), "字".repeat(5000)).unwrap(),
            15_000.0
        );
        assert_eq!(
            notice_duration("success".into(), "😀".repeat(35)).unwrap(),
            4500.0
        );
    }
}
