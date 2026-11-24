//! The Rust port of `src/onboarding-model.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::{is_nil, js_number, parse_js_number, to_number};
use serde_json::{Map, Value, json};

pub const TOUR_VERSION: f64 = 1.0;
pub const TOUR_STORAGE_KEY: &str = "codex-plus-plus.onboarding";

fn step_json(
    id: &str,
    target: Option<&str>,
    title: &str,
    body: &str,
    args: Option<Vec<String>>,
    plain_title: Option<bool>,
) -> Value {
    let mut map = Map::new();
    map.insert("id".into(), Value::String(id.into()));
    if let Some(t) = target {
        map.insert("target".into(), Value::String(t.into()));
    }
    map.insert("title".into(), Value::String(title.into()));
    map.insert("body".into(), Value::String(body.into()));
    if let Some(a) = args {
        map.insert(
            "args".into(),
            Value::Array(a.into_iter().map(Value::String).collect()),
        );
    }
    if let Some(pt) = plain_title {
        map.insert("plainTitle".into(), Value::Bool(pt));
    }
    Value::Object(map)
}

const RAW_STEPS: &[(&str, Option<&str>, &str, &str)] = &[
    (
        "welcome",
        None,
        "欢迎使用 Codex++",
        "Codex++ 给 Codex、Claude Code 等 Agent 加上拓展和增强，并让它们经同一个本地网关接入任意供应商。花半分钟认识一下侧栏。",
    ),
    (
        "management",
        Some("management"),
        "万物皆拓展",
        "Codex++ 的增强能力都以拓展开放：在这里启用内置拓展、安装社区拓展，或在 Codex 对话里说一句就造一个自己的。下面是 MCP、Skills 和会话。",
    ),
    (
        "hub",
        Some("hub"),
        "聚合网关",
        "任意 Agent 经 Codex++ 接入任意供应商：一个 Agent 可以同时用多家，按模型路由、出错自动换下一家。总览里一屏看全谁在用、走哪家。",
    ),
    (
        "providers",
        Some("nav-gatewayProviders"),
        "第一步：添加供应商",
        "把 API Key 或 ChatGPT、Copilot 等订阅加进供应商池，所有 Agent 共用这一个池子。",
    ),
    (
        "agents-connect",
        Some("nav-gateway"),
        "第二步：接入 Agent",
        "给 Agent 选一个默认模型、打开接入开关；供应商池里的模型就会出现在它自己的模型列表里。",
    ),
    (
        "agents",
        Some("agents"),
        "Agent 增强",
        "每个 Agent 一行，点开后它的页面列在下面：概览、连接与供应商、内置拓展、皮肤……Codex 行的播放键可以启动或停止 Codex。",
    ),
    (
        "palette",
        Some("palette"),
        "搜索页面和操作",
        "结束导览后，按 ⌘K 搜索页面、开关和常用操作。",
    ),
    (
        "settings",
        Some("settings"),
        "设置",
        "主题、版本更新、日志诊断和各项高级设置都在齿轮里。想再看一遍这个引导，在「设置 → 通用」里重新打开。",
    ),
];

/// Port of `tourSteps` (`src/onboarding-model.ts:74`).
pub fn tour_steps(
    solo_name: Option<String>,
    enhance_only: Option<bool>,
) -> Result<Vec<Value>, String> {
    if enhance_only.unwrap_or(false) {
        return Ok(RAW_STEPS
            .iter()
            .filter(|(id, _, _, _)| !matches!(*id, "hub" | "providers" | "agents-connect"))
            .map(|(id, target, title, body)| {
                let body = match *id {
                    "welcome" => "Codex++ 给 Codex 和 Claude Code 加上增强、皮肤、拓展等功能，供应商仍由你自己的工具管理。花半分钟认识一下侧栏。",
                    "agents" => "点开 Codex 或 Claude Code，在它下面的页面里启用所需增强；Codex 行的播放键可以启动或停止 Codex。",
                    "management" => "Codex++ 的增强能力都以拓展开放：在这里启用内置拓展、安装社区拓展，或在对话里造一个自己的。MCP、Skills 和会话就在它下面。",
                    _ => body,
                };
                step_json(id, *target, title, body, None, None)
            })
            .collect());
    }

    let solo = solo_name.filter(|s| !s.is_empty());
    Ok(RAW_STEPS
        .iter()
        .map(|(id, target, title, body)| {
            if let Some(ref name) = solo {
                let ov = match *id {
                    "welcome" => Some((
                        None,
                        "Codex++ 给 {0} 加上拓展和增强，并让它经本地网关接入任意供应商。花半分钟认识一下侧栏。",
                    )),
                    "agents-connect" => Some((
                        Some("第二步：让 {0} 走网关"),
                        "打开 {0} 的网关开关并选一个默认模型；供应商池里的模型就会出现在 {0} 自己的模型列表里。",
                    )),
                    "agents" => Some((
                        Some("{0} 的页面"),
                        "点开 {0} 后它的页面列在下面：概览、连接与供应商、内置拓展、皮肤……其他 Agent 的入口可在设置里切到「全部功能」后显示。",
                    )),
                    _ => None,
                };
                if let Some((ov_title, ov_body)) = ov {
                    return step_json(
                        id,
                        *target,
                        ov_title.unwrap_or(title),
                        ov_body,
                        Some(vec![name.clone()]),
                        Some(ov_title.is_none()),
                    );
                }
            }
            step_json(id, *target, title, body, None, None)
        })
        .collect())
}

