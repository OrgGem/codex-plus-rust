//! The Rust port of `src/provider-test.ts`.

#![forbid(unsafe_code)]

use crate::js::math_round;
use manager_common_rs::{is_nil, is_truthy, number_to_string};
use serde_json::{Value, json};

fn hint(text: &str, args: Vec<Value>) -> Value {
    json!({ "text": text, "args": args })
}

fn to_fixed_1(x: f64) -> String {
    if x >= 1e21 {
        return number_to_string(x);
    }
    let bits = x.to_bits();
    let exp = ((bits >> 52) & 0x7FF) as i32 - 1023 - 52;
    let mant = (1u64 << 52) | (bits & ((1u64 << 52) - 1));
    let scaled = u128::from(mant) * 10;
    let tenths = if exp >= 0 {
        scaled << (exp as u32)
    } else {
        let den = 1u128 << ((-exp) as u32);
        let q = scaled / den;
        let r = scaled % den;
        if r >= (den >> 1) { q + 1 } else { q }
    };
    format!("{}.{}", tenths / 10, tenths % 10)
}

/// Port of `formatProbeMs` (`src/provider-test.ts:32`).
pub fn format_probe_ms(ms: Option<f64>) -> Result<String, String> {
    let Some(ms) = ms.filter(|m| m.is_finite()) else {
        return Ok("—".into());
    };
    if ms < 1000.0 {
        let rounded = math_round(ms);
        let clamped = if rounded <= 0.0 { 0.0 } else { rounded };
        return Ok(format!("{} ms", number_to_string(clamped)));
    }
    Ok(format!("{} s", to_fixed_1(ms / 1000.0)))
}

/// Port of `probeWireLabel` (`src/provider-test.ts:38`).
pub fn probe_wire_label(wire: String) -> Result<String, String> {
    Ok(match wire.as_str() {
        "chat" => "Chat Completions",
        "anthropic" => "Anthropic Messages",
        _ => "Responses",
    }
    .into())
}

/// Port of `probeKeyHint` (`src/provider-test.ts:45`).
pub fn probe_key_hint(result: Value) -> Result<Option<Value>, String> {
    if is_nil(&result) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'keyTail')".into(),
        );
    }
    let Some(tail) = result.get("keyTail").filter(|v| is_truthy(v)).cloned() else {
        return Ok(None);
    };
    if let Some(name) = result.get("keyName").filter(|v| is_truthy(v)).cloned() {
        Ok(Some(hint(
            "本次用的是 Key「{0}」（末 4 位 {1}）。",
            vec![name, tail],
        )))
    } else {
        Ok(Some(hint("本次用的是末 4 位为 {0} 的 Key。", vec![tail])))
    }
}

/// Port of `probeHints` (`src/provider-test.ts:53`).
pub fn probe_hints(result: Value, account: Option<bool>) -> Result<Vec<Value>, String> {
    if is_nil(&result) {
        return Err("TypeError: Cannot read properties of null/undefined (reading 'ok')".into());
    }
    if result.get("ok").is_some_and(is_truthy) {
        return Ok(Vec::new());
    }
    let is_account = account.unwrap_or(false);
    let cat = result.get("category").and_then(Value::as_str).unwrap_or("");
    let msg = match cat {
        "subscription_policy" => "订阅额度策略阻止了测试：请检查使用上限和官方额度是否已刷新。",
        "auth" => {
            let first = if is_account {
                "登录凭据无效或已过期：请在详情底部点「重新登录」。"
            } else {
                "Key 无效、已过期，或没有这个模型的权限：核对 Key 是否填对，并在供应商后台确认它已启用。"
            };
            let mut hints = vec![hint(first, Vec::new())];
            if let Some(key_hint) = probe_key_hint(result)? {
                hints.push(key_hint);
            }
            return Ok(hints);
        }
        "not_found" => {
            "模型名不对，或上游没有这个模型：用「从上游获取」刷新模型列表，或换一个模型再试。"
        }
        "rate_limited" => {
            "被上游限速，或额度已用完：稍等一会儿再试，或到供应商后台查看用量与余额。"
        }
        "quota" => "额度或余额已用完：充值、等额度重置，或换一个 Key。",
        "timeout" => "等待超时：检查 Base URL 和代理是否正确可用；上游过忙时也会这样。",
        "connect" => {
            "连不上上游：检查 Base URL 和代理设置（这家供应商的代理或系统代理），确认网络能访问这个地址。"
        }
        "server" => "上游服务出错：多半是上游临时故障，稍后再试。",
        "stream" => "上游开始回复后又报错或断开：见下方的错误原文。",
        _ => "请求没有成功：见下方的错误原文。",
    };
    Ok(vec![hint(msg, Vec::new())])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_ms_and_wire_labels() {
        assert_eq!(format_probe_ms(None).unwrap(), "—");
        assert_eq!(format_probe_ms(Some(f64::NAN)).unwrap(), "—");
        assert_eq!(format_probe_ms(Some(850.4)).unwrap(), "850 ms");
        assert_eq!(format_probe_ms(Some(-0.4)).unwrap(), "0 ms");
        assert_eq!(format_probe_ms(Some(1234.0)).unwrap(), "1.2 s");
        assert_eq!(format_probe_ms(Some(1250.0)).unwrap(), "1.3 s");
        assert_eq!(probe_wire_label("chat".into()).unwrap(), "Chat Completions");
        assert_eq!(
            probe_wire_label("anthropic".into()).unwrap(),
            "Anthropic Messages"
        );
        assert_eq!(probe_wire_label("responses".into()).unwrap(), "Responses");
    }

    #[test]
    fn generates_failure_hints_and_errors_on_nil() {
        assert!(probe_hints(json!({ "ok": true }), None).unwrap().is_empty());
        let auth = probe_hints(
            json!({ "ok": false, "category": "auth", "keyName": "备用", "keyTail": "BBBB" }),
            None,
        )
        .unwrap();
        assert_eq!(auth.len(), 2);
        assert_eq!(
            auth[1],
            json!({ "text": "本次用的是 Key「{0}」（末 4 位 {1}）。", "args": ["备用", "BBBB"] })
        );
        assert_eq!(
            probe_hints(json!({ "ok": false, "category": "auth" }), Some(true))
                .unwrap()
                .len(),
            1
        );
        assert!(probe_key_hint(Value::Null).is_err());
        assert!(probe_hints(Value::Null, None).is_err());
    }
}
