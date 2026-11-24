//! Polling loop, message dispatch, and QR login flow ported from `codex_wechat.py`.

use crate::codex_client::CodexRunner;
use crate::config::{BridgeConfig, ILINK_BASE_URL, load_json_object, load_sync_buf, save_sync_buf};
use crate::protocol::{
    EP_GET_BOT_QR, EP_GET_QR_STATUS, EP_GET_UPDATES, api_get, api_post, extract_text,
    get_typing_ticket, handle_command, is_authorized, normalize_prompt, send_text, send_typing,
    strip_prefix, url_encode_component,
};
use anyhow::{Result, bail};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

fn unix_secs() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0.0, |d| d.as_secs_f64())
}

pub fn process_message(
    cfg: &BridgeConfig,
    runner: &CodexRunner,
    msg: &Value,
    seen: &mut HashMap<String, f64>,
) -> Result<()> {
    if msg.get("message_type").and_then(Value::as_i64).unwrap_or(1) != 1 {
        return Ok(());
    }
    let user_id = msg
        .get("from_user_id")
        .and_then(Value::as_str)
        .unwrap_or("");
    if user_id.is_empty() || user_id == cfg.account_id {
        return Ok(());
    }

    let msg_id = msg.get("message_id").and_then(Value::as_str).unwrap_or("");
    let now = unix_secs();
    seen.retain(|_, ts| now - *ts <= 300.0);
    if !msg_id.is_empty() {
        if seen.contains_key(msg_id) {
            return Ok(());
        }
        seen.insert(msg_id.to_string(), now);
    }

    let context_token = msg
        .get("context_token")
        .and_then(Value::as_str)
        .unwrap_or("");
    let item_list = msg
        .get("item_list")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let text = extract_text(item_list).trim().to_string();

    let cmd_reply = handle_command(cfg, user_id, &text);
    if !cmd_reply.is_empty()
        && (is_authorized(cfg, user_id) || cfg.allow_id_command_for_unauthorized)
    {
        send_text(cfg, user_id, &cmd_reply, context_token)?;
        return Ok(());
    }
    if !is_authorized(cfg, user_id) {
        return Ok(());
    }
    let (accepted, stripped) = strip_prefix(cfg, &text);
    if !accepted {
        return Ok(());
    }
    let prompt = normalize_prompt(&stripped, item_list);
    if prompt.is_empty() {
        send_text(cfg, user_id, "没有可处理的文本内容。", context_token)?;
        return Ok(());
    }

    let typing_ticket = if cfg.enable_typing {
        match get_typing_ticket(cfg, user_id, context_token) {
            Ok(ticket) => {
                let _ = send_typing(cfg, user_id, &ticket, 1);
                ticket
            }
            Err(_) => String::new(),
        }
    } else {
        String::new()
    };

    let reply = runner.run(user_id, &prompt);
    let send_res = send_text(cfg, user_id, &reply, context_token);
    if cfg.enable_typing && !typing_ticket.is_empty() {
        let _ = send_typing(cfg, user_id, &typing_ticket, 0);
    }
    send_res
}

pub fn poll_loop_with(cfg: &BridgeConfig, max_polls: Option<usize>) -> Result<()> {
    if cfg.token.is_empty() {
        bail!("缺少 token：请在配置里填写 token，或设置 WEIXIN_TOKEN / ILINK_TOKEN。");
    }
    fs::create_dir_all(&cfg.state_dir)?;
    let mut sync_buf = load_sync_buf(cfg);
    let mut seen: HashMap<String, f64> = HashMap::new();
    let mut failures: u32 = 0;
    let runner = CodexRunner::new(cfg.clone());

    let mut polls = 0_usize;
    loop {
        if max_polls.is_some_and(|m| polls >= m) {
            return Ok(());
        }
        polls += 1;

        match api_post(
            &cfg.base_url,
            EP_GET_UPDATES,
            json!({ "get_updates_buf": sync_buf }),
            &cfg.token,
            cfg.poll_timeout_seconds + 5,
        ) {
            Ok(resp) => {
                let ret_err = resp
                    .get("ret")
                    .is_some_and(|v| !v.is_null() && v.as_i64() != Some(0));
                let code_err = resp
                    .get("errcode")
                    .is_some_and(|v| !v.is_null() && v.as_i64() != Some(0));
                if ret_err || code_err {
                    failures += 1;
                    let delay = if failures >= cfg.max_consecutive_failures {
                        failures = 0;
                        cfg.backoff_delay_seconds
                    } else {
                        cfg.retry_delay_seconds
                    };
                    std::thread::sleep(Duration::from_secs(delay));
                    continue;
                }
                failures = 0;
                if let Some(nb) = resp.get("get_updates_buf").and_then(Value::as_str)
                    && !nb.is_empty()
                {
                    sync_buf = nb.to_string();
                    let _ = save_sync_buf(cfg, &sync_buf);
                }
                if let Some(msgs) = resp.get("msgs").and_then(Value::as_array) {
                    for msg in msgs {
                        if msg.is_object() {
                            let _ = process_message(cfg, &runner, msg, &mut seen);
                        }
                    }
                }
            }
            Err(_) => {
                failures += 1;
                let delay = if failures >= cfg.max_consecutive_failures {
                    failures = 0;
                    cfg.backoff_delay_seconds
                } else {
                    cfg.retry_delay_seconds
                };
                std::thread::sleep(Duration::from_secs(delay));
            }
        }
    }
}

