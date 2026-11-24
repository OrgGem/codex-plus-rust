//! The Rust port of `src/relay-session.ts`.

#![forbid(unsafe_code)]

/// Port of `sessionProviderForProtocol` (`src/relay-session.ts:5`).
pub fn session_provider_for_protocol(
    session_provider: String,
    protocol: String,
) -> Result<String, String> {
    if protocol != "responses" && session_provider == "openai" {
        Ok("custom".into())
    } else {
        Ok(session_provider)
    }
}

/// Port of `openAiSessionBlocksModelSave` (`src/relay-session.ts:14`).
pub fn open_ai_session_blocks_model_save(
    session_provider: String,
    protocol: String,
) -> Result<bool, String> {
    Ok(session_provider == "openai" && protocol != "responses")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_provider_and_save_gate_match_protocol() {
        assert_eq!(
            session_provider_for_protocol("openai".into(), "chatCompletions".into()).unwrap(),
            "custom"
        );
        assert_eq!(
            session_provider_for_protocol("openai".into(), "responses".into()).unwrap(),
            "openai"
        );
        assert_eq!(
            session_provider_for_protocol("custom".into(), "anthropicMessages".into()).unwrap(),
            "custom"
        );
        assert!(
            open_ai_session_blocks_model_save("openai".into(), "chatCompletions".into()).unwrap()
        );
        assert!(!open_ai_session_blocks_model_save("openai".into(), "responses".into()).unwrap());
        assert!(
            !open_ai_session_blocks_model_save("custom".into(), "chatCompletions".into()).unwrap()
        );
    }
}
