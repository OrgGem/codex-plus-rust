//! The Rust port of `src/i18n.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::{
    i18n_en, is_nil, is_plain_object, string_units, translate_plain, translate_template,
    utf16_string,
};
use serde_json::Value;
use std::cell::RefCell;

thread_local! {
    static LANG: RefCell<String> = RefCell::new("zh".to_string());
}

fn current_lang() -> String {
    LANG.with(|lang| lang.borrow().clone())
}

fn find_placeholders(tmpl: &str) -> (bool, Vec<usize>) {
    let bytes = tmpl.as_bytes();
    let mut found = false;
    let mut indexes = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'{'
            && let Some(rel_end) = bytes[i + 1..].iter().position(|&b| b == b'}')
            && rel_end > 0
            && bytes[i + 1..i + 1 + rel_end].iter().all(u8::is_ascii_digit)
        {
            found = true;
            if let Ok(idx) = std::str::from_utf8(&bytes[i + 1..i + 1 + rel_end])
                .unwrap_or("")
                .parse::<usize>()
            {
                indexes.push(idx);
            }
            i += rel_end + 2;
            continue;
        }
        i += tmpl[i..].chars().next().map_or(1, char::len_utf8);
    }
    (found, indexes)
}

/// Port of `getLanguage` (`src/i18n.ts:33`).
pub fn get_language() -> Result<String, String> {
    Ok(current_lang())
}

/// Port of `t` (`src/i18n.ts:38`).
pub fn t(zh: String) -> Result<String, String> {
    let lang = current_lang();
    if lang != "en" {
        return Ok(zh);
    }
    let translated = translate_plain(&zh, &lang);
    // JS `if (plain) return plain;` skips empty-string dictionary entries (e.g. "个": "").
    if translated.is_empty() && !zh.is_empty() {
        return Ok(zh);
    }
    Ok(translated)
}

/// Port of `tf` (`src/i18n.ts:54`).
pub fn tf(key: String, args: Value) -> Result<String, String> {
    let lang = current_lang();
    if let Some(arr) = args.as_array() {
        return Ok(translate_template(&key, arr, &lang));
    }
    let tmpl = if lang == "en" {
        i18n_en()
            .template
            .get(&key)
            .map_or(key.as_str(), String::as_str)
    } else {
        key.as_str()
    };
    let (has_placeholder, indexes) = find_placeholders(tmpl);
    if !has_placeholder {
        return Ok(translate_template(&key, &[], &lang));
    }
    if is_nil(&args) {
        return Err("TypeError: Cannot read properties of null/undefined".into());
    }
    let mut slice = Vec::new();
    let max_idx = indexes.iter().copied().max().unwrap_or(0);
    if max_idx < 4096 {
        if let Some(obj) = args.as_object().filter(|_| is_plain_object(&args)) {
            slice.resize(max_idx + 1, Value::Null);
            for idx in indexes {
                if let Some(v) = obj.get(&idx.to_string()) {
                    slice[idx] = v.clone();
                }
            }
        } else if let Some(units) = string_units(&args) {
            slice.resize(max_idx.min(units.len()) + 1, Value::Null);
            for idx in indexes {
                if let Some(&u) = units.get(idx) {
                    slice[idx] = utf16_string(&[u]);
                }
            }
        }
    }
    Ok(translate_template(&key, &slice, &lang))
}

/// Port of `setLanguage` (`src/i18n.ts:63`).
pub fn set_language(language: String) -> Result<(), String> {
    LANG.with(|lang| *lang.borrow_mut() = language);
    Ok(())
}

/// Port of `toggleLanguage` (`src/i18n.ts:75`).
pub fn toggle_language() -> Result<(), String> {
    LANG.with(|lang| {
        let mut current = lang.borrow_mut();
        *current = if *current == "en" { "zh" } else { "en" }.to_string();
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn switches_language_and_translates_plain_backend_and_templates() {
        assert_eq!(set_language("zh".into()), Ok(()));
        assert_eq!(get_language(), Ok("zh".into()));
        assert_eq!(t("暂无描述".into()), Ok("暂无描述".into()));
        assert_eq!(
            tf("备份数量：{0}".into(), json!([3])),
            Ok("备份数量：3".into())
        );

        assert_eq!(set_language("en".into()), Ok(()));
        assert_eq!(get_language(), Ok("en".into()));
        assert_eq!(t("暂无描述".into()), Ok("No description".into()));
        assert_eq!(t("个".into()), Ok("个".into()));
        assert_eq!(
            t("Skill 包含无法安全比较的符号链接：C:/skills/example/nested".into()),
            Ok("The skill contains a symbolic link that cannot be compared safely: C:/skills/example/nested".into())
        );
        assert_eq!(
            tf(
                "已统计 {0}/{1} 份，文件大小：{2}".into(),
                json!([2, 10, "4 KB"])
            ),
            Ok("Calculated 2/10 backups; file size: 4 KB".into())
        );
        assert_eq!(tf("a".into(), Value::Null), Ok("a".into()));
        assert!(tf("{0}".into(), Value::Null).is_err());

        assert_eq!(toggle_language(), Ok(()));
        assert_eq!(get_language(), Ok("zh".into()));
        assert_eq!(toggle_language(), Ok(()));
        assert_eq!(get_language(), Ok("en".into()));
        assert_eq!(set_language("zh".into()), Ok(()));
    }
}
