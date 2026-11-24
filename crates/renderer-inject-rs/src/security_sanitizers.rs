//! Extension API route allowlist (`91-extension-api.js`) and DOM XSS sanitizers (`C-3`, `H-5`).

#![forbid(unsafe_code)]

use std::collections::HashSet;
use url::Url;

/// Exact 25 routes exported via `window.codexPlus` (`91-extension-api.js`).
/// Never includes `/hooks/test-run`, `/hooks/file/write`, `/settings/set`, or `/delete` (`C-3`).
pub const EXTENSION_ALLOWED_ROUTES: &[&str] = &[
    "/diagnostics/log",
    "/thread-usage-history",
    "/usage/history",
    "/usage/archive",
    "/archived-thread",
    "/export-markdown",
    "/user-scripts/list",
    "/cdp/targets",
    "/cdp/evaluate",
    "/system/cursor-position",
    "/request-log/recent",
    "/proxy-status",
    "/thread-usage-live",
    "/backend/status",
    "/settings/public",
    "/model-catalog/public",
    "/agent-pet/catalog",
    "/agent-pet/image",
    "/agent-pet/sound",
    "/agent-pet/import-sound",
    "/whale/media",
    "/whale/full",
    "/whale/history",
    "/whale/session",
    "/image-overlay/image",
];

/// Returns true only when `route` is explicitly on the read/safe extension allowlist.
pub fn is_extension_route_allowed(route: &str) -> bool {
    EXTENSION_ALLOWED_ROUTES.contains(&route)
}

/// Returns read-only request coalescing window in milliseconds (`codexPlusExtensionCoalesceWindowMs`).
pub fn extension_coalesce_window_ms(route: &str) -> Option<u64> {
    match route {
        "/request-log/recent" | "/thread-usage-live" | "/proxy-status" => Some(2000),
        "/backend/status" => Some(2200),
        "/thread-usage-history" => Some(3000),
        "/settings/public" | "/cdp/targets" => Some(5000),
        "/model-catalog/public" => Some(30000),
        _ => None,
    }
}

/// Escapes `&`, `<`, `>`, `"`, and `'` for safe HTML text/attribute interpolation.
pub fn escape_html(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for ch in raw.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(ch),
        }
    }
    out
}

/// Escapes a string according to CSSOM `CSS.escape()` rules so selectors cannot inject attributes (`H-5`).
pub fn escape_css_ident(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let chars: Vec<char> = raw.chars().collect();
    if chars.len() == 1 && chars[0] == '-' {
        return "\\-".to_string();
    }
    for (idx, &ch) in chars.iter().enumerate() {
        let code = ch as u32;
        if code == 0 {
            out.push('\u{FFFD}');
        } else if (0x0001..=0x001F).contains(&code)
            || code == 0x007F
            || (idx == 0 && ch.is_ascii_digit())
            || (idx == 1 && ch.is_ascii_digit() && chars[0] == '-')
        {
            out.push_str(&format!("\\{code:x} "));
        } else if code >= 0x0080 || ch == '-' || ch == '_' || ch.is_ascii_alphanumeric() {
            out.push(ch);
        } else {
            out.push('\\');
            out.push(ch);
        }
    }
    out
}

const ALLOWED_SVG_ELEMENTS: &[&str] = &[
    "svg",
    "g",
    "path",
    "circle",
    "ellipse",
    "line",
    "rect",
    "polygon",
    "polyline",
    "defs",
    "linearGradient",
    "radialGradient",
    "stop",
    "clipPath",
    "mask",
    "title",
    "desc",
];

