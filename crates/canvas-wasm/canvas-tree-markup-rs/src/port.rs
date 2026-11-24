//! The Rust port of `tree-markup.mjs`.

#![forbid(unsafe_code)]

use canvas_common_rs::{
    as_number, esc_html, is_truthy, is_undefined, js_strict_eq, set_has, undefined,
    visible_tree_rows,
};
use serde_json::{Value, json};

fn opt_usize(v: Option<&Value>) -> usize {
    let Some(val) = v.filter(|x| is_truthy(x)) else {
        return 0;
    };
    as_number(val).map_or(0, |n| {
        if n.is_finite() && n > 0.0 {
            n.trunc() as usize
        } else {
            0
        }
    })
}

/// Port of `treeMarkup` (`tree-markup.mjs:3`).
pub fn tree_markup(
    nodes: Value,
    current_node_id: Value,
    collapsed: Value,
    selected: Value,
    options: Value,
) -> Result<Value, String> {
    let col = if is_undefined(&collapsed) {
        json!({ "$set": [] })
    } else {
        collapsed
    };
    let sel = if is_undefined(&selected) {
        Value::Null
    } else {
        selected
    };
    if options.is_null() {
        return Err("TypeError: Cannot read properties of null (reading 'offset')".into());
    }
    let opt = if is_undefined(&options) {
        json!({})
    } else {
        options
    };
    let view = visible_tree_rows(nodes, current_node_id.clone(), col.clone())?;
    let offset = opt_usize(opt.get("offset"));
    let pending_offset = opt_usize(opt.get("pendingOffset"));
    let u = undefined();
    let card = |n: &Value| -> String {
        let nid = n.get("id").unwrap_or(&u);
        let sc = ["", "selected"][usize::from(js_strict_eq(&sel, nid))];
        let cc = ["", "current"][usize::from(js_strict_eq(nid, &current_node_id))];
        let sk = if n.get("summaryKind") == Some(&json!("原文摘录")) {
            "原文摘录"
        } else {
            "模型归纳"
        };
        let cp = ["", " · 当前推进"][usize::from(js_strict_eq(nid, &current_node_id))];
        format!(
            r#"<button class="node {sc} {cc}" data-node="{}"><small>{} · {sk}{cp}</small><strong>{}</strong><p>{}</p></button>"#,
            esc_html(nid),
            esc_html(n.get("status").unwrap_or(&u)),
            esc_html(n.get("title").unwrap_or(&u)),
            esc_html(n.get("summary").unwrap_or(&u))
        )
    };
    let rows = view["rows"].as_array().map(Vec::as_slice).unwrap_or(&[]);
    let tree = if rows.is_empty() {
        r#"<p class="note">点击「整理脉络」，由 Codex 识别目标、方案与多层尝试。</p>"#.to_string()
    } else {
        let mut items = String::new();
        for row in &rows[offset.min(rows.len())..(offset + 100).min(rows.len())] {
            let n = &row["node"];
            let nid = n.get("id").unwrap_or(&u);
            let d = row["depth"].as_u64().unwrap_or(0) as usize;
            let cnt = row["childCount"].as_u64().unwrap_or(0);
            let ac = ["", "active-path"][usize::from(row["active"].as_bool() == Some(true))];
            let toggle = if cnt > 0 {
                let is_col = set_has(&col, nid)?;
                let (lbl, exp, mark) = if is_col {
                    ("展开", "false", "▸")
                } else {
                    ("收起", "true", "▾")
                };
                format!(
                    r#"<button class="tree-toggle" data-collapse="{}" aria-label="{lbl} {}" aria-expanded="{exp}">{mark}</button>"#,
                    esc_html(nid),
                    esc_html(n.get("title").unwrap_or(&u))
                )
            } else {
                r#"<span class="tree-leaf" aria-hidden="true">·</span>"#.into()
            };
            let deep = if d > 8 {
                format!("<small>第 {} 层 · 详情中可查看完整路径</small>", d + 1)
            } else {
                String::new()
            };
            items.push_str(&format!(
                r#"<li class="task-branch {ac}" style="margin-left:{}px"><div class="tree-row">{toggle}{}</div>{deep}</li>"#,
                d.min(8) * 14,
                card(n)
            ));
        }
        format!(r#"<ul class="task-tree" aria-label="模型识别的任务树">{items}</ul>"#)
    };
    let pend = view["pending"].as_array().map(Vec::as_slice).unwrap_or(&[]);
    let p_html = if pend.is_empty() {
        String::new()
    } else {
        let po = pending_offset.min(pend.len());
        let pe = (pending_offset + 20).min(pend.len());
        let cards: String = pend[po..pe].iter().map(card).collect();
        let open = ["", "open"][usize::from(opt.get("pendingOpen").is_some_and(is_truthy))];
        let dis = ["", "disabled"][usize::from(pending_offset == 0)];
        let dis2 = ["", "disabled"][usize::from(pending_offset + 20 >= pend.len())];
        format!(
            r#"<details class="pending-messages" {open}><summary>待整理消息 · {} 条</summary><p class="note">任务归属将在下一次整理时识别。</p><div class="pending-list">{cards}</div><div class="tree-actions"><button data-pending-page="{}" {dis}>前 20 条</button><span>{}–{}</span><button data-pending-page="{}" {dis2}>后 20 条</button></div></details>"#,
            pend.len(),
            pending_offset.saturating_sub(20),
            pending_offset + 1,
            pe,
            pending_offset + 20
        )
    };
    Ok(Value::String(tree + &p_html))
}
