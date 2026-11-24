//! Pure-Rust implementations of functions kept in `js_only` across Phase 4 WASM crates.

#![forbid(unsafe_code)]

use crate::relay_editing::{
    active_relay_profile, normalize_aggregate_relay_profile, sync_legacy_relay_fields,
    with_generated_relay_files,
};
use manager_common_rs::assets_data::translate_template;
use manager_common_rs::gateway_shared::{
    GROUP_MODEL_PREFIX, mask_account_name, member_status, provider_enabled,
};
use manager_common_rs::js_rt::{
    is_nil, is_truthy, js_number, js_str, js_trim, local_datetime_from_ms, u64_to_radix_36,
};
use manager_common_rs::relay_shared::{
    aggregate_member_candidates, flow_agent_str, flow_status_str, is_aggregate_profile,
    unique_key_id,
};
use serde_json::{Map, Value, json};
use std::collections::{HashMap, HashSet};

/// Port of `gateway-model.ts::unlistedModels`.
pub fn unlisted_models(payload: &Value) -> HashSet<String> {
    let mut hidden = HashSet::new();
    if is_nil(payload) || !is_truthy(payload) {
        return hidden;
    }
    let add_group = |group: &Value, hidden: &mut HashSet<String>| {
        let Some(members) = group.get("members").and_then(Value::as_array) else {
            return;
        };
        for member in members {
            let model = member
                .get("model")
                .and_then(Value::as_str)
                .map(js_trim)
                .unwrap_or("");
            if !model.is_empty() && !model.contains('*') && !model.starts_with(GROUP_MODEL_PREFIX) {
                hidden.insert(model.to_lowercase());
            }
        }
    };
    if let Some(groups) = payload.get("groups").and_then(Value::as_array) {
        for g in groups {
            let disabled = g.get("disabled").is_some_and(is_truthy);
            let hide = g.get("hideMembers").is_some_and(is_truthy);
            if !disabled && hide {
                add_group(g, &mut hidden);
            }
        }
    }
    let auto_models_on =
        payload.get("gateway").and_then(|gw| gw.get("autoModels")) != Some(&Value::Bool(false));
    if auto_models_on && let Some(auto_groups) = payload.get("autoGroups").and_then(Value::as_array)
    {
        for g in auto_groups {
            let hide = g.get("hideMembers").is_some_and(is_truthy);
            let is_hidden = g.get("hidden").is_some_and(is_truthy);
            if hide && !is_hidden {
                add_group(g, &mut hidden);
            }
        }
    }
    hidden
}

