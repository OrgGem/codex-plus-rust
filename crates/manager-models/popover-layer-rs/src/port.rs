//! The Rust port of `src/popover-layer.ts`.

#![forbid(unsafe_code)]

use crate::js::{math_round, number};
use manager_common_rs::{is_nil, is_undefined, to_number};
use serde_json::{Map, Value, json};

const POPOVER_DEFAULT_MAX_HEIGHT: f64 = 320.0;
const POPOVER_DEFAULT_GAP: f64 = 6.0;
const POPOVER_DEFAULT_EDGE: f64 = 8.0;
const ANCHORED_MENU_DEFAULT_GAP: f64 = 8.0;
const ANCHORED_MENU_DEFAULT_EDGE: f64 = 12.0;
const ANCHORED_MENU_MIN_HEIGHT: f64 = 240.0;

fn js_max(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else if a == 0.0 && b == 0.0 {
        if a.is_sign_positive() || b.is_sign_positive() {
            0.0
        } else {
            -0.0
        }
    } else if a > b {
        a
    } else {
        b
    }
}

fn js_min(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else if a == 0.0 && b == 0.0 {
        if a.is_sign_negative() || b.is_sign_negative() {
            -0.0
        } else {
            0.0
        }
    } else if a < b {
        a
    } else {
        b
    }
}

fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value.is_nan() {
        return min;
    }
    js_min(js_max(value, min), js_max(min, max))
}

fn check_not_nil<'a>(v: &'a Value, prop: &str) -> Result<Option<&'a Map<String, Value>>, String> {
    if is_nil(v) {
        return Err(format!(
            "TypeError: Cannot read properties of null/undefined (reading '{prop}')"
        ));
    }
    Ok(v.as_object())
}

fn num_prop(obj: Option<&Map<String, Value>>, key: &str, default: Option<f64>) -> f64 {
    match obj.and_then(|o| o.get(key)) {
        None => default.unwrap_or(f64::NAN),
        Some(v) if is_undefined(v) => default.unwrap_or(f64::NAN),
        Some(v) => to_number(v),
    }
}

/// Port of `resolvePopoverLayout` (`src/popover-layer.ts:67`).
pub fn resolve_popover_layout(arg0: Value) -> Result<Value, String> {
    let root = check_not_nil(&arg0, "trigger")?;
    let undef = crate::js::undefined();
    let trigger_val = root.and_then(|o| o.get("trigger")).unwrap_or(&undef);
    let viewport_val = root.and_then(|o| o.get("viewport")).unwrap_or(&undef);
    let viewport = check_not_nil(viewport_val, "width")?;
    let trigger = check_not_nil(trigger_val, "width")?;

    let menu_height = num_prop(root, "menuHeight", None);
    let align_end = root.and_then(|o| o.get("align")).and_then(Value::as_str) == Some("end");
    let menu_width = match root.and_then(|o| o.get("menuWidth")) {
        Some(v) if !is_nil(v) => to_number(v),
        _ => num_prop(trigger, "width", None),
    };
    let min_width = num_prop(root, "minWidth", Some(0.0));
    let max_height = num_prop(root, "maxHeight", Some(POPOVER_DEFAULT_MAX_HEIGHT));
    let gap = num_prop(root, "gap", Some(POPOVER_DEFAULT_GAP));
    let edge = num_prop(root, "edge", Some(POPOVER_DEFAULT_EDGE));

    let vp_w = num_prop(viewport, "width", None);
    let vp_h = num_prop(viewport, "height", None);
    let usable_width = js_max(0.0, vp_w - edge * 2.0);
    let desired_width = js_max(min_width, menu_width);
    let width = math_round(clamp(desired_width, 0.0, usable_width));

    let anchored_left = if align_end {
        num_prop(trigger, "right", None) - width
    } else {
        num_prop(trigger, "left", None)
    };
    let left = math_round(clamp(
        anchored_left,
        edge,
        js_max(edge, vp_w - edge - width),
    ));

    let height_limit = js_max(0.0, max_height);
    let desired_height = js_min(
        if menu_height > 0.0 {
            menu_height
        } else {
            height_limit
        },
        height_limit,
    );
    let trig_bottom = num_prop(trigger, "bottom", None);
    let trig_top = num_prop(trigger, "top", None);
    let space_below = vp_h - edge - (trig_bottom + gap);
    let space_above = trig_top - gap - edge;
    let placement = if space_below >= desired_height || space_below >= space_above {
        "bottom"
    } else {
        "top"
    };
    let available = js_max(
        0.0,
        if placement == "bottom" {
            space_below
        } else {
            space_above
        },
    );
    let final_height = js_max(0.0, js_min(desired_height, available));
    let anchored_top = if placement == "bottom" {
        trig_bottom + gap
    } else {
        trig_top - gap - final_height
    };
    let top = math_round(clamp(
        anchored_top,
        edge,
        js_max(edge, vp_h - edge - final_height),
    ));

    Ok(json!({
        "left": number(left),
        "top": number(top),
        "width": number(width),
        "maxHeight": number(math_round(final_height)),
        "placement": placement,
    }))
}

