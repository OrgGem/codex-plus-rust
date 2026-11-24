//! Rust port of `tools/rice-maid-pet` (`backend/main.mjs`, `hooks/request.mjs`, `renderer.js` math).

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const SERVICE_NAME: &str = "codexpp.riceMaidPet";
pub const RATIO: f64 = 568.0 / 512.0;
pub const GAP: f64 = 8.0;
pub const TOP: f64 = 48.0;
pub const FRAME_DURATIONS: [u64; 11] = [100, 100, 120, 100, 100, 180, 100, 100, 160, 100, 100];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BiteState {
    pub epoch: String,
    pub count: u64,
    pub at: u64,
}

pub fn random_uuid_v4() -> Result<String, String> {
    let mut b = [0_u8; 16];
    getrandom::getrandom(&mut b).map_err(|e| format!("RNG error: {e}"))?;
    b[6] = (b[6] & 0x0f) | 0x40;
    b[8] = (b[8] & 0x3f) | 0x80;
    Ok(format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        b[0],
        b[1],
        b[2],
        b[3],
        b[4],
        b[5],
        b[6],
        b[7],
        b[8],
        b[9],
        b[10],
        b[11],
        b[12],
        b[13],
        b[14],
        b[15]
    ))
}

type EmitCb = Box<dyn FnMut(&str, BiteState)>;

/// Port of `tools/rice-maid-pet/backend/main.mjs`.
pub struct RiceMaidBackend {
    epoch: String,
    count: u64,
    at: u64,
    emit: EmitCb,
}

impl RiceMaidBackend {
    pub fn new(emit: impl FnMut(&str, BiteState) + 'static) -> Result<Self, String> {
        Ok(Self {
            epoch: random_uuid_v4()?,
            count: 0,
            at: 0,
            emit: Box::new(emit),
        })
    }

    pub fn get_state(&self) -> BiteState {
        BiteState {
            epoch: self.epoch.clone(),
            count: self.count,
            at: self.at,
        }
    }

    pub fn request_bite(&mut self, now_ms: u64) -> BiteState {
        self.count += 1;
        self.at = now_ms;
        let state = self.get_state();
        (self.emit)("bite", state.clone());
        state
    }
}

/// Port of `tools/rice-maid-pet/hooks/request.mjs`.
/// Never inspects the request payload and swallows unavailable-service errors.
pub fn pre_model_request(
    call_request_bite: impl FnOnce() -> Result<BiteState, String>,
) -> Option<Value> {
    let _ = call_request_bite();
    None
}

pub fn clamp_pet_size(configured: Option<f64>) -> f64 {
    match configured {
        Some(n) if n.is_finite() => n.clamp(112.0, 280.0),
        _ => 192.0,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewportLayout {
    pub width: f64,
    pub height: f64,
    pub pet_size: f64,
    pub pet_height: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PetPosition {
    pub x: f64,
    pub y: f64,
}

pub fn viewport_size(inner_width: f64, inner_height: f64, size: f64) -> ViewportLayout {
    let width = if inner_width.is_finite() {
        inner_width.max(1.0)
    } else {
        1.0
    };
    let height = if inner_height.is_finite() {
        inner_height.max(1.0)
    } else {
        1.0
    };
    let base_size = if size.is_finite() && size > 0.0 {
        size
    } else {
        clamp_pet_size(None)
    };
    let room_w = if width > GAP * 2.0 {
        width - GAP * 2.0
    } else {
        width
    };
    let room_h = if height > TOP + GAP {
        height - TOP - GAP
    } else {
        height
    };
    let pet_size = base_size.min(room_w).min(room_h / RATIO);
    ViewportLayout {
        width,
        height,
        pet_size,
        pet_height: pet_size * RATIO,
    }
}

pub fn clamp_position(
    next: Option<PetPosition>,
    inner_width: f64,
    inner_height: f64,
    size: f64,
) -> PetPosition {
    let vp = viewport_size(inner_width, inner_height, size);
    let default_pos = PetPosition {
        x: vp.width - vp.pet_size - 28.0,
        y: vp.height - vp.pet_height - 164.0,
    };
    let candidate = next
        .filter(|p| p.x.is_finite() && p.y.is_finite())
        .unwrap_or(default_pos);
    let max_x = (vp.width - vp.pet_size - GAP).max(0.0);
    let max_y = (vp.height - vp.pet_height - GAP).max(0.0);
    let min_x = GAP.min(max_x);
    let min_y = TOP.min(max_y);
    PetPosition {
        x: candidate.x.clamp(min_x, max_x),
        y: candidate.y.clamp(min_y, max_y),
    }
}

pub fn frame_transform(index: usize) -> String {
    let col = (index % 4) * 25;
    let row = ((index / 4) * 100) as f64 / 3.0;
    format!("translate3d(-{col}%, -{row}%, 0)")
}
