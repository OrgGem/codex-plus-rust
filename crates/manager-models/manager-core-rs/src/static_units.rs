//! Pure-Rust ports of the 12 non-wire / static / callback / DOM-logic manager units:
//! `presets`, `i18n-en`, `mcp-presets`, `vlm-test-translation`, `flow-provider-colors`,
//! `skill-source-scan`, `escape-layer`, `mac-titlebar`, `provider-brand-icons`,
//! `settings-events`, `lib/utils`, and `legacy-webkit`.

#![forbid(unsafe_code)]

use manager_common_rs::assets_data::{
    I18nDict, i18n_en, presets_value, translate_plain, translate_template,
};
use manager_common_rs::js_rt::{js_str, js_trim};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use std::collections::{HashMap, HashSet};

pub const BACKEND_SETTINGS_CHANGED_EVENT: &str = "codex-plus:backend-settings-changed";
pub const MAC_TITLEBAR_CLASS: &str = "mac-titlebar";

// ── 1. presets.ts ───────────────────────────────────────────────────────────

pub fn presets() -> &'static Value {
    presets_value()
}

// ── 2. i18n-en.ts ───────────────────────────────────────────────────────────

pub fn en_dict() -> &'static I18nDict {
    i18n_en()
}

pub fn translate_en(zh: &str) -> String {
    translate_plain(zh, "en")
}

pub fn translate_template_en(key: &str, args: &[Value]) -> String {
    translate_template(key, args, "en")
}

// ── 3. mcp-presets.ts ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct McpPresetMeta {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub homepage: String,
}

pub fn mcp_presets() -> Vec<McpPresetMeta> {
    vec![
        McpPresetMeta {
            id: "fetch".into(),
            name: "mcp-server-fetch".into(),
            description: "抓取网页并转成适合模型阅读的文本。需要本机有 uvx（uv）。".into(),
            tags: vec!["stdio".into(), "web".into()],
            homepage: "https://github.com/modelcontextprotocol/servers/tree/main/src/fetch".into(),
        },
        McpPresetMeta {
            id: "time".into(),
            name: "@modelcontextprotocol/server-time".into(),
            description: "查询当前时间并做时区换算。".into(),
            tags: vec!["stdio".into(), "utility".into()],
            homepage: "https://github.com/modelcontextprotocol/servers/tree/main/src/time".into(),
        },
        McpPresetMeta {
            id: "memory".into(),
            name: "@modelcontextprotocol/server-memory".into(),
            description: "基于知识图谱的长期记忆，跨会话保留事实。".into(),
            tags: vec!["stdio".into(), "memory".into()],
            homepage: "https://github.com/modelcontextprotocol/servers/tree/main/src/memory".into(),
        },
        McpPresetMeta {
            id: "sequential-thinking".into(),
            name: "@modelcontextprotocol/server-sequential-thinking".into(),
            description: "把复杂问题拆成可回溯的思考步骤。".into(),
            tags: vec!["stdio".into(), "reasoning".into()],
            homepage:
                "https://github.com/modelcontextprotocol/servers/tree/main/src/sequentialthinking"
                    .into(),
        },
        McpPresetMeta {
            id: "context7".into(),
            name: "@upstash/context7-mcp".into(),
            description: "按需拉取库和框架的最新文档，减少版本过时的回答。".into(),
            tags: vec!["stdio".into(), "docs".into()],
            homepage: "https://context7.com".into(),
        },
    ]
}

pub fn mcp_preset_by_id(id: &str) -> Option<McpPresetMeta> {
    mcp_presets().into_iter().find(|preset| preset.id == id)
}

fn stdio_toml_body(package_name: &str, windows: bool) -> String {
    let (cmd, args): (&str, Vec<&str>) = if windows {
        ("cmd", vec!["/c", "npx", "-y", package_name])
    } else {
        ("npx", vec!["-y", package_name])
    };
    let quoted_args: Vec<String> = args
        .into_iter()
        .map(|v| serde_json::to_string(v).unwrap_or_else(|_| "\"\"".into()))
        .collect();
    let quoted_cmd = serde_json::to_string(cmd).unwrap_or_else(|_| "\"\"".into());
    format!(
        "command = {quoted_cmd}\nargs = [{}]\n",
        quoted_args.join(", ")
    )
}

