//! ThreadFollower state mirror and Immer patch applier ported from `follower.mjs`.

use crate::ipc_client::{IpcClient, uuid_v4};
use anyhow::{Result, bail};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};

pub fn apply_patches(mut state: Value, patches: &[Value]) -> Result<Value> {
    for patch in patches {
        let op = patch.get("op").and_then(Value::as_str).unwrap_or("");
        let path = patch
            .get("path")
            .and_then(Value::as_array)
            .ok_or_else(|| anyhow::anyhow!("patch missing path array"))?;
        let value = patch.get("value").cloned().unwrap_or(Value::Null);
        if path.is_empty() {
            if op != "remove" {
                state = value;
            }
            continue;
        }
        let mut parent = &mut state;
        for i in 0..path.len() - 1 {
            let next_is_arr = path[i + 1].is_number();
            let fill = || if next_is_arr { json!([]) } else { json!({}) };
            match parent {
                Value::Array(a) => {
                    let idx = seg_idx(&path[i])?;
                    while a.len() <= idx {
                        a.push(Value::Null);
                    }
                    if a[idx].is_null() {
                        a[idx] = fill();
                    }
                    parent = &mut a[idx];
                }
                Value::Object(o) => {
                    let k = seg_str(&path[i]);
                    let entry = o.entry(k).or_insert_with(fill);
                    if entry.is_null() {
                        *entry = fill();
                    }
                    parent = entry;
                }
                _ => bail!("cannot traverse non-container"),
            }
        }
        let last = &path[path.len() - 1];
        match parent {
            Value::Array(a) => {
                if seg_str(last) == "length" && op != "remove" {
                    let n = value.as_u64().unwrap_or(0) as usize;
                    a.resize(n, Value::Null);
                } else {
                    let idx = seg_idx(last)?;
                    if op == "remove" {
                        if idx < a.len() {
                            a.remove(idx);
                        }
                    } else if op == "add" {
                        let pos = idx.min(a.len());
                        a.insert(pos, value);
                    } else {
                        while a.len() <= idx {
                            a.push(Value::Null);
                        }
                        a[idx] = value;
                    }
                }
            }
            Value::Object(o) => {
                let k = seg_str(last);
                if op == "remove" {
                    o.shift_remove(&k);
                } else {
                    o.insert(k, value);
                }
            }
            _ => bail!("cannot mutate non-container"),
        }
    }
    Ok(state)
}

fn seg_str(v: &Value) -> String {
    v.as_str().map_or_else(|| v.to_string(), str::to_string)
}

fn seg_idx(v: &Value) -> Result<usize> {
    v.as_u64()
        .map(|n| n as usize)
        .or_else(|| v.as_str()?.parse::<usize>().ok())
        .ok_or_else(|| anyhow::anyhow!("invalid array index {v}"))
}

#[derive(Default)]
pub struct FollowerState {
    pub state: Option<Value>,
    pub revision: Option<Value>,
    pub owner: Option<String>,
    pub gap_events: Vec<(Value, Value)>,
}

pub struct ThreadFollower {
    ipc: Arc<IpcClient>,
    pub thread_id: String,
    pub host_id: String,
    pub inner: Arc<Mutex<FollowerState>>,
}

impl ThreadFollower {
    pub fn attach(
        ipc: Arc<IpcClient>,
        thread_id: impl Into<String>,
        host_id: impl Into<String>,
    ) -> Self {
        let tid = thread_id.into();
        let inner = Arc::new(Mutex::new(FollowerState::default()));
        let inner_cb = Arc::clone(&inner);
        let ipc_cb = Arc::clone(&ipc);
        let tid_cb = tid.clone();

        ipc.on_method(
            "thread-stream-state-changed",
            Arc::new(move |msg| {
                let Some(params) = msg.get("params") else {
                    return;
                };
                if params.get("conversationId").and_then(Value::as_str) != Some(tid_cb.as_str()) {
                    return;
                }
                let Some(change) = params.get("change") else {
                    return;
                };
                let Ok(mut st) = inner_cb.lock() else { return };
                if change.get("type").and_then(Value::as_str) == Some("snapshot") {
                    st.state = change.get("conversationState").cloned();
                    st.revision = change.get("revision").cloned();
                    st.owner = msg
                        .get("sourceClientId")
                        .and_then(Value::as_str)
                        .map(str::to_string);
                } else {
                    let base = change.get("baseRevision");
                    if st.state.is_none() || base != st.revision.as_ref() {
                        let have = st.revision.clone().unwrap_or(Value::Null);
                        st.gap_events
                            .push((have, base.cloned().unwrap_or(Value::Null)));
                        let owner = st.owner.clone();
                        drop(st);
                        let _ = ipc_cb.request(
                            "thread-follower-load-complete-history",
                            json!({ "conversationId": tid_cb }),
                            owner.as_deref(),
                            None,
                            8000,
                        );
                        return;
                    }
                    let patches = change
                        .get("patches")
                        .and_then(Value::as_array)
                        .map(Vec::as_slice)
                        .unwrap_or(&[]);
                    if let Some(cur) = st.state.take()
                        && let Ok(next) = apply_patches(cur, patches)
                    {
                        st.state = Some(next);
                        st.revision = change.get("revision").cloned();
                    }
                }
            }),
        );

        Self {
            ipc,
            thread_id: tid,
            host_id: host_id.into(),
            inner,
        }
    }

