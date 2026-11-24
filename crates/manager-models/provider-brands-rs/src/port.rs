//! The Rust port of `src/provider-brands.ts`.

#![forbid(unsafe_code)]

use serde_json::Value;

#[derive(Clone, Copy)]
enum Pat {
    Sub(&'static str),
    Word(&'static str),
    RightWord(&'static str),
}

use Pat::{RightWord as R, Sub as S, Word as W};

const RULES: &[(&[Pat], &str)] = &[
    (&[S("claude"), S("anthropic")], "claude-color"),
    (&[S("chatgpt"), S("openai"), W("gpt"), S("codex")], "openai"),
    (&[S("deepseek")], "deepseek-color"),
    (&[R("z.ai"), W("zai")], "zai"),
    (
        &[S("zhipu"), S("bigmodel"), W("glm"), S("智谱")],
        "zhipu-color",
    ),
    (&[S("kimi"), S("moonshot"), S("月之暗面")], "kimi-color"),
    (&[S("bailian"), S("百炼")], "bailian-color"),
    (
        &[S("qwen"), S("dashscope"), S("aliyun"), S("通义"), S("千问")],
        "qwen-color",
    ),
    (&[S("stepfun"), S("阶跃")], "stepfun-color"),
    (&[S("minimax"), S("hailuo"), S("海螺")], "minimax-color"),
    (&[S("doubao"), S("豆包")], "doubao-color"),
    (&[S("volc"), W("ark"), S("火山")], "volcengine-color"),
    (
        &[
            S("qianfan"),
            S("baidu"),
            S("wenxin"),
            S("千帆"),
            S("百度"),
            S("文心"),
        ],
        "baiducloud-color",
    ),
    (&[S("mimo"), S("xiaomi"), S("小米")], "xiaomimimo"),
    (&[S("modelscope"), S("魔搭")], "modelscope-color"),
    (&[S("longcat")], "longcat-color"),
    (&[S("openrouter")], "openrouter"),
    (&[S("silicon"), S("硅基")], "siliconcloud-color"),
    (&[S("aihubmix")], "aihubmix-color"),
    (&[S("novita")], "novita-color"),
    (&[S("azure")], "azure-color"),
    (&[S("grok")], "grok"),
    (&[R("x.ai"), W("xai")], "xai"),
    (&[S("ollama")], "ollama"),
    (&[S("lm studio"), S("lmstudio")], "lmstudio"),
    (&[S("antigravity")], "antigravity-color"),
    (&[S("vertex")], "vertexai-color"),
    (&[S("gemini")], "gemini-color"),
    (&[S("google")], "google-color"),
    (&[S("copilot")], "githubcopilot"),
    (&[S("mistral")], "mistral-color"),
    (&[S("groq")], "groq"),
    (&[S("together")], "together-color"),
    (&[S("fireworks")], "fireworks-color"),
    (&[S("hunyuan"), S("混元")], "hunyuan-color"),
    (&[S("tencent"), S("腾讯")], "tencentcloud-color"),
    (&[S("huawei"), S("华为")], "huawei-color"),
    (&[S("cursor")], "cursor"),
    (&[S("kiro")], "kiro-color"),
    (&[S("qoder")], "qoder-color"),
    (&[S("workbuddy")], "workbuddy-color"),
    (&[S("codebuddy")], "codebuddy-color"),
    (&[S("command code"), S("commandcode")], "commandcode"),
    (&[S("zcode")], "zai"),
    (&[W("zed")], "zed"),
    (&[S("devin")], "devin-color"),
    (&[S("opencode")], "opencode"),
    (&[S("perplexity")], "perplexity-color"),
    (&[S("new-api"), S("newapi")], "newapi-color"),
    (&[S("hugging face"), S("huggingface")], "huggingface-color"),
    (&[S("nvidia")], "nvidia-color"),
];

fn is_js_ws(c: char) -> bool {
    c != '\u{0085}' && (c.is_whitespace() || c == '\u{FEFF}')
}

fn is_js_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn has_bounded(haystack: &str, needle: &str, left_wb: bool, right_wb: bool) -> bool {
    let bytes = haystack.as_bytes();
    haystack.match_indices(needle).any(|(idx, _)| {
        let left_ok = !left_wb || idx == 0 || !is_js_word_byte(bytes[idx - 1]);
        let end = idx + needle.len();
        let right_ok = !right_wb || end == bytes.len() || !is_js_word_byte(bytes[end]);
        left_ok && right_ok
    })
}

fn matches_pat(haystack: &str, pat: Pat) -> bool {
    match pat {
        S(s) => haystack.contains(s),
        W(s) => has_bounded(haystack, s, true, true),
        R(s) => has_bounded(haystack, s, false, true),
    }
}

/// Port of `brandIconKey` (`src/provider-brands.ts:59`).
pub fn brand_icon_key(name: String, base_url: Option<String>) -> Result<Value, String> {
    let base = base_url.unwrap_or_default();
    for text in [name.as_str(), base.as_str()] {
        let lower = text.trim_matches(is_js_ws).to_lowercase();
        if lower.is_empty() {
            continue;
        }
        if let Some((_, key)) = RULES
            .iter()
            .find(|(pats, _)| pats.iter().any(|&p| matches_pat(&lower, p)))
        {
            return Ok(Value::String((*key).to_string()));
        }
    }
    Ok(Value::Null)
}

/// Port of `brandIconIsColor` (`src/provider-brands.ts:69`).
pub fn brand_icon_is_color(key: String) -> Result<bool, String> {
    Ok(key.ends_with("-color"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn recognizes_brand_icons_and_word_boundaries() {
        assert_eq!(
            brand_icon_key("DeepSeek".into(), None).unwrap(),
            json!("deepseek-color")
        );
        assert_eq!(
            brand_icon_key("Zhipu GLM".into(), None).unwrap(),
            json!("zhipu-color")
        );
        assert_eq!(brand_icon_key("Z.ai".into(), None).unwrap(), json!("zai"));
        assert_eq!(brand_icon_key("z.aibot".into(), None).unwrap(), Value::Null);
        assert_eq!(
            brand_icon_key(
                "哈基米 国模".into(),
                Some("https://api.moonshot.cn/v1".into())
            )
            .unwrap(),
            json!("kimi-color")
        );
        assert_eq!(
            brand_icon_key("Grok (SuperGrok)".into(), None).unwrap(),
            json!("grok")
        );
        assert_eq!(brand_icon_key("xAI".into(), None).unwrap(), json!("xai"));
        assert_eq!(
            brand_icon_key("LM Studio".into(), None).unwrap(),
            json!("lmstudio")
        );
        assert_eq!(
            brand_icon_key("火山引擎 Ark".into(), None).unwrap(),
            json!("volcengine-color")
        );
        assert_eq!(
            brand_icon_key("智谱glm".into(), None).unwrap(),
            json!("zhipu-color")
        );
        assert_eq!(
            brand_icon_key("我的中转".into(), None).unwrap(),
            Value::Null
        );
        assert!(brand_icon_is_color("deepseek-color".into()).unwrap());
        assert!(!brand_icon_is_color("openai".into()).unwrap());
    }
}