pub fn mcp_preset_toml_body(id: &str, windows: bool) -> Option<String> {
    match id {
        "fetch" => Some("command = \"uvx\"\nargs = [\"mcp-server-fetch\"]\n".into()),
        "time" => Some(stdio_toml_body(
            "@modelcontextprotocol/server-time",
            windows,
        )),
        "memory" => Some(stdio_toml_body(
            "@modelcontextprotocol/server-memory",
            windows,
        )),
        "sequential-thinking" => Some(stdio_toml_body(
            "@modelcontextprotocol/server-sequential-thinking",
            windows,
        )),
        "context7" => Some(stdio_toml_body("@upstash/context7-mcp", windows)),
        _ => None,
    }
}

// ── 4. vlm-test-translation.ts ──────────────────────────────────────────────

pub fn vlm_test_translation(
    status: &str,
    http_code: Option<i64>,
    duration_ms: f64,
    tr: impl Fn(&str, &[&str]) -> String,
) -> String {
    let secs = format!("{:.1}", duration_ms / 1000.0);
    match status {
        "ok" => tr("✅ 识别成功（耗时 {0}s）", &[&secs]),
        "http_error" => match http_code {
            Some(code @ (401 | 403)) => {
                let c = code.to_string();
                tr("❌ 认证失败（HTTP {0}）：API Key 或模型名可能不正确", &[&c])
            }
            Some(404) => tr("❌ 接口不存在：Base URL 或模型名有误（HTTP 404）", &[]),
            Some(429) => tr("❌ 被限流，稍后再试（HTTP 429）", &[]),
            other => {
                let c = other.map_or_else(|| "?".to_string(), |n| n.to_string());
                tr("❌ 服务返回错误（HTTP {0}）", &[&c])
            }
        },
        "timeout" => tr("❌ 请求超时：VLM 响应过慢或网络不通", &[]),
        "send_error" => tr("❌ 发送失败：网络错误，检查 Base URL 是否可达", &[]),
        "json_error" => tr("❌ 返回内容解析失败：上游返回的不是有效 JSON", &[]),
        "no_text" => tr(
            "❌ 返回中未找到描述文本：模型可能不支持视觉或返回格式异常",
            &[],
        ),
        "parse_error" => tr("❌ 批量描述解析失败（单图测试不应触发）", &[]),
        "client_error" => tr("❌ HTTP 客户端构建失败", &[]),
        "invalid_image" => tr("❌ 测试图片无效：仅支持图片文件，且不超过 10MB", &[]),
        _ => tr("❌ 未知错误", &[]),
    }
}

pub fn vlm_test_translation_zh(status: &str, http_code: Option<i64>, duration_ms: f64) -> String {
    vlm_test_translation(status, http_code, duration_ms, |tmpl, params| {
        let vals: Vec<Value> = params
            .iter()
            .map(|s| Value::String((*s).to_string()))
            .collect();
        translate_template(tmpl, &vals, "zh")
    })
}

// ── 5. flow-provider-colors.ts ──────────────────────────────────────────────

#[derive(Debug, Default, Clone)]
pub struct FlowProviderColorStore {
    pub slots: Vec<(String, i32)>,
}

impl FlowProviderColorStore {
    pub fn from_json(raw: &str) -> Self {
        let mut store = Self::default();
        let Ok(Value::Array(items)) = serde_json::from_str::<Value>(raw) else {
            return store;
        };
        let mut used_primary = HashSet::new();
        for item in items {
            let Some(arr) = item.as_array() else {
                continue;
            };
            let Some(Value::String(id)) = arr.first() else {
                continue;
            };
            let Some(slot_f) = arr.get(1).and_then(Value::as_f64) else {
                continue;
            };
            if slot_f.fract() != 0.0 || !(0.0..=3.0).contains(&slot_f) {
                continue;
            }
            let slot = slot_f as i32;
            if slot == 3 || !used_primary.contains(&slot) {
                used_primary.insert(slot);
                if let Some(pos) = store.slots.iter().position(|(k, _)| k == id) {
                    store.slots[pos].1 = slot;
                } else {
                    store.slots.push((id.clone(), slot));
                }
            }
        }
        store
    }

    pub fn assign(&mut self, id: &str) -> i32 {
        assign_provider_slot(&mut self.slots, id)
    }

    pub fn to_json(&self) -> String {
        let pairs: Vec<Value> = self.slots.iter().map(|(k, v)| json!([k, v])).collect();
        serde_json::to_string(&pairs).unwrap_or_else(|_| "[]".into())
    }
}

pub fn assign_provider_slot(registry: &mut Vec<(String, i32)>, id: &str) -> i32 {
    if id.is_empty() {
        return -1;
    }
    if let Some((_, slot)) = registry.iter().find(|(k, _)| k == id) {
        return *slot;
    }
    let used: HashSet<i32> = registry.iter().map(|(_, v)| *v).collect();
    let slot = [0, 1, 2]
        .into_iter()
        .find(|v| !used.contains(v))
        .unwrap_or(3);
    registry.push((id.to_string(), slot));
    slot
}