    pub fn start(&self) -> Result<()> {
        let owner = self.ipc.request(
            "thread-owner-discovery",
            json!({ "hostId": self.host_id, "conversationId": self.thread_id }),
            None,
            None,
            10_000,
        )?;
        if owner.get("resultType").and_then(Value::as_str) != Some("success") {
            bail!("no owner: {owner}");
        }
        let owner_id = owner
            .get("handledByClientId")
            .and_then(Value::as_str)
            .map(str::to_string);
        if let Ok(mut g) = self.inner.lock() {
            g.owner = owner_id;
        }
        self.ipc.broadcast(
            "thread-stream-following-changed",
            json!({ "conversationId": self.thread_id, "hostId": self.host_id, "following": true }),
            None,
        )?;
        let _ = self.resync()?;
        Ok(())
    }

    pub fn resync(&self) -> Result<Value> {
        let owner = self.inner.lock().ok().and_then(|g| g.owner.clone());
        self.ipc.request(
            "thread-follower-load-complete-history",
            json!({ "conversationId": self.thread_id }),
            owner.as_deref(),
            None,
            8000,
        )
    }

    pub fn stop(&self) -> Result<()> {
        self.ipc.broadcast(
            "thread-stream-following-changed",
            json!({ "conversationId": self.thread_id, "hostId": self.host_id, "following": false }),
            None,
        )
    }

    pub fn call(&self, method: &str, mut params: Value, timeout_ms: u64) -> Result<Value> {
        let owner = self.inner.lock().ok().and_then(|g| g.owner.clone());
        if let Some(obj) = params.as_object_mut() {
            obj.insert("conversationId".into(), json!(self.thread_id));
        }
        self.ipc
            .request(method, params, owner.as_deref(), None, timeout_ms)
    }

    pub fn turns(&self) -> Vec<Value> {
        let Ok(g) = self.inner.lock() else {
            return Vec::new();
        };
        let Some(state) = &g.state else {
            return Vec::new();
        };
        if let Some(history) = state.get("turnHistory").and_then(|t| t.get("history")) {
            let entities = history.get("entitiesByKey").unwrap_or(&Value::Null);
            let mut out = Vec::new();
            if let Some(islands) = history.get("islands").and_then(Value::as_array) {
                for island in islands {
                    if let Some(entries) = island.get("entries").and_then(Value::as_array) {
                        for entry in entries {
                            if let Some(k) = entry.get("value").and_then(Value::as_str)
                                && let Some(v) = entities.get(k)
                            {
                                out.push(v.clone());
                            }
                        }
                    }
                }
            }
            return out;
        }
        state
            .get("turns")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default()
    }

    pub fn start_turn(&self, text: &str, client_user_message_id: Option<&str>) -> Result<Value> {
        let mid = client_user_message_id.map_or_else(uuid_v4, str::to_string);
        self.call(
            "thread-follower-start-turn",
            json!({
                "turnStart": {
                    "request": { "threadId": self.thread_id, "input": [{ "type": "text", "text": text, "text_elements": [] }], "clientUserMessageId": mid },
                    "context": {}
                }
            }),
            20_000,
        )
    }

    pub fn steer(
        &self,
        text: &str,
        cwd: &str,
        client_user_message_id: Option<&str>,
    ) -> Result<Value> {
        let mid = client_user_message_id.map_or_else(uuid_v4, str::to_string);
        self.call(
            "thread-follower-steer-turn",
            json!({
                "input": [{ "type": "text", "text": text, "text_elements": [] }],
                "clientUserMessageId": mid,
                "restoreMessage": {
                    "id": mid,
                    "text": text,
                    "cwd": cwd,
                    "context": { "workspaceRoots": [cwd], "commentAttachments": [], "fileAttachments": [], "collaborationMode": null }
                },
                "attachments": [],
                "serviceTier": null
            }),
            20_000,
        )
    }

    pub fn interrupt(&self, expected_turn_id: &str) -> Result<Value> {
        self.call(
            "thread-follower-interrupt-turn",
            json!({ "mode": "user-stop", "expectedTurnId": expected_turn_id }),
            20_000,
        )
    }

    pub fn decide_command(&self, request_id: &Value, decision: Value) -> Result<Value> {
        self.call(
            "thread-follower-command-approval-decision",
            json!({ "requestId": request_id, "decision": decision }),
            20_000,
        )
    }

    pub fn decide_file_change(&self, request_id: &Value, decision: Value) -> Result<Value> {
        self.call(
            "thread-follower-file-approval-decision",
            json!({ "requestId": request_id, "decision": decision }),
            20_000,
        )
    }

    pub fn answer_user_input(&self, request_id: &Value, answers: Value) -> Result<Value> {
        self.call(
            "thread-follower-submit-user-input",
            json!({ "requestId": request_id, "response": { "answers": answers } }),
            20_000,
        )
    }
}
