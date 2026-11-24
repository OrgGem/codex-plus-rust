//! The Rust port of `src/github-repository.ts`.

#![forbid(unsafe_code)]

use url::Url;

const NON_OWNER_ROUTES: &[&str] = &[
    "about",
    "account",
    "advisories",
    "apps",
    "business",
    "codespaces",
    "collections",
    "contact",
    "copilot",
    "customer-stories",
    "dashboard",
    "edu",
    "enterprise",
    "events",
    "explore",
    "features",
    "git-guides",
    "github-copilot",
    "issues",
    "login",
    "marketplace",
    "mcp",
    "mobile",
    "new",
    "newsletter",
    "newsroom",
    "notifications",
    "open-source",
    "organizations",
    "orgs",
    "pricing",
    "pulls",
    "readme",
    "resources",
    "search",
    "security",
    "settings",
    "site",
    "sitemap",
    "social-impact",
    "solutions",
    "sponsors",
    "stars",
    "team",
    "topics",
    "trending",
    "users",
    "why-github",
];

fn is_valid_owner(owner: &str) -> bool {
    !owner.is_empty()
        && owner.len() <= 39
        && !NON_OWNER_ROUTES.contains(&owner.to_ascii_lowercase().as_str())
        && owner
            .split('-')
            .all(|seg| !seg.is_empty() && seg.bytes().all(|b| b.is_ascii_alphanumeric()))
}

fn is_valid_repo(repo: &str) -> bool {
    !repo.is_empty()
        && repo.len() <= 100
        && repo
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-'))
}

/// Port of `isGitHubRepositoryHomepage` (`src/github-repository.ts:55`).
pub fn is_git_hub_repository_homepage(value: String) -> Result<bool, String> {
    let Ok(url) = Url::parse(&value) else {
        return Ok(false);
    };
    if url.scheme() != "https" || !matches!(url.host_str(), Some("github.com" | "www.github.com")) {
        return Ok(false);
    }
    if !url.username().is_empty()
        || url.password().is_some_and(|p| !p.is_empty())
        || url.port().is_some()
    {
        return Ok(false);
    }
    let path = url.path();
    let Some(after_lead) = path.strip_prefix('/') else {
        return Ok(false);
    };
    let trimmed = after_lead.strip_suffix('/').unwrap_or(after_lead);
    let Some((owner, raw_repo)) = trimmed.split_once('/') else {
        return Ok(false);
    };
    if owner.is_empty() || raw_repo.is_empty() || raw_repo.contains('/') {
        return Ok(false);
    }
    let repository = raw_repo.strip_suffix(".git").unwrap_or(raw_repo);
    Ok(is_valid_owner(owner) && is_valid_repo(repository))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_and_rejects_invalid_github_urls() {
        for valid in [
            "https://github.com/0xTotoroX/codex-hide-usage-alert",
            "https://www.github.com/0xTotoroX/tux-toolbar-buddy/",
            "https://github.com/0xTotoroX/codex-hide-usage-alert.git",
            "https://github.com/BigPizzaV3/CodexPlusPlus?tab=readme-ov-file#readme",
        ] {
            assert!(
                is_git_hub_repository_homepage(valid.into()).unwrap(),
                "{valid}"
            );
        }
        for invalid in [
            "",
            "not a URL",
            "http://github.com/0xTotoroX/codex-hide-usage-alert",
            "https://github.com/0xTotoroX",
            "https://github.com/0xTotoroX/codex-hide-usage-alert/issues",
            "https://github.com/settings/profile",
            "https://github.com/0xTotoroX//codex-hide-usage-alert",
            "https://github.com/0xTotoroX/codex-hide-usage-alert%2Fissues",
            "https://github.com/-invalid/codex-hide-usage-alert",
        ] {
            assert!(
                !is_git_hub_repository_homepage(invalid.into()).unwrap(),
                "{invalid}"
            );
        }
    }
}