const ALLOWED_SVG_ATTRS: &[&str] = &[
    "xmlns",
    "viewBox",
    "width",
    "height",
    "fill",
    "fill-rule",
    "fill-opacity",
    "stroke",
    "stroke-width",
    "stroke-linecap",
    "stroke-linejoin",
    "stroke-dasharray",
    "stroke-dashoffset",
    "stroke-miterlimit",
    "stroke-opacity",
    "opacity",
    "d",
    "cx",
    "cy",
    "r",
    "rx",
    "ry",
    "x",
    "y",
    "x1",
    "y1",
    "x2",
    "y2",
    "points",
    "transform",
    "id",
    "class",
    "clip-path",
    "clip-rule",
    "mask",
    "offset",
    "stop-color",
    "stop-opacity",
    "gradientUnits",
    "gradientTransform",
    "aria-hidden",
    "focusable",
    "role",
];

fn is_xml_ws(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\r' | b'\n')
}

fn is_svg_name_start(b: u8) -> bool {
    b.is_ascii_alphabetic()
}

fn is_svg_elem_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'-'
}

fn is_svg_attr_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || matches!(b, b'-' | b':')
}

fn is_safe_svg_id(s: &str) -> bool {
    !s.is_empty()
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-' | b'.'))
}

fn is_safe_svg_text_entities(text: &str) -> bool {
    let mut rest = text;
    while let Some(amp) = rest.find('&') {
        let after = &rest[amp + 1..];
        let Some(semi) = after.find(';') else {
            return false;
        };
        if !matches!(&after[..semi], "amp" | "lt" | "gt" | "quot" | "apos") {
            return false;
        }
        rest = &after[semi + 1..];
    }
    true
}

fn is_safe_svg_attr_value(attr_name: &str, raw_val: &str) -> bool {
    if raw_val
        .chars()
        .any(|c| c.is_control() && !matches!(c, '\t' | '\n' | '\r'))
    {
        return false;
    }
    if raw_val.contains(['<', '>', '\\', '&']) {
        return false;
    }
    let compact_lower: String = raw_val
        .chars()
        .filter(|c| !c.is_whitespace())
        .flat_map(char::to_lowercase)
        .collect();
    if compact_lower.contains("javascript:")
        || compact_lower.contains("vbscript:")
        || compact_lower.contains("data:")
        || compact_lower.contains("expression(")
    {
        return false;
    }
    if attr_name == "xmlns" && raw_val.trim() != "http://www.w3.org/2000/svg" {
        return false;
    }
    if attr_name == "id" && !is_safe_svg_id(raw_val) {
        return false;
    }
    if compact_lower.contains("url(") {
        let Some(inner) = raw_val
            .trim()
            .strip_prefix("url(#")
            .and_then(|s| s.strip_suffix(')'))
        else {
            return false;
        };
        if !is_safe_svg_id(inner) {
            return false;
        }
    }
    true
}

