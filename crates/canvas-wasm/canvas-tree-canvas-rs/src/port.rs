//! The Rust port of `tree-canvas.mjs`.

#![forbid(unsafe_code)]

use canvas_common_rs::{
    esc_html, is_nil, is_truthy, js_number, js_strict_eq, number_to_string, set_has, to_number,
    undefined,
};
use serde_json::{Value, json};

fn jmin(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else {
        a.min(b)
    }
}
fn jmax(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else {
        a.max(b)
    }
}
fn num_field(v: &Value, k: &str, u: &Value) -> f64 {
    to_number(v.get(k).unwrap_or(u))
}

/// Port of `fitCanvas` (`tree-canvas.mjs:19`).
pub fn fit_canvas(layout: Value, width: Value, height: Value) -> Result<Value, String> {
    if is_nil(&layout) {
        return Err("TypeError: Cannot read properties of null/undefined (reading 'width')".into());
    }
    let u = undefined();
    let lw_v = layout.get("width").unwrap_or(&u);
    if !is_truthy(lw_v) || !is_truthy(&width) || !is_truthy(&height) {
        return Ok(json!({ "x": 40, "y": 40, "scale": 1 }));
    }
    let (lw, lh) = (
        to_number(lw_v),
        to_number(layout.get("height").unwrap_or(&u)),
    );
    let (w, h) = (to_number(&width), to_number(&height));
    let scale = jmax(
        0.00002,
        jmin(
            1.0,
            jmin((jmax(100.0, w) - 80.0) / lw, (jmax(100.0, h) - 80.0) / lh),
        ),
    );
    Ok(json!({
        "x": js_number((w - lw * scale) / 2.0),
        "y": js_number((h - lh * scale) / 2.0),
        "scale": js_number(scale)
    }))
}

/// Port of `zoomCanvas` (`tree-canvas.mjs:24`).
pub fn zoom_canvas(camera: Value, scale: Value, x: Value, y: Value) -> Result<Value, String> {
    if is_nil(&camera) {
        return Err("TypeError: Cannot read properties of null/undefined".into());
    }
    let u = undefined();
    let (cx, cy, cs) = (
        num_field(&camera, "x", &u),
        num_field(&camera, "y", &u),
        num_field(&camera, "scale", &u),
    );
    let (xv, yv) = (to_number(&x), to_number(&y));
    let sc = jmax(0.00002, jmin(2.5, to_number(&scale)));
    Ok(json!({
        "x": js_number(xv - (xv - cx) * sc / cs),
        "y": js_number(yv - (yv - cy) * sc / cs),
        "scale": js_number(sc)
    }))
}

/// Port of `canvasViewport` (`tree-canvas.mjs:28`).
pub fn canvas_viewport(
    layout: Value,
    camera: Value,
    width: Value,
    height: Value,
    limit: Option<f64>,
) -> Result<Value, String> {
    if is_nil(&layout) || is_nil(&camera) {
        return Err("TypeError: Cannot read properties of null/undefined".into());
    }
    let u = undefined();
    let (cx, cy, cs) = (
        num_field(&camera, "x", &u),
        num_field(&camera, "y", &u),
        num_field(&camera, "scale", &u),
    );
    let (w, h, lim) = (
        to_number(&width),
        to_number(&height),
        limit.unwrap_or(600.0),
    );
    let (left, top, right, bottom) = (
        (-cx - 120.0) / cs,
        (-cy - 120.0) / cs,
        (w - cx + 120.0) / cs,
        (h - cy + 120.0) / cs,
    );
    let raw_cards = layout
        .get("cards")
        .and_then(Value::as_array)
        .ok_or("TypeError: layout.cards")?;
    let (mut cards, mut total) = (Vec::new(), 0_usize);
    for c in raw_cards {
        if is_nil(c) {
            return Err("TypeError: null card".into());
        }
        let (x, y, cw, ch) = (
            num_field(c, "x", &u),
            num_field(c, "y", &u),
            num_field(c, "width", &u),
            num_field(c, "height", &u),
        );
        if x + cw >= left && x <= right && y + ch >= top && y <= bottom {
            total += 1;
            if (cards.len() as f64) < lim {
                cards.push(c.clone());
            }
        }
    }
    let raw_edges = layout
        .get("edges")
        .and_then(Value::as_array)
        .ok_or("TypeError: layout.edges")?;
    let mut edges = Vec::new();
    for edge in raw_edges {
        if is_nil(edge) {
            return Err("TypeError: null edge".into());
        }
        let (a, b) = (edge.get("from").unwrap_or(&u), edge.get("to").unwrap_or(&u));
        let (ax, ay, aw, ah) = (
            num_field(a, "x", &u),
            num_field(a, "y", &u),
            num_field(a, "width", &u),
            num_field(a, "height", &u),
        );
        let (bx, by) = (num_field(b, "x", &u), num_field(b, "y", &u));
        if bx >= left
            && ax + aw <= right
            && jmax(ay, by) + ah / 2.0 >= top
            && jmin(ay, by) + ah / 2.0 <= bottom
        {
            edges.push(edge.clone());
            if edges.len() >= 2400 {
                break;
            }
        }
    }
    Ok(json!({ "cards": cards, "edges": edges, "total": total }))
}