// ── 6. skill-source-scan.ts ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SkillRepoRef {
    pub owner: String,
    pub name: String,
    pub branch: String,
    pub subdir: String,
    #[serde(default)]
    pub enabled: bool,
}

pub fn skill_source_repo_key(repo: &SkillRepoRef) -> String {
    if repo.subdir.is_empty() {
        format!("{}/{}@{}", repo.owner, repo.name, repo.branch)
    } else {
        format!(
            "{}/{}@{}:{}",
            repo.owner, repo.name, repo.branch, repo.subdir
        )
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SkillSourceScanStatus {
    Idle,
    Scanning,
    Paused,
    Complete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillSourceScanState {
    pub status: SkillSourceScanStatus,
    pub repos: Vec<SkillRepoRef>,
    pub current_repo: String,
    pub candidates: Vec<Value>,
    pub warnings: Vec<String>,
    pub progress: usize,
    pub evidence: Vec<Value>,
    pub evidence_errors: Vec<String>,
    pub evidence_scanning: bool,
    pub consumed_local_ids: HashSet<String>,
    pub generation: u64,
}

impl Default for SkillSourceScanState {
    fn default() -> Self {
        Self {
            status: SkillSourceScanStatus::Idle,
            repos: Vec::new(),
            current_repo: String::new(),
            candidates: Vec::new(),
            warnings: Vec::new(),
            progress: 0,
            evidence: Vec::new(),
            evidence_errors: Vec::new(),
            evidence_scanning: false,
            consumed_local_ids: HashSet::new(),
            generation: 0,
        }
    }
}

impl SkillSourceScanState {
    pub fn ensure(&mut self, repos: Vec<SkillRepoRef>, local_ids: &[String]) {
        if self.status == SkillSourceScanStatus::Idle {
            self.restart(repos, local_ids);
        } else {
            for repo in repos {
                self.add_repo(repo);
            }
        }
    }

    pub fn restart(&mut self, repos: Vec<SkillRepoRef>, local_ids: &[String]) {
        self.generation += 1;
        self.consumed_local_ids.clear();
        self.status = if repos.is_empty() && local_ids.is_empty() {
            SkillSourceScanStatus::Complete
        } else {
            SkillSourceScanStatus::Scanning
        };
        self.repos = repos;
        self.current_repo.clear();
        self.candidates.clear();
        self.warnings.clear();
        self.progress = 0;
        self.evidence.clear();
        self.evidence_errors.clear();
        self.evidence_scanning = !local_ids.is_empty();
    }

    pub fn add_repo(&mut self, repo: SkillRepoRef) {
        let key = skill_source_repo_key(&repo).to_lowercase();
        if self
            .repos
            .iter()
            .any(|item| skill_source_repo_key(item).to_lowercase() == key)
        {
            return;
        }
        self.repos.push(repo);
        if self.status == SkillSourceScanStatus::Complete {
            self.status = SkillSourceScanStatus::Scanning;
        }
    }

    pub fn pause(&mut self) {
        if self.status == SkillSourceScanStatus::Scanning {
            self.status = SkillSourceScanStatus::Paused;
        }
    }

    pub fn resume(&mut self) {
        if self.status == SkillSourceScanStatus::Paused {
            self.status = SkillSourceScanStatus::Scanning;
        }
    }

    pub fn consume(&mut self, local_ids: &[String]) {
        for id in local_ids {
            self.consumed_local_ids.insert(id.clone());
        }
        self.candidates.retain(|c| {
            c.get("localId")
                .and_then(Value::as_str)
                .is_none_or(|id| !self.consumed_local_ids.contains(id))
        });
    }

    pub fn dispose(&mut self) {
        self.generation += 1;
        self.status = SkillSourceScanStatus::Idle;
    }
}

// ── 7. escape-layer.ts ──────────────────────────────────────────────────────

#[derive(Debug, Default)]
pub struct EscapeLayerStack {
    next_token: u64,
    layers: Vec<(u64, String)>,
}

impl EscapeLayerStack {
    pub fn push(&mut self, label: impl Into<String>) -> u64 {
        self.next_token += 1;
        let token = self.next_token;
        self.layers.push((token, label.into()));
        token
    }

    pub fn unregister(&mut self, token: u64) {
        if let Some(idx) = self.layers.iter().rposition(|(t, _)| *t == token) {
            self.layers.remove(idx);
        }
    }

    /// Returns `Some(closed_label)` when `key == "Escape"` and `!is_composing` and a layer exists.
    pub fn handle_escape_key(&mut self, key: &str, is_composing: bool) -> Option<String> {
        if key != "Escape" || is_composing {
            return None;
        }
        self.layers.last().map(|(_, label)| label.clone())
    }

    pub fn len(&self) -> usize {
        self.layers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.layers.is_empty()
    }

    pub fn reset(&mut self) {
        self.layers.clear();
    }
}

// ── 8. mac-titlebar.ts ──────────────────────────────────────────────────────

pub fn uses_mac_overlay_titlebar(
    search: &str,
    is_tauri: bool,
    platform: &str,
    user_agent: &str,
) -> bool {
    let query = search.strip_prefix('?').unwrap_or(search);
    for pair in query.split('&') {
        if let Some((k, v)) = pair.split_once('=')
            && k == "macTitlebar"
            && v == "1"
        {
            return true;
        }
    }
    if !is_tauri {
        return false;
    }
    let combined = format!("{platform} {user_agent}");
    combined.to_ascii_lowercase().contains("mac")
}

// ── 9. provider-brand-icons.ts ──────────────────────────────────────────────

pub const BRAND_ICON_KEYS: &[&str] = &[
    "aihubmix-color",
    "antigravity-color",
    "azure-color",
    "baiducloud-color",
    "bailian-color",
    "claude-color",
    "claudecode-color",
    "cline",
    "codebuddy-color",
    "commandcode",
    "cursor",
    "deepseek-color",
    "devin-color",
    "doubao-color",
    "fireworks-color",
    "gemini-color",
    "geminicli-color",
    "githubcopilot",
    "google-color",
    "goose",
    "grok",
    "groq",
    "hermesagent",
    "huawei-color",
    "huggingface-color",
    "hunyuan-color",
    "jetbrains",
    "kimi-color",
    "kiro-color",
    "lmstudio",
    "longcat-color",
    "minimax-color",
    "mistral-color",
    "modelscope-color",
    "newapi-color",
    "novita-color",
    "nvidia-color",
    "ollama",
    "openai",
    "openclaw-color",
    "opencode",
    "openrouter",
    "perplexity-color",
    "qoder-color",
    "qwen-color",
    "raycast",
    "siliconcloud-color",
    "stepfun-color",
    "tencentcloud-color",
    "together-color",
    "vertexai-color",
    "volcengine-color",
    "vscode-color",
    "vscode-insiders-color",
    "vscodium",
    "workbuddy-color",
    "xai",
    "xiaomimimo",
    "zai",
    "zed",
    "zhipu-color",
];

pub fn brand_icon_asset_filename(key: &str) -> Option<String> {
    BRAND_ICON_KEYS
        .contains(&key)
        .then(|| format!("assets/providers/{key}.svg"))
}

pub fn brand_icon_asset_map() -> Map<String, Value> {
    let mut map = Map::new();
    for key in BRAND_ICON_KEYS {
        map.insert(
            (*key).into(),
            Value::String(format!("assets/providers/{key}.svg")),
        );
    }
    map
}

// ── 11. lib/utils.ts (`cn`) ─────────────────────────────────────────────────

pub fn cn_classes(inputs: &[&str]) -> String {
    let mut tokens: Vec<&str> = Vec::new();
    for input in inputs {
        for tok in input.split(char::is_whitespace).filter(|t| !t.is_empty()) {
            if let Some(pos) = tokens.iter().position(|existing| *existing == tok) {
                tokens.remove(pos);
            }
            tokens.push(tok);
        }
    }
    tokens.join(" ")
}

// ── 12. legacy-webkit.ts helper ─────────────────────────────────────────────

pub fn resolve_relative_index(len: usize, index: f64) -> Option<usize> {
    if len == 0 || index.is_infinite() {
        return None;
    }
    if index.is_nan() {
        return Some(0);
    }
    let trunc = index.trunc();
    if trunc < 0.0 {
        if trunc < isize::MIN as f64 {
            return None;
        }
        len.checked_add_signed(trunc as isize)
            .filter(|&pos| pos < len)
    } else {
        if trunc >= len as f64 || trunc > usize::MAX as f64 {
            return None;
        }
        Some(trunc as usize)
    }
}

pub fn value_trim_str(v: &Value) -> String {
    js_trim(&js_str(v)).to_string()
}

pub fn get_i18n_keys_counts() -> HashMap<&'static str, usize> {
    let d = en_dict();
    HashMap::from([
        ("plain", d.plain.len()),
        ("template", d.template.len()),
        ("backend", d.backend.len()),
        ("backendPatterns", d.backend_patterns.len()),
    ])
}