pub fn qr_login(output: &Path, bot_type: &str, timeout_seconds: u64) -> Result<()> {
    qr_login_with(
        ILINK_BASE_URL,
        output,
        bot_type,
        timeout_seconds,
        "https",
        Duration::from_secs(1),
    )
}

pub fn qr_login_with(
    initial_base_url: &str,
    output: &Path,
    bot_type: &str,
    timeout_seconds: u64,
    redirect_scheme: &str,
    poll_interval: Duration,
) -> Result<()> {
    let q_endpoint = format!(
        "{EP_GET_BOT_QR}?bot_type={}",
        url_encode_component(bot_type)
    );
    let resp = api_get(initial_base_url, &q_endpoint, 35)?;
    let qrcode = resp
        .get("qrcode")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let qrcode_url = resp
        .get("qrcode_img_content")
        .and_then(Value::as_str)
        .unwrap_or("");
    if qrcode.is_empty() {
        bail!("获取二维码失败：响应里没有 qrcode。");
    }
    println!("请用微信扫描下面的二维码链接：\n{qrcode_url}\n");

    let deadline = Instant::now() + Duration::from_secs(timeout_seconds);
    let mut current_base = initial_base_url.to_string();
    while Instant::now() < deadline {
        let st_endpoint = format!(
            "{EP_GET_QR_STATUS}?qrcode={}",
            url_encode_component(&qrcode)
        );
        let status_resp = api_get(&current_base, &st_endpoint, 35)?;
        let status = status_resp
            .get("status")
            .and_then(Value::as_str)
            .unwrap_or("wait");
        match status {
            "wait" => {}
            "scaned" => println!("\n已扫码，请在微信中确认。"),
            "scaned_but_redirect" => {
                let host = status_resp
                    .get("redirect_host")
                    .and_then(Value::as_str)
                    .unwrap_or("");
                if !host.is_empty() {
                    current_base = format!("{redirect_scheme}://{host}");
                    println!("\n切换 iLink base_url：{current_base}");
                }
            }
            "confirmed" => {
                let token = status_resp
                    .get("bot_token")
                    .and_then(Value::as_str)
                    .unwrap_or("");
                let account_id = status_resp
                    .get("ilink_bot_id")
                    .and_then(Value::as_str)
                    .unwrap_or("");
                let base_url = status_resp
                    .get("baseurl")
                    .and_then(Value::as_str)
                    .filter(|s| !s.is_empty())
                    .unwrap_or(&current_base);
                let user_id = status_resp
                    .get("ilink_user_id")
                    .and_then(Value::as_str)
                    .unwrap_or("");
                if token.is_empty() || account_id.is_empty() {
                    bail!("扫码成功但没有拿到 bot_token/account_id。");
                }
                let mut config = if output.exists() {
                    load_json_object(output)?
                } else {
                    serde_json::Map::new()
                };
                config.insert("base_url".into(), json!(base_url));
                config.insert("token".into(), json!(token));
                config.insert("account_id".into(), json!(account_id));
                if !user_id.is_empty() {
                    config.insert("login_user_id".into(), json!(user_id));
                }
                if let Some(parent) = output.parent().filter(|p| !p.as_os_str().is_empty()) {
                    fs::create_dir_all(parent)?;
                }
                let rendered = format!(
                    "{}\n",
                    serde_json::to_string_pretty(&Value::Object(config))?
                );
                fs::write(output, rendered)?;
                println!("\n登录成功，配置已写入：{}", output.display());
                return Ok(());
            }
            "expired" => bail!("\n二维码已过期，请重新运行 login。"),
            _ => {}
        }
        std::thread::sleep(poll_interval);
    }
    bail!("\n登录超时。")
}
