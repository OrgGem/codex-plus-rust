//! Shared utilities, static embedded manager assets, and reusable domain helpers.

#![forbid(unsafe_code)]

pub mod assets_data;
pub mod domain;
pub mod gateway_shared;
pub mod js_rt;
pub mod relay_shared;

pub use assets_data::*;
pub use domain::*;
pub use js_rt::*;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn embedded_assets_load_completely() {
        assert_eq!(
            presets_value().as_array().map(Vec::len),
            Some(23),
            "all presets loaded"
        );
        assert!(enhancement_settings_value().get("items").is_some());
        let dict = i18n_en();
        assert_eq!(
            dict.backend_patterns.len(),
            112,
            "all 112 patterns compiled"
        );
        assert_eq!(
            translate_template("供应商 {0}", &[json!(2)], "en"),
            "Provider 2"
        );
        assert_eq!(translate_plain("暂无描述", "en"), "No description");
        assert_eq!(translate_plain("暂无描述", "zh"), "暂无描述");
    }

    #[test]
    fn js_sort_handles_nan_on_large_slices_and_domain_guards_hold() {
        let mut items: Vec<(usize, f64)> = (0..70)
            .map(|i| {
                let ts = if i % 3 == 0 { f64::NAN } else { (i % 7) as f64 };
                (i, ts)
            })
            .collect();
        js_sort(&mut items, |a, b| b.1 - a.1);
        assert_eq!(items.len(), 70);

        let mut equal_items: Vec<(usize, i32)> = (0..80).map(|i| (i, 5)).collect();
        js_sort(&mut equal_items, |a, b| f64::from(a.1 - b.1));
        let indices: Vec<usize> = equal_items.into_iter().map(|(i, _)| i).collect();
        assert_eq!(indices, (0..80).collect::<Vec<_>>());

        assert!(!is_valid_github_repo("."));
        assert!(!is_valid_github_repo(".."));
        assert!(!is_git_hub_repository_homepage(
            "https://github.com/owner/..git"
        ));
        assert!(!is_git_hub_repository_homepage(
            "https://github.com/owner/...git"
        ));
        assert!(is_valid_github_repo(".dotfiles"));

        let toml = "[sec]\na+b(1)[x = \"ok\"\n";
        assert_eq!(
            toml_section_string_value(toml, "sec", "a+b(1)[x").unwrap(),
            "ok"
        );
    }
}