/// Port of `provider-meta-sync-model.ts::defaultPicked`.
pub fn default_picked(preview: &Value, kind: &str) -> HashSet<String> {
    let key = if kind == "windows" {
        "windows"
    } else {
        "reasoning"
    };
    preview
        .get(key)
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter(|c| !c.get("manual").is_some_and(is_truthy))
                .filter_map(|c| c.get("model").and_then(Value::as_str).map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

fn link_keys(entry: &Value) -> Option<(String, Option<String>)> {
    let agent = flow_agent_str(entry).ok()?;
    let a_key = format!("agent:{agent}>hub");
    let p_key = entry
        .get("relayId")
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .map(|id| format!("hub>provider:{id}"));
    Some((a_key, p_key))
}

/// Port of `request-flow-model.ts::busyLinks`.
pub fn busy_links(entries: &[Value]) -> HashSet<String> {
    let mut keys = HashSet::new();
    for entry in entries {
        if flow_status_str(entry).ok() != Some("running") {
            continue;
        }
        if let Some((a, p)) = link_keys(entry) {
            keys.insert(a);
            if let Some(pk) = p {
                keys.insert(pk);
            }
        }
    }
    keys
}

/// Port of `request-flow-model.ts::returningLinks`.
pub fn returning_links(entries: &[Value]) -> HashSet<String> {
    let mut keys = HashSet::new();
    for entry in entries {
        if flow_status_str(entry).ok() != Some("running") {
            continue;
        }
        let phase = entry.get("phase").and_then(Value::as_str);
        if matches!(phase, Some("retrying" | "waiting" | "requesting")) {
            continue;
        }
        let has_first_token = !entry.get("firstTokenMs").is_none_or(is_nil);
        let est_tokens = entry
            .get("estimatedOutputTokens")
            .and_then(Value::as_f64)
            .unwrap_or(0.0);
        if phase != Some("generating") && !has_first_token && est_tokens <= 0.0 {
            continue;
        }
        if let Some((a, p)) = link_keys(entry) {
            keys.insert(a);
            if let Some(pk) = p {
                keys.insert(pk);
            }
        }
    }
    keys
}

const ROUTE_MATCH_MS: f64 = 5_000.0;

/// Port of `hub-overview-model.ts::routeRecordsByEntry`.
pub fn route_records_by_entry(payload: &Value, entries: &[Value]) -> HashMap<String, Value> {
    let mut records = HashMap::new();
    let Some(agents) = payload
        .get("activity")
        .and_then(|a| a.get("agents"))
        .and_then(Value::as_array)
    else {
        return records;
    };
    for agent in agents {
        let Some(agent_id) = agent.get("agent").and_then(Value::as_str) else {
            continue;
        };
        let Some(record) = agent.get("last").filter(|v| !is_nil(v)) else {
            continue;
        };
        let finished_at = record
            .get("finishedAtMs")
            .and_then(Value::as_f64)
            .unwrap_or(f64::NAN);
        let best = entries
            .iter()
            .filter(|item| {
                flow_agent_str(item).ok().as_deref() == Some(agent_id)
                    && flow_status_str(item).ok() != Some("running")
            })
            .max_by(|l, r| {
                let lt = l.get("timestampMs").and_then(Value::as_f64).unwrap_or(0.0);
                let rt = r.get("timestampMs").and_then(Value::as_f64).unwrap_or(0.0);
                lt.partial_cmp(&rt).unwrap_or(std::cmp::Ordering::Equal)
            });
        if let Some(entry) = best {
            let ts = entry
                .get("timestampMs")
                .and_then(Value::as_f64)
                .unwrap_or(f64::NAN);
            if (ts - finished_at).abs() <= ROUTE_MATCH_MS
                && let Some(id) = entry.get("id").and_then(Value::as_str)
            {
                records.insert(id.to_string(), record.clone());
            }
        }
    }
    records
}

/// Port of `hub-overview-model.ts::hubProviders`.
pub fn hub_providers(
    payload: &Value,
    calls_by_provider: Option<&HashMap<String, f64>>,
    now_ms: f64,
) -> Result<Vec<Value>, String> {
    let Some(pool) = payload.get("pool").and_then(Value::as_array) else {
        return Ok(Vec::new());
    };
    let mut out = Vec::new();
    for entry in pool {
        if !provider_enabled(entry.clone())? {
            continue;
        }
        let id = js_str(entry.get("id").unwrap_or(&Value::Null));
        let name = mask_account_name(js_str(entry.get("name").unwrap_or(&Value::Null)))?;
        let status = member_status(payload.clone(), id.clone(), json!(now_ms))?;
        let calls = match calls_by_provider {
            Some(map) => js_number(map.get(&id).copied().unwrap_or(0.0)),
            None => Value::Null,
        };
        out.push(json!({
            "id": id,
            "name": name,
            "baseUrl": entry.get("baseUrl").cloned().unwrap_or(Value::String(String::new())),
            "keyCount": entry.get("keyCount").cloned().unwrap_or(json!(0)),
            "status": status,
            "calls": calls,
        }));
    }
    Ok(out)
}

/// Port of `provider-api-keys.ts::createRelayApiKey` with explicit `now_ms`.
pub fn create_relay_api_key_at(api_keys: &[Value], now_ms: u64) -> Value {
    let mut used: HashSet<String> = api_keys
        .iter()
        .filter_map(|e| e.get("id").and_then(Value::as_str).map(str::to_string))
        .collect();
    let base = format!("key-{}", u64_to_radix_36(now_ms));
    let id = unique_key_id(&base, &mut used);
    json!({
        "id": id,
        "name": format!("Key {}", api_keys.len() + 1),
        "apiKey": "",
    })
}

/// Port of `relay-profile-editing.ts::createRelayProfile` with explicit `now_ms`, baseUrl, and contextSelection.
pub fn create_relay_profile_at(
    settings: &Value,
    now_ms: u64,
    default_relay_base_url: &str,
    context_selection: Value,
) -> Result<Value, String> {
    let count = settings
        .get("relayProfiles")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    let id = format!("relay-{}", u64_to_radix_36(now_ms));
    let name = translate_template("供应商 {0}", &[json!(count + 1)], "zh");
    let next = json!({
        "id": id,
        "name": name,
        "model": "",
        "baseUrl": default_relay_base_url,
        "upstreamBaseUrl": default_relay_base_url,
        "apiKey": "",
        "protocol": "responses",
        "relayMode": "official",
        "sessionProvider": "custom",
        "officialMixApiKey": false,
        "hideOfficialUsageAlert": false,
        "testModel": "",
        "configContents": "",
        "authContents": "",
        "useCommonConfig": true,
        "contextSelection": context_selection,
        "contextSelectionInitialized": true,
        "contextWindow": "",
        "autoCompactLimit": "",
        "modelList": "",
        "modelWindows": "",
        "modelAutoCompact": "",
        "modelMetadata": "",
        "modelVlm": "",
        "vlmApiKey": "",
        "vlmModel": "",
        "vlmBaseUrl": "",
        "userAgent": "",
        "customHeaders": [],
        "sub2apiEnabled": false,
        "noAuth": false,
        "sub2apiMultiplier": "",
        "modelRoutes": [],
        "standardOpenaiProtocol": false,
        "webSearchHistoryCompat": false,
    });
    with_generated_relay_files(next)
}

/// Port of `relay-profile-editing.ts::createAggregateRelayProfile` with explicit `now_ms` & contextSelection.
pub fn create_aggregate_relay_profile_at(
    settings: &Value,
    now_ms: u64,
    context_selection: Value,
) -> Result<Value, String> {
    let profiles = settings
        .get("relayProfiles")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let agg_count = profiles
        .iter()
        .filter(|p| is_aggregate_profile(p).unwrap_or(false))
        .count();
    let id = format!("aggregate-{}", u64_to_radix_36(now_ms));
    let name = translate_template("聚合供应商 {0}", &[json!(agg_count + 1)], "zh");
    let cands = aggregate_member_candidates(profiles, &id)?;
    let first_member: Vec<Value> = cands
        .into_iter()
        .take(1)
        .filter_map(|p| {
            p.get("id")
                .and_then(Value::as_str)
                .map(|s| json!({ "profileId": s, "weight": 1 }))
        })
        .collect();
    let draft = json!({
        "id": id,
        "name": name,
        "model": "",
        "baseUrl": "",
        "upstreamBaseUrl": "",
        "apiKey": "",
        "protocol": "responses",
        "relayMode": "aggregate",
        "sessionProvider": "custom",
        "officialMixApiKey": false,
        "hideOfficialUsageAlert": false,
        "testModel": "",
        "configContents": "",
        "authContents": "",
        "useCommonConfig": true,
        "contextSelection": context_selection,
        "contextSelectionInitialized": true,
        "contextWindow": "",
        "autoCompactLimit": "",
        "modelList": "",
        "modelWindows": "",
        "modelAutoCompact": "",
        "modelMetadata": "",
        "modelVlm": "",
        "vlmApiKey": "",
        "vlmModel": "",
        "vlmBaseUrl": "",
        "userAgent": "",
        "customHeaders": [],
        "sub2apiEnabled": false,
        "noAuth": false,
        "sub2apiMultiplier": "",
        "modelRoutes": [],
        "standardOpenaiProtocol": false,
        "webSearchHistoryCompat": false,
        "aggregate": {
            "strategy": "failover",
            "modelAggregationEnabled": false,
            "members": first_member,
            "defaultRelayId": "",
        }
    });
    normalize_aggregate_relay_profile(draft, settings.clone())
}

/// Port of `relay-profile-editing.ts::duplicateRelayProfile` with explicit `now_ms`.
pub fn duplicate_relay_profile_at(
    settings: &Value,
    id: &str,
    now_ms: u64,
) -> Result<Value, String> {
    let profiles = settings
        .get("relayProfiles")
        .and_then(Value::as_array)
        .ok_or("TypeError: relayProfiles must be array")?;
    let source_idx = profiles
        .iter()
        .position(|p| p.get("id").and_then(Value::as_str) == Some(id));
    let source = match source_idx {
        Some(i) => profiles[i].clone(),
        None => active_relay_profile(settings.clone())?,
    };
    let next_id = format!("relay-{}", u64_to_radix_36(now_ms));
    let src_name = source
        .get("name")
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .unwrap_or("未命名供应商");
    let mut next_obj = source.as_object().cloned().unwrap_or_default();
    next_obj.insert("id".into(), Value::String(next_id));
    next_obj.insert(
        "name".into(),
        Value::String(translate_template("{0} 副本", &[json!(src_name)], "zh")),
    );
    let next = Value::Object(next_obj);
    let normalized = if is_aggregate_profile(&next)? {
        normalize_aggregate_relay_profile(next, settings.clone())?
    } else {
        next
    };
    let mut out_profiles = profiles.clone();
    let insert_at = source_idx.map_or(out_profiles.len(), |i| i + 1);
    out_profiles.insert(insert_at, normalized);
    let mut out_settings = settings.as_object().cloned().unwrap_or_default();
    out_settings.insert("relayProfiles".into(), Value::Array(out_profiles));
    sync_legacy_relay_fields(Value::Object(out_settings))
}

// ── update-notification / provider-balances / popover-layer / dream-skin ────

#[derive(Debug, Clone, Default)]
pub struct UpdateNotificationsState {
    pub dismissed_versions: HashSet<String>,
    pub muted_day: Option<String>,
    pub auto_check: bool,
    pub skipped_version: String,
    pub auto_download: bool,
    pub preferences_loaded: bool,
}

impl UpdateNotificationsState {
    pub fn new() -> Self {
        Self {
            auto_check: true,
            auto_download: true,
            ..Self::default()
        }
    }

    pub fn set_preferences(
        &mut self,
        auto_check: bool,
        skipped_version: &str,
        auto_download: bool,
    ) {
        self.auto_check = auto_check;
        self.skipped_version = skipped_version.trim().to_string();
        self.auto_download = auto_download;
        self.preferences_loaded = true;
    }

    pub fn dismiss(&mut self, version: &str) {
        self.dismissed_versions.insert(version.to_string());
    }

    pub fn skip_version(&mut self, version: &str) {
        self.skipped_version = version.trim().to_string();
    }

    pub fn mute_today_at(&mut self, now_ms: f64) {
        if let Some(dt) = local_datetime_from_ms(now_ms) {
            self.muted_day = Some(dt.format("%Y-%m-%d").to_string());
        }
    }

    pub fn should_remind(
        &self,
        status: &str,
        update_available: bool,
        latest_version: Option<&str>,
        now_ms: f64,
    ) -> bool {
        if status != "ok" || !update_available {
            return false;
        }
        let Some(latest) = latest_version.filter(|s| !s.is_empty()) else {
            return false;
        };
        if self.preferences_loaded && is_version_skipped(latest, &self.skipped_version) {
            return false;
        }
        if let Some(dt) = local_datetime_from_ms(now_ms) {
            let today = dt.format("%Y-%m-%d").to_string();
            if self.muted_day.as_deref() == Some(today.as_str()) {
                return false;
            }
        }
        true
    }

    pub fn should_prompt(
        &self,
        status: &str,
        update_available: bool,
        latest_version: Option<&str>,
        manual: bool,
        now_ms: f64,
    ) -> bool {
        if status != "ok" || !update_available || latest_version.is_none_or(str::is_empty) {
            return false;
        }
        if manual {
            return true;
        }
        if let Some(v) = latest_version
            && self.dismissed_versions.contains(v)
        {
            return false;
        }
        self.should_remind(status, update_available, latest_version, now_ms)
    }
}

fn version_parts(value: &str) -> Option<Vec<u64>> {
    let trimmed = js_trim(value);
    let after_v = trimmed.trim_start_matches(['v', 'V']);
    let digits_end = after_v
        .find(|c: char| !matches!(c, '0'..='9' | '.'))
        .unwrap_or(after_v.len());
    let digits = &after_v[..digits_end];
    if digits.is_empty() {
        return None;
    }
    digits
        .split('.')
        .map(|part| part.parse::<u64>().ok())
        .collect()
}

pub fn is_version_skipped(candidate: &str, skipped: &str) -> bool {
    if js_trim(skipped).is_empty() {
        return false;
    }
    match (version_parts(candidate), version_parts(skipped)) {
        (Some(l), Some(r)) => {
            for i in 0..l.len().max(r.len()) {
                let a = l.get(i).copied().unwrap_or(0);
                let b = r.get(i).copied().unwrap_or(0);
                if a != b {
                    return a < b;
                }
            }
            true
        }
        _ => js_trim(candidate) == js_trim(skipped),
    }
}

#[derive(Debug, Default)]
pub struct ProviderBalanceStore {
    pub balances: Option<Vec<Value>>,
    pub at_ms: f64,
}

impl ProviderBalanceStore {
    pub fn is_fresh(&self, now_ms: f64, ttl_ms: Option<f64>) -> bool {
        let Some(_) = self.balances else {
            return false;
        };
        match ttl_ms {
            None => true,
            Some(ttl) => now_ms - self.at_ms < ttl,
        }
    }

    pub fn update(&mut self, balances: Vec<Value>, now_ms: f64) {
        self.balances = Some(balances);
        self.at_ms = now_ms;
    }

    pub fn reset(&mut self) {
        self.balances = None;
        self.at_ms = 0.0;
    }
}

#[derive(Debug, Default)]
pub struct OpenPopoverRegistry {
    open_ids: Vec<String>,
}

impl OpenPopoverRegistry {
    pub fn register(&mut self, id: &str) -> Vec<String> {
        let others: Vec<String> = self
            .open_ids
            .iter()
            .filter(|other| other.as_str() != id)
            .cloned()
            .collect();
        self.open_ids.clear();
        self.open_ids.push(id.to_string());
        others
    }

    pub fn release(&mut self, id: &str) {
        self.open_ids.retain(|item| item != id);
    }

    pub fn count(&self) -> usize {
        self.open_ids.len()
    }

    pub fn reset(&mut self) {
        self.open_ids.clear();
    }
}

pub fn default_dream_skin_theme_for_ua(user_agent: Option<&str>) -> Value {
    let is_windows = user_agent.is_some_and(|ua| {
        ua.to_ascii_lowercase()
            .split(|c: char| !c.is_ascii_alphanumeric())
            .any(|w| w == "windows")
    });
    if is_windows {
        json!({
            "schemaVersion": 1,
            "id": "preset-arina-hashimoto",
            "name": "桥本有菜",
            "brandSubtitle": "CODEX DREAM SKIN",
            "tagline": "把柔光与玫瑰带进今天的工作台。",
            "projectPrefix": "选择项目 · ",
            "projectLabel": "◉  选择项目",
            "statusText": "DREAM SKIN ONLINE",
            "quote": "MAKE SOMETHING WONDERFUL",
            "image": "dream-reference.jpg",
            "appearance": "auto",
            "art": { "focusX": 0.72, "focusY": 0.45, "safeArea": "left", "taskMode": "ambient" }
        })
    } else {
        json!({
            "schemaVersion": 1,
            "id": "custom-1784123441349",
            "name": "Dream Skin",
            "brandSubtitle": "CODEX DREAM SKIN",
            "tagline": "把喜欢的画面变成可交互的 Codex 工作台。",
            "projectPrefix": "选择项目 · ",
            "projectLabel": "◉  选择项目",
            "statusText": "THEME ONLINE",
            "quote": "Make something wonderful",
            "colors": {
                "background": "#F7F4F5",
                "panel": "#FFFFFF",
                "panelAlt": "#FFF7F8",
                "accent": "#E25563",
                "accentAlt": "#F07A86",
                "secondary": "#F3A8AF",
                "highlight": "#C93D4C",
                "text": "#2B2224",
                "muted": "#8A7A7D",
                "line": "rgba(196, 120, 128, .22)"
            },
            "image": "portal-hero.png",
            "promoTitle": "感谢 Passion8 赞助",
            "promoSub": "passion8.cc",
            "promoUrl": "https://passion8.cc/register?aff=TuPe"
        })
    }
}

pub fn serialize_map_str_f64_summary(map: &Map<String, Value>) -> usize {
    map.len()
}