/// Port of `canvasCardsMarkup` (`tree-canvas.mjs:37`).
pub fn canvas_cards_markup(
    cards: Value,
    collapsed: Value,
    selected: Value,
    current_node_id: Value,
) -> Result<Value, String> {
    let Value::Array(list) = &cards else {
        return Err("TypeError: cards.map is not a function".into());
    };
    let u = undefined();
    let mut out = String::new();
    for c in list {
        if is_nil(c) {
            return Err("TypeError: null card".into());
        }
        let n = c.get("node").ok_or("TypeError: card.node")?;
        if is_nil(n) {
            return Err("TypeError: null node".into());
        }
        let nid = n.get("id").unwrap_or(&u);
        let title = n.get("title").unwrap_or(&u);
        let status = n.get("status").unwrap_or(&u);
        let cnt = c.get("childCount").unwrap_or(&u);
        let cc = ["", "current"][usize::from(js_strict_eq(nid, &current_node_id))];
        let sc = ["", "selected"][usize::from(js_strict_eq(&selected, nid))];
        let ac = ["", "on-path"][usize::from(c.get("active").is_some_and(is_truthy))];
        let cur_tag = ["", " · 当前推进"][usize::from(
            js_strict_eq(nid, &current_node_id) && status.as_str() != Some("当前推进"),
        )];
        let src_len = n
            .get("sources")
            .filter(|v| !is_nil(v))
            .and_then(|v| {
                v.as_array()
                    .map(|a| a.len() as f64)
                    .or_else(|| canvas_common_rs::string_units(v).map(|u| u.len() as f64))
                    .or_else(|| v.get("length").and_then(canvas_common_rs::as_number))
            })
            .unwrap_or(0.0);
        let meta = if is_truthy(cnt) {
            format!("{} 个分支", number_to_string(to_number(cnt)))
        } else {
            "任务节点".into()
        };
        let fold = if is_truthy(cnt) {
            let is_col = set_has(&collapsed, nid)?;
            let (lbl, exp, mark) = if is_col {
                ("展开", "false", "+")
            } else {
                ("收起", "true", "−")
            };
            format!(
                r#"<button class="canvas-fold" data-collapse="{}" aria-label="{lbl} {}" aria-expanded="{exp}">{mark}</button>"#,
                esc_html(nid),
                esc_html(title)
            )
        } else {
            String::new()
        };
        out.push_str(&format!(
            r#"<div class="canvas-card-wrap" style="left:{}px;top:{}px;width:{}px;height:{}px"><button class="node canvas-card {cc} {sc} {ac}" data-node="{}" title="{}"><small>{}{cur_tag}</small><strong>{}</strong><p>{}</p><span class="canvas-card-meta">{meta} · {} 条依据</span></button>{fold}</div>"#,
            number_to_string(num_field(c, "x", &u)),
            number_to_string(num_field(c, "y", &u)),
            number_to_string(num_field(c, "width", &u)),
            number_to_string(num_field(c, "height", &u)),
            esc_html(nid), esc_html(title), esc_html(status), esc_html(title), esc_html(n.get("summary").unwrap_or(&u)), number_to_string(src_len)
        ));
    }
    Ok(Value::String(out))
}
