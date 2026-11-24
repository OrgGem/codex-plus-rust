//! The Rust port of `src/aggregate-routes.ts`.

#![forbid(unsafe_code)]

use crate::js;
use manager_common_rs::js_rt::{is_nil, is_truthy, js_trim};
use serde_json::{Value, json};

fn set_has(set_val: &Value, key: &Value) -> Result<bool, String> {
    let arr = set_val
        .as_object()
        .filter(|o| o.len() == 1)
        .and_then(|o| o.get("$set"))
        .and_then(Value::as_array)
        .ok_or("TypeError: memberIds.has is not a function")?;
    Ok(arr
        .iter()
        .any(|x| match (js::as_number(x), js::as_number(key)) {
            (Some(a), Some(b)) => (a.is_nan() && b.is_nan()) || a == b,
            _ => x == key,
        }))
}

fn route_parts(route: &Value) -> Result<(&str, &Value, &Value), String> {
    if is_nil(route) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'pattern')".into(),
        );
    }
    let pattern = route
        .get("pattern")
        .and_then(Value::as_str)
        .map(js_trim)
        .ok_or("TypeError: route.pattern.trim is not a function")?;
    let profile_id = if route
        .as_object()
        .is_some_and(|o| o.contains_key("profileId"))
    {
        route.get("profileId").unwrap_or(&Value::Null)
    } else {
        route.get("relayId").unwrap_or_else(|| {
            static UNDEF: std::sync::OnceLock<Value> = std::sync::OnceLock::new();
            UNDEF.get_or_init(js::undefined)
        })
    };
    let priority = route.get("priority").unwrap_or_else(|| {
        static UNDEF: std::sync::OnceLock<Value> = std::sync::OnceLock::new();
        UNDEF.get_or_init(js::undefined)
    });
    Ok((pattern, profile_id, priority))
}

/// Port of `clampAggregateRoutePriority` (`src/aggregate-routes.ts:32`).
pub fn clamp_aggregate_route_priority(value: f64) -> Result<f64, String> {
    if !value.is_finite() {
        return Ok(0.0);
    }
    let r = js::math_round(value).clamp(0.0, 999.0);
    Ok(if r == 0.0 { 0.0 } else { r })
}

/// Port of `normalizeAggregateRoutes` (`src/aggregate-routes.ts:37`).
pub fn normalize_aggregate_routes(
    routes: Vec<Value>,
    options: Value,
) -> Result<Vec<Value>, String> {
    if options.is_null() {
        return Err(
            "TypeError: Cannot destructure property 'dropEmptyPattern' of 'options' as it is null."
                .into(),
        );
    }
    let drop_empty = options.get("dropEmptyPattern").is_some_and(is_truthy);
    let member_ids = options
        .get("memberIds")
        .filter(|v| !is_nil(v) && is_truthy(v));
    let clamp = match options.get("clampPriority") {
        None => true,
        Some(v) if js::is_undefined(v) => true,
        Some(v) => is_truthy(v),
    };
    let mut mapped = Vec::with_capacity(routes.len());
    for route in &routes {
        let (pattern, profile_id, prio_val) = route_parts(route)?;
        let priority = if clamp {
            let n = js::as_number(prio_val).unwrap_or(f64::NAN);
            js::number(clamp_aggregate_route_priority(n)?)
        } else {
            prio_val.clone()
        };
        mapped.push((pattern.to_string(), profile_id.clone(), priority));
    }
    let mut out = Vec::new();
    for (pattern, profile_id, priority) in mapped {
        if drop_empty && pattern.is_empty() {
            continue;
        }
        if let Some(m) = member_ids
            && !set_has(m, &profile_id)?
        {
            continue;
        }
        out.push(json!({ "pattern": pattern, "profileId": profile_id, "priority": priority }));
    }
    Ok(out)
}

/// Port of `validateAggregateRoutes` (`src/aggregate-routes.ts:55`).
pub fn validate_aggregate_routes(
    routes: Vec<Value>,
    member_ids: Value,
) -> Result<Option<Vec<Value>>, String> {
    let mut issues = Vec::new();
    for route in &routes {
        let (pattern, profile_id, prio_val) = route_parts(route)?;
        if pattern.is_empty() {
            let pid = profile_id
                .as_str()
                .map(js_trim)
                .ok_or("TypeError: profileId.trim is not a function")?;
            if !pid.is_empty() {
                issues.push(json!({ "code": "emptyPattern", "pattern": "" }));
            }
            continue;
        }
        let valid_prio = js::as_number(prio_val)
            .is_some_and(|n| n.is_finite() && n.fract() == 0.0 && (0.0..=999.0).contains(&n));
        if !valid_prio {
            issues.push(json!({ "code": "invalidPriority", "pattern": pattern }));
            continue;
        }
        if !set_has(&member_ids, profile_id)? {
            issues.push(json!({ "code": "notMember", "pattern": pattern }));
        }
    }
    Ok((!issues.is_empty()).then_some(issues))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_normalize_and_validate() {
        assert_eq!(clamp_aggregate_route_priority(f64::NAN).unwrap(), 0.0);
        assert_eq!(clamp_aggregate_route_priority(3.6).unwrap(), 4.0);
        assert_eq!(clamp_aggregate_route_priority(1500.0).unwrap(), 999.0);
        let rs = vec![
            json!({"pattern":"  ds-* ","profileId":"m1","priority":1200}),
            json!({"pattern":" ","profileId":"m2","priority":1}),
        ];
        let norm = normalize_aggregate_routes(
            rs.clone(),
            json!({"dropEmptyPattern":true,"memberIds":{"$set":["m1"]}}),
        )
        .unwrap();
        assert_eq!(
            norm,
            vec![json!({"pattern":"ds-*","profileId":"m1","priority":999})]
        );
        assert!(normalize_aggregate_routes(rs, Value::Null).is_err());
        assert_eq!(
            validate_aggregate_routes(
                vec![json!({"pattern":"","profileId":"","priority":1})],
                json!({"$set":[]})
            )
            .unwrap(),
            None
        );
        assert_eq!(
            validate_aggregate_routes(
                vec![json!({"pattern":"a-*","profileId":"m2","priority":1000})],
                json!({"$set":["m1"]})
            )
            .unwrap()
            .unwrap()[0]["code"],
            json!("invalidPriority")
        );
    }
}
