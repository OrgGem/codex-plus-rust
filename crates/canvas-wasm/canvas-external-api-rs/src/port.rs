//! The Rust port of `external-api.mjs`.

#![forbid(unsafe_code)]

use canvas_common_rs::{
    is_js_ws, is_nil, is_truthy, js_str, string_units, undefined, utf16_string,
};
use serde_json::{Map, Value, json};
use url::Url;

/// Port of `apiEndpoint` (`external-api.mjs:2`).
pub fn api_endpoint(raw: Value) -> Result<Value, String> {
    let trimmed = js_str(&raw).trim_matches(is_js_ws).to_string();
    let Ok(mut u) = Url::parse(&trimmed) else {
        return Err("请输入完整的 HTTPS API 地址".into());
    };
    let bad_q = u.query().is_some_and(|q| !q.is_empty());
    let bad_f = u.fragment().is_some_and(|f| !f.is_empty());
    if u.scheme() != "https" || !u.username().is_empty() || u.password().is_some() || bad_q || bad_f
    {
        return Err("API 地址须使用 HTTPS，且不含账号、密码、查询参数或片段".into());
    }
    if u.query() == Some("") {
        u.set_query(None);
    }
    if u.fragment() == Some("") {
        u.set_fragment(None);
    }
    let p = u.path().trim_end_matches('/').to_string();
    let target = if p.ends_with("/chat/completions") {
        p
    } else {
        format!("{}/chat/completions", if p.is_empty() { "/v1" } else { &p })
    };
    u.set_path(&target);
    Ok(Value::String(u.to_string()))
}

/// Port of `storedApiConfig` (`external-api.mjs:21`).
pub fn stored_api_config(config: Value) -> Result<Value, String> {
    if is_nil(&config) {
        return Err("TypeError: Cannot destructure property 'channel' of null/undefined".into());
    }
    let mut out = Map::new();
    for k in [
        "channel", "baseUrl", "model", "remember", "speed", "revision",
    ] {
        out.insert(k.into(), config.get(k).cloned().unwrap_or_else(undefined));
    }
    if config.get("remember").is_some_and(is_truthy) {
        out.insert(
            "key".into(),
            config.get("key").cloned().unwrap_or_else(undefined),
        );
    }
    Ok(Value::Object(out))
}

/// Port of `apiRequestBody` (`external-api.mjs:35`).
pub fn api_request_body(config: Value, prompt: Value) -> Result<Value, String> {
    if is_nil(&config) {
        return Err("TypeError: Cannot read properties of null/undefined".into());
    }
    let u = undefined();
    let model = config.get("model").cloned().unwrap_or_else(undefined);
    let ep_str = js_str(config.get("endpoint").unwrap_or(&u));
    let ep = Url::parse(ep_str.trim_matches(is_js_ws))
        .map_err(|_| "TypeError: Invalid URL".to_string())?;
    let mut body = json!({
        "model": model,
        "stream": true,
        "messages": [{ "role": "user", "content": prompt }]
    });
    let deepseek_host = ep.host_str() == Some("api.deepseek.com");
    let v4_model = matches!(
        model.as_str(),
        Some("deepseek-v4-flash" | "deepseek-v4-pro")
    );
    let not_provider = config.get("speed").and_then(Value::as_str) != Some("provider");
    if deepseek_host && v4_model && not_provider {
        body["thinking"] = json!({ "type": "disabled" });
    }
    Ok(body)
}

/// Port of `completionText` (`external-api.mjs:76`).
pub fn completion_text(body: Value) -> Result<Value, String> {
    let choice = match body.get("choices") {
        Some(Value::Array(a)) => a.first(),
        Some(Value::Object(o)) => o.get("0"),
        _ => None,
    };
    let finish = choice
        .and_then(|c| c.get("finish_reason"))
        .and_then(Value::as_str);
    if finish == Some("length") {
        return Err("API 输出达到长度上限，本批未提交；请使用输出容量更大的模型".into());
    }
    let msg = choice.and_then(|c| c.get("message"));
    if finish == Some("content_filter") || msg.and_then(|m| m.get("refusal")).is_some_and(is_truthy)
    {
        return Err("API 未能生成本批整理结果".into());
    }
    let content = msg.and_then(|m| m.get("content"));
    let out = match content {
        Some(c) if string_units(c).is_some() => c.clone(),
        Some(Value::Array(parts)) => {
            let mut joined = Vec::new();
            for p in parts {
                if !is_nil(p) && p.get("type").and_then(Value::as_str) == Some("text") {
                    let t = p.get("text").filter(|v| is_truthy(v));
                    if let Some(v) = t {
                        joined.extend(
                            string_units(v).unwrap_or_else(|| js_str(v).encode_utf16().collect()),
                        );
                    }
                }
            }
            utf16_string(&joined)
        }
        _ => Value::String(String::new()),
    };
    if js_str(&out).trim_matches(is_js_ws).is_empty() {
        return Err(
            "API 未返回有效的 choices[0].message.content，请确认兼容 Chat Completions".into(),
        );
    }
    Ok(out)
}
