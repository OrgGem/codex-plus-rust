//! Cross-platform CDP inspector injector unifying `inject-cn.js` and `inject-cn.ps1`.

use anyhow::{Context, Result, bail, ensure};
use regex::Regex;
use serde_json::{Value, json};
use std::fs;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};
use tungstenite::{Message, WebSocket};

pub fn clean_absolute_path(path: &Path) -> Result<PathBuf> {
    let abs = std::path::absolute(path)
        .with_context(|| format!("failed to resolve absolute path: {}", path.display()))?;
    let s = abs.to_string_lossy();
    if let Some(stripped) = s.strip_prefix(r"\\?\UNC\") {
        Ok(PathBuf::from(format!(r"\\{stripped}")))
    } else if let Some(stripped) = s.strip_prefix(r"\\?\") {
        Ok(PathBuf::from(stripped))
    } else {
        Ok(abs)
    }
}

#[derive(Clone, Debug)]
pub struct InjectConfig {
    pub app: PathBuf,
    pub hook: PathBuf,
    pub timeout_secs: u64,
    pub no_launch: bool,
    pub attach: bool,
    pub port: Option<u16>,
    pub user_data_dir: Option<PathBuf>,
    pub locale: String,
    pub voice_input: bool,
    pub localization: bool,
    pub bash_timeout_json: String,
    pub bash_report_file: Option<String>,
    pub bash_launch_id: Option<String>,
}

pub fn exec_path_for(app: &Path) -> PathBuf {
    if cfg!(windows) {
        app.to_path_buf()
    } else if app.extension().and_then(|e| e.to_str()) == Some("app") {
        app.join("Contents/MacOS/Claude")
    } else {
        app.to_path_buf()
    }
}

pub fn validate_locale(locale: &str) -> Result<()> {
    let re = Regex::new(r"^[a-z]{2,3}(?:-[A-Za-z0-9]{2,8})*$")?;
    ensure!(re.is_match(locale), "无效的语言代码: {locale}");
    Ok(())
}

pub fn free_port() -> Result<u16> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    Ok(listener.local_addr()?.port())
}

pub fn list_targets(port: u16) -> Result<Vec<Value>> {
    let url = format!("http://127.0.0.1:{port}/json/list");
    let resp = ureq::get(&url).timeout(Duration::from_secs(2)).call()?;
    let body = resp.into_string()?;
    let val: Value = serde_json::from_str(&body)?;
    Ok(val.as_array().cloned().unwrap_or_default())
}

pub fn wait_for_node_target(
    port: u16,
    timeout: Duration,
    mut child: Option<&mut Child>,
) -> Result<Option<String>> {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        if let Ok(list) = list_targets(port) {
            for item in list {
                if item.get("type").and_then(Value::as_str) == Some("node")
                    && let Some(ws_url) = item.get("webSocketDebuggerUrl").and_then(Value::as_str)
                {
                    return Ok(Some(ws_url.to_string()));
                }
            }
        }
        if let Some(c) = child.as_deref_mut()
            && let Some(status) = c.try_wait()?
        {
            bail!("副本进程已提前退出（状态: {status}），inspector 未出现");
        }
        thread::sleep(Duration::from_millis(500));
    }
    Ok(None)
}

struct CdpClient {
    ws: WebSocket<tungstenite::stream::MaybeTlsStream<TcpStream>>,
    next_id: u64,
}

impl CdpClient {
    fn connect(ws_url: &str) -> Result<Self> {
        let io_timeout = Duration::from_secs(30);
        if let Some(rest) = ws_url.strip_prefix("ws://") {
            let host_port = rest.split('/').next().unwrap_or(rest);
            if let Ok(addr) = host_port.parse::<SocketAddr>() {
                let stream = TcpStream::connect_timeout(&addr, io_timeout)
                    .with_context(|| format!("TCP connection failed: {ws_url}"))?;
                stream.set_read_timeout(Some(io_timeout))?;
                stream.set_write_timeout(Some(io_timeout))?;
                let (ws, _) =
                    tungstenite::client(ws_url, tungstenite::stream::MaybeTlsStream::Plain(stream))
                        .with_context(|| format!("WebSocket connection failed: {ws_url}"))?;
                return Ok(Self { ws, next_id: 0 });
            }
        }
        let (ws, _) = tungstenite::connect(ws_url)
            .with_context(|| format!("WebSocket connection failed: {ws_url}"))?;
        if let tungstenite::stream::MaybeTlsStream::Plain(stream) = ws.get_ref() {
            stream.set_read_timeout(Some(io_timeout))?;
            stream.set_write_timeout(Some(io_timeout))?;
        }
        Ok(Self { ws, next_id: 0 })
    }