/// Port of `resolveAnchoredMenuLayout` (`src/popover-layer.ts:129`).
pub fn resolve_anchored_menu_layout(
    trigger: Value,
    viewport: Value,
    gap: Value,
    edge: Value,
) -> Result<Value, String> {
    let vp = check_not_nil(&viewport, "width")?;
    let trig = check_not_nil(&trigger, "right")?;
    let gap = if is_undefined(&gap) {
        ANCHORED_MENU_DEFAULT_GAP
    } else {
        to_number(&gap)
    };
    let edge = if is_undefined(&edge) {
        ANCHORED_MENU_DEFAULT_EDGE
    } else {
        to_number(&edge)
    };

    let vp_w = num_prop(vp, "width", None);
    let vp_h = num_prop(vp, "height", None);
    let right = math_round(js_max(edge, vp_w - num_prop(trig, "right", None)));
    let trig_bottom = num_prop(trig, "bottom", None);
    let trig_top = num_prop(trig, "top", None);
    let space_below = vp_h - edge - (trig_bottom + gap);
    let space_above = trig_top - edge - gap;
    let placement = if space_below >= ANCHORED_MENU_MIN_HEIGHT || space_below >= space_above {
        "bottom"
    } else {
        "top"
    };
    let available = js_max(
        0.0,
        if placement == "bottom" {
            space_below
        } else {
            space_above
        },
    );
    let max_height = math_round(js_min(
        js_max(available, ANCHORED_MENU_MIN_HEIGHT),
        js_max(0.0, vp_h - edge * 2.0),
    ));
    let preferred_top = if placement == "bottom" {
        trig_bottom + gap
    } else {
        trig_top - gap - max_height
    };
    let top = math_round(js_min(
        js_max(preferred_top, edge),
        js_max(edge, vp_h - edge - max_height),
    ));
    Ok(json!({
        "placement": placement,
        "top": number(top),
        "right": number(right),
        "maxHeight": number(max_height),
    }))
}

/// Port of `isTriggerVisible` (`src/popover-layer.ts:160`).
pub fn is_trigger_visible(
    trigger: Value,
    viewport: Value,
    edge: Option<f64>,
) -> Result<bool, String> {
    let trig = check_not_nil(&trigger, "width")?;
    let edge = edge.unwrap_or(0.0);
    if num_prop(trig, "width", None) <= 0.0 || num_prop(trig, "height", None) <= 0.0 {
        return Ok(false);
    }
    let bottom_ok = num_prop(trig, "bottom", None) > edge;
    if !bottom_ok {
        return Ok(false);
    }
    let vp = check_not_nil(&viewport, "height")?;
    let top_ok = num_prop(trig, "top", None) < num_prop(vp, "height", None) - edge;
    let right_ok = num_prop(trig, "right", None) > edge;
    let left_ok = num_prop(trig, "left", None) < num_prop(vp, "width", None) - edge;
    Ok(top_ok && right_ok && left_ok)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rect(top: f64, left: f64, width: f64, height: f64) -> Value {
        json!({ "top": top, "left": left, "right": left + width, "bottom": top + height, "width": width, "height": height })
    }

    #[test]
    fn resolves_popover_and_anchored_layouts() {
        let vp = json!({ "width": 1200, "height": 800 });
        let below = resolve_popover_layout(
            json!({ "trigger": rect(100.0, 40.0, 260.0, 40.0), "viewport": vp, "menuHeight": 180 }),
        )
        .unwrap();
        assert_eq!(
            below,
            json!({ "left": 40, "top": 146, "width": 260, "maxHeight": 180, "placement": "bottom" })
        );

        let flipped = resolve_popover_layout(
            json!({ "trigger": rect(700.0, 40.0, 200.0, 40.0), "viewport": vp, "menuHeight": 300 }),
        )
        .unwrap();
        assert_eq!(flipped["placement"], "top");
        assert_eq!(flipped["top"], 394);

        let anchored_top = resolve_anchored_menu_layout(
            rect(780.0, 900.0, 200.0, 40.0),
            json!({ "width": 1200, "height": 820 }),
            json!({ "$undefined": true }),
            json!({ "$undefined": true }),
        )
        .unwrap();
        assert_eq!(anchored_top["placement"], "top");
        assert!(anchored_top["maxHeight"].as_f64().unwrap() > 0.0);

        assert!(!is_trigger_visible(rect(-80.0, 20.0, 200.0, 40.0), vp.clone(), None).unwrap());
        assert!(is_trigger_visible(rect(100.0, 20.0, 200.0, 40.0), vp, None).unwrap());
        assert!(resolve_popover_layout(Value::Null).is_err());
    }
}