/// Port of `shouldShowTour` (`src/onboarding-model.ts:107`).
pub fn should_show_tour(stored: Option<String>) -> Result<bool, String> {
    let Some(s) = stored.filter(|v| !v.is_empty()) else {
        return Ok(true);
    };
    let version = parse_js_number(&s);
    Ok(!version.is_finite() || version < TOUR_VERSION)
}

fn js_min(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else {
        a.min(b)
    }
}

fn js_max(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else {
        a.max(b)
    }
}

fn field_num(obj: &Value, key: &str) -> f64 {
    obj.get(key).map_or(f64::NAN, to_number)
}

/// Port of `placeCard` (`src/onboarding-model.ts:114`).
pub fn place_card(
    target: Value,
    card: Value,
    viewport: Value,
    gap: Option<f64>,
) -> Result<Value, String> {
    if is_nil(&target) {
        return Err("TypeError: Cannot read properties of null/undefined (reading 'top')".into());
    }
    if is_nil(&card) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'height')".into(),
        );
    }
    if is_nil(&viewport) {
        return Err("TypeError: Cannot read properties of null/undefined (reading 'width')".into());
    }
    let gap = gap.unwrap_or(14.0);
    let t_top = field_num(&target, "top");
    let t_bottom = field_num(&target, "bottom");
    let t_right = field_num(&target, "right");
    let t_left = field_num(&target, "left");
    let c_width = field_num(&card, "width");
    let c_height = field_num(&card, "height");
    let vp_width = field_num(&viewport, "width");
    let vp_height = field_num(&viewport, "height");

    let clamp_top = |top: f64| js_max(12.0, js_min(top, vp_height - c_height - 12.0));
    let middle = (t_top + t_bottom) / 2.0 - c_height / 2.0;

    let (left, top, side) = if t_right + gap + c_width <= vp_width - 12.0 {
        (t_right + gap, clamp_top(middle), "right")
    } else if t_left - gap - c_width >= 12.0 {
        (t_left - gap - c_width, clamp_top(middle), "left")
    } else {
        let clamped_left = js_max(12.0, js_min(t_left, vp_width - c_width - 12.0));
        (clamped_left, clamp_top(t_bottom + gap), "bottom")
    };

    Ok(json!({
        "left": js_number(left),
        "top": js_number(top),
        "side": side,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tour_steps_modes_visibility_and_card_placement() {
        assert!(should_show_tour(None).unwrap());
        assert!(should_show_tour(Some("garbage".into())).unwrap());
        assert!(!should_show_tour(Some("1".into())).unwrap());

        assert_eq!(tour_steps(None, None).unwrap().len(), 8);
        assert_eq!(tour_steps(None, Some(true)).unwrap().len(), 5);
        let solo = tour_steps(Some("Codex".into()), None).unwrap();
        assert_eq!(solo[0]["plainTitle"], json!(true));
        assert_eq!(solo[0]["args"], json!(["Codex"]));

        let right = place_card(
            json!({ "left": 10, "top": 100, "right": 210, "bottom": 130 }),
            json!({ "width": 320, "height": 180 }),
            json!({ "width": 1200, "height": 800 }),
            None,
        )
        .unwrap();
        assert_eq!(right, json!({ "left": 224, "top": 25, "side": "right" }));
        assert!(place_card(Value::Null, json!({}), json!({}), None).is_err());
    }
}