    fn eval(&mut self, expression: &str, await_promise: bool) -> Result<Value> {
        self.next_id += 1;
        let req_id = self.next_id;
        let msg = json!({
            "id": req_id,
            "method": "Runtime.evaluate",
            "params": {
                "expression": expression,
                "returnByValue": true,
                "includeCommandLineAPI": true,
                "awaitPromise": await_promise,
            }
        });
        self.ws.send(Message::Text(msg.to_string()))?;

        loop {
            match self.ws.read()? {
                Message::Text(txt) => {
                    let Ok(parsed) = serde_json::from_str::<Value>(&txt) else {
                        continue;
                    };
                    if parsed.get("id").and_then(Value::as_u64) != Some(req_id) {
                        continue;
                    }
                    if let Some(err) = parsed.get("error") {
                        bail!("CDP error: {err}");
                    }
                    let result = parsed.get("result").cloned().unwrap_or(Value::Null);
                    if let Some(exc) = result.get("exceptionDetails") {
                        let desc = exc
                            .get("exception")
                            .and_then(|e| e.get("description"))
                            .and_then(Value::as_str)
                            .map(ToString::to_string)
                            .unwrap_or_else(|| exc.to_string());
                        bail!(
                            "页面/进程异常: {}",
                            desc.chars().take(300).collect::<String>()
                        );
                    }
                    return Ok(result
                        .get("result")
                        .and_then(|r| r.get("value"))
                        .cloned()
                        .unwrap_or(Value::Null));
                }
                Message::Close(_) => bail!("CDP connection closed"),
                _ => {}
            }
        }
    }

    fn eval_retry(&mut self, expression: &str, tries: usize) -> Result<Value> {
        let mut last = None;
        for i in 0..tries.max(1) {
            match self.eval(expression, false) {
                Ok(v) => return Ok(v),
                Err(e) => {
                    last = Some(e);
                    if i + 1 < tries {
                        thread::sleep(Duration::from_millis(2000));
                    }
                }
            }
        }
        Err(last.unwrap_or_else(|| anyhow::anyhow!("evaluate failed")))
    }

    fn close(mut self) {
        let _ = self.ws.close(None);
    }
}

pub fn build_bootstrap_script(cfg: &InjectConfig) -> Result<String> {
    let abs_hook = clean_absolute_path(&cfg.hook)?;
    let timeout_helper = abs_hook
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("bash-timeout.js");
    let raw_timeout = if cfg.bash_timeout_json.trim().is_empty() {
        "null"
    } else {
        cfg.bash_timeout_json.as_str()
    };
    let timeout_json = serde_json::to_string(raw_timeout)?;
    let helper_json = serde_json::to_string(&timeout_helper.to_string_lossy().into_owned())?;
    let report_json = serde_json::to_string(&json!({
        "file": cfg.bash_report_file,
        "launchId": cfg.bash_launch_id,
    }))?;
    Ok(format!(
        "(() => {{\n\
  try {{\n\
    globalThis.__claudeCnBashTimeout = JSON.parse({timeout_json});\n\
    require({helper_json}).beginTimeoutOverrides(globalThis.__claudeCnBashTimeout, process.resourcesPath, {report_json});\n\
  }} catch (error) {{\n\
    globalThis.__claudeCnBashTimeoutResult = {{ enabled: true, prepared: false, error: String(error.message || error) }};\n\
    globalThis.__claudeCnBashTimeoutReady = Promise.resolve(globalThis.__claudeCnBashTimeoutResult);\n\
  }}\n\
  return true;\n\
}})()"
    ))
}

pub fn build_hook_script(cfg: &InjectConfig) -> Result<String> {
    let abs_hook = clean_absolute_path(&cfg.hook)?;
    let locale_json = serde_json::to_string(&cfg.locale)?;
    let hook_json = serde_json::to_string(&abs_hook.to_string_lossy().into_owned())?;
    let voice = if cfg.voice_input { "true" } else { "false" };
    let loc = if cfg.localization { "true" } else { "false" };
    Ok(format!(
        "globalThis.__claudeCnTargetLocale = {locale_json}; \
globalThis.__claudeCnVoiceInput = {voice}; \
globalThis.__claudeCnLocalization = {loc}; \
require({hook_json}); \
globalThis.__claudeCnHookInstalled === true"
    ))
}

