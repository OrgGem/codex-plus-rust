//! JS bindings for `src/github-repository.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod port;
pub mod utf16;

/// `isGitHubRepositoryHomepage`.
fn is_git_hub_repository_homepage_inner(value: String) -> Result<bool, String> {
    let out = port::is_git_hub_repository_homepage(value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "isGitHubRepositoryHomepage")]
pub fn is_git_hub_repository_homepage(value: String) -> Result<bool, JsError> {
    is_git_hub_repository_homepage_inner(value).map_err(|e| JsError::new(&e))
}
