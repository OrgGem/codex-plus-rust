//! The Rust port of `src/theme-mode.ts`.

#![forbid(unsafe_code)]

/// Port of `parseThemePreference` (`src/theme-mode.ts:11`).
pub fn parse_theme_preference(raw: Option<String>) -> Result<String, String> {
    Ok(match raw.as_deref() {
        Some("dark") => "dark".into(),
        Some("light") => "light".into(),
        _ => "system".into(),
    })
}

/// Port of `resolveTheme` (`src/theme-mode.ts:15`).
pub fn resolve_theme(preference: String, system_prefers_dark: bool) -> Result<String, String> {
    if preference == "system" {
        Ok(if system_prefers_dark { "dark" } else { "light" }.into())
    } else {
        Ok(preference)
    }
}

/// Port of `toggledThemePreference` (`src/theme-mode.ts:21`).
pub fn toggled_theme_preference(current: String) -> Result<String, String> {
    Ok(if current == "dark" { "light" } else { "dark" }.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_and_fallback_preferences() {
        assert_eq!(
            parse_theme_preference(Some("dark".into())).ok(),
            Some("dark".into())
        );
        assert_eq!(
            parse_theme_preference(Some("light".into())).ok(),
            Some("light".into())
        );
        assert_eq!(
            parse_theme_preference(Some("system".into())).ok(),
            Some("system".into())
        );
        assert_eq!(parse_theme_preference(None).ok(), Some("system".into()));
        assert_eq!(
            parse_theme_preference(Some("blue".into())).ok(),
            Some("system".into())
        );
    }

    #[test]
    fn resolves_and_toggles_theme() {
        assert_eq!(
            resolve_theme("system".into(), true).ok(),
            Some("dark".into())
        );
        assert_eq!(
            resolve_theme("system".into(), false).ok(),
            Some("light".into())
        );
        assert_eq!(
            resolve_theme("dark".into(), false).ok(),
            Some("dark".into())
        );
        assert_eq!(
            resolve_theme("light".into(), true).ok(),
            Some("light".into())
        );
        assert_eq!(
            toggled_theme_preference("dark".into()).ok(),
            Some("light".into())
        );
        assert_eq!(
            toggled_theme_preference("light".into()).ok(),
            Some("dark".into())
        );
    }
}