pub fn inject_into_ws(ws_url: &str, cfg: &InjectConfig, expected_exec: &Path) -> Result<()> {
    let mut client = CdpClient::connect(ws_url)?;
    let remote_exec_val = client.eval_retry("process.execPath", 4)?;
    let remote_exec = remote_exec_val
        .as_str()
        .context("process.execPath did not return string")?;

    let expected_canon = fs::canonicalize(expected_exec)?;
    let remote_canon = fs::canonicalize(remote_exec).with_context(|| {
        format!("failed to canonicalize remote process.execPath: {remote_exec}")
    })?;
    if remote_canon != expected_canon {
        client.close();
        bail!("✗ 目标进程不是这个副本（{remote_exec}），放弃注入。");
    }

    let bootstrap = build_bootstrap_script(cfg)?;
    let _ = client.eval(&bootstrap, false)?;
    let version = client
        .eval_retry("require('electron').app.getVersion()", 4)
        .unwrap_or(Value::Null);
    let pid = client.eval_retry("process.pid", 4).unwrap_or(Value::Null);
    println!("→ 已连上主进程（Claude {version}, pid {pid}）");

    let hook_expr = build_hook_script(cfg)?;
    let ok_flag = client.eval(&hook_expr, false)?;
    ensure!(
        ok_flag.as_bool() == Some(true),
        "✗ hook 没有装载成功: {}",
        cfg.hook.display()
    );
    println!("→ 主进程 hook 已装载，页面注入就绪");

    match client.eval("globalThis.__claudeCnBashTimeoutReady", true) {
        Ok(v) if !v.is_null() => println!("CODEX_PLUS_CLAUDE_BASH_RESULT={v}"),
        Ok(_) => eprintln!("提示：Cowork Bash 适配结果为空，其他补丁继续启动"),
        Err(e) => eprintln!("提示：Cowork Bash 适配结果不可用，其他补丁继续启动：{e}"),
    }

    let _ = client.eval(
        "setImmediate(()=>require('node:inspector').close()); 'closing'",
        false,
    );
    client.close();
    Ok(())
}

pub fn run_inject(cfg: &InjectConfig) -> Result<()> {
    validate_locale(&cfg.locale)?;
    let expected_exec = exec_path_for(&cfg.app);
    ensure!(
        expected_exec.exists(),
        "找不到副本：{}",
        expected_exec.display()
    );
    let abs_hook = clean_absolute_path(&cfg.hook)?;
    ensure!(abs_hook.exists(), "找不到 hook：{}", abs_hook.display());

    let mut launched_child: Option<Child> = None;
    let port = if !cfg.attach && !cfg.no_launch {
        let p = match cfg.port {
            Some(p) => p,
            None => free_port()?,
        };
        println!("→ 启动副本（inspector 端口 {p}）");
        if cfg!(windows) {
            let mut cmd = Command::new(&expected_exec);
            cmd.arg(format!("--inspect=127.0.0.1:{p}"));
            if let Some(prof) = &cfg.user_data_dir {
                cmd.arg(format!("--user-data-dir={}", prof.display()));
            }
            let child = cmd
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()?;
            launched_child = Some(child);
        } else {
            let mut args = vec![
                "-n".to_string(),
                "-a".to_string(),
                cfg.app.to_string_lossy().into_owned(),
            ];
            if let Ok(plugin_dirs) = std::env::var("CLAUDE_CODE_PLUGIN_DIRS")
                && !plugin_dirs.is_empty()
            {
                args.push("--env".to_string());
                args.push(format!("CLAUDE_CODE_PLUGIN_DIRS={plugin_dirs}"));
            }
            args.push("--args".to_string());
            args.push(format!("--inspect=127.0.0.1:{p}"));
            if let Some(prof) = &cfg.user_data_dir {
                args.push(format!("--user-data-dir={}", prof.display()));
            }
            let status = Command::new("open").args(&args).status()?;
            ensure!(status.success(), "open failed");
        }
        p
    } else {
        cfg.port.unwrap_or(9411)
    };

    println!(
        "→ 等待 inspector（127.0.0.1:{port}，最多 {}s）",
        cfg.timeout_secs
    );
    let ws_url = wait_for_node_target(
        port,
        Duration::from_secs(cfg.timeout_secs),
        launched_child.as_mut(),
    )?
    .context("inspector 没出现（副本未启动 / fuse 未开启 / 端口被占）")?;

    inject_into_ws(&ws_url, cfg, &expected_exec)?;
    println!("启动注入已完成。");
    Ok(())
}