/// Sanitizes an SVG icon string with a strict XML/SVG allowlist parser, blocking
/// `<svg onload=...>`, `<set>`, character-reference protocol smuggling, and trailing HTML (`H-5`).
pub fn sanitize_svg_icon(svg: &str) -> Option<String> {
    let trimmed = svg.trim();
    if trimmed.is_empty()
        || trimmed
            .chars()
            .any(|c| c.is_control() && !matches!(c, '\t' | '\n' | '\r'))
    {
        return None;
    }
    let bytes = trimmed.as_bytes();
    if bytes.first().copied() != Some(b'<') {
        return None;
    }

    let mut pos = 0usize;
    let mut stack: Vec<&str> = Vec::new();
    let mut saw_root = false;

    while pos < bytes.len() {
        if saw_root && stack.is_empty() {
            return None;
        }
        if bytes[pos] == b'<' {
            let next = *bytes.get(pos + 1)?;
            if next == b'!' || next == b'?' {
                return None;
            }
            if next == b'/' {
                let expected = stack.pop()?;
                pos += 2;
                if !bytes.get(pos).copied().is_some_and(is_svg_name_start) {
                    return None;
                }
                let name_start = pos;
                while bytes.get(pos).copied().is_some_and(is_svg_elem_char) {
                    pos += 1;
                }
                if &trimmed[name_start..pos] != expected {
                    return None;
                }
                while bytes.get(pos).copied().is_some_and(is_xml_ws) {
                    pos += 1;
                }
                if bytes.get(pos).copied() != Some(b'>') {
                    return None;
                }
                pos += 1;
                continue;
            }
            if !is_svg_name_start(next) {
                return None;
            }
            pos += 1;
            let name_start = pos;
            while bytes.get(pos).copied().is_some_and(is_svg_elem_char) {
                pos += 1;
            }
            let tag_name = &trimmed[name_start..pos];
            if !saw_root {
                if tag_name != "svg" {
                    return None;
                }
                saw_root = true;
            }
            if !ALLOWED_SVG_ELEMENTS.contains(&tag_name) {
                return None;
            }

            let mut seen_attrs: HashSet<&str> = HashSet::new();
            loop {
                if bytes.get(pos).copied() == Some(b'>') {
                    pos += 1;
                    stack.push(tag_name);
                    break;
                }
                if bytes.get(pos..pos + 2) == Some(b"/>") {
                    pos += 2;
                    break;
                }
                if !bytes.get(pos).copied().is_some_and(is_xml_ws) {
                    return None;
                }
                while bytes.get(pos).copied().is_some_and(is_xml_ws) {
                    pos += 1;
                }
                if bytes.get(pos).copied() == Some(b'>') {
                    pos += 1;
                    stack.push(tag_name);
                    break;
                }
                if bytes.get(pos..pos + 2) == Some(b"/>") {
                    pos += 2;
                    break;
                }
                if !bytes.get(pos).copied().is_some_and(is_svg_name_start) {
                    return None;
                }
                let attr_start = pos;
                while bytes.get(pos).copied().is_some_and(is_svg_attr_char) {
                    pos += 1;
                }
                let attr_name = &trimmed[attr_start..pos];
                if !seen_attrs.insert(attr_name)
                    || attr_name.to_ascii_lowercase().starts_with("on")
                    || !ALLOWED_SVG_ATTRS.contains(&attr_name)
                {
                    return None;
                }
                while bytes.get(pos).copied().is_some_and(is_xml_ws) {
                    pos += 1;
                }
                if bytes.get(pos).copied() != Some(b'=') {
                    return None;
                }
                pos += 1;
                while bytes.get(pos).copied().is_some_and(is_xml_ws) {
                    pos += 1;
                }
                let quote = *bytes.get(pos)?;
                if quote != b'"' && quote != b'\'' {
                    return None;
                }
                pos += 1;
                let val_start = pos;
                while pos < bytes.len() && bytes[pos] != quote {
                    pos += 1;
                }
                if bytes.get(pos).copied() != Some(quote) {
                    return None;
                }
                let attr_val = &trimmed[val_start..pos];
                if !is_safe_svg_attr_value(attr_name, attr_val) {
                    return None;
                }
                pos += 1;
            }
        } else {
            if stack.is_empty() {
                return None;
            }
            let text_start = pos;
            while pos < bytes.len() && bytes[pos] != b'<' {
                pos += 1;
            }
            let text_slice = &trimmed[text_start..pos];
            if matches!(stack.last().copied(), Some("title" | "desc")) {
                if text_slice.contains('>') || !is_safe_svg_text_entities(text_slice) {
                    return None;
                }
            } else if !text_slice.bytes().all(is_xml_ws) {
                return None;
            }
        }
    }

    if !saw_root || !stack.is_empty() {
        return None;
    }
    Some(trimmed.to_string())
}

/// Accepts only safe external `http://` or `https://` URLs without credentials or control chars (`H-5`).
pub fn sanitize_external_http_url(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed.chars().any(char::is_control) {
        return None;
    }
    let parsed = Url::parse(trimmed).ok()?;
    if !matches!(parsed.scheme(), "http" | "https") {
        return None;
    }
    if !parsed.username().is_empty() || parsed.password().is_some() || parsed.host_str().is_none() {
        return None;
    }
    Some(parsed.to_string())
}
