//! JS bindings for `src/multi-launch-model.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `multiLaunchInstanceLabel`.
fn multi_launch_instance_label_inner(instance: String, labels: String) -> Result<String, String> {
    let instance = crate::js::from_json(&instance)?;
    let labels = crate::js::from_json(&labels)?;
    let out = port::multi_launch_instance_label(instance, labels)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "multiLaunchInstanceLabel")]
pub fn multi_launch_instance_label(instance: String, labels: String) -> Result<String, JsError> {
    multi_launch_instance_label_inner(instance, labels).map_err(|e| JsError::new(&e))
}

/// `multiLaunchInstanceStatus`.
fn multi_launch_instance_status_inner(instance: String) -> Result<String, String> {
    let instance = crate::js::from_json(&instance)?;
    let out = port::multi_launch_instance_status(instance)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "multiLaunchInstanceStatus")]
pub fn multi_launch_instance_status(instance: String) -> Result<String, JsError> {
    multi_launch_instance_status_inner(instance).map_err(|e| JsError::new(&e))
}

/// `multiLaunchInstanceRunning`.
fn multi_launch_instance_running_inner(instance: String) -> Result<bool, String> {
    let instance = crate::js::from_json(&instance)?;
    let out = port::multi_launch_instance_running(instance)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "multiLaunchInstanceRunning")]
pub fn multi_launch_instance_running(instance: String) -> Result<bool, JsError> {
    multi_launch_instance_running_inner(instance).map_err(|e| JsError::new(&e))
}

/// `multiLaunchInstanceVisible`.
fn multi_launch_instance_visible_inner(instance: String) -> Result<bool, String> {
    let instance = crate::js::from_json(&instance)?;
    let out = port::multi_launch_instance_visible(instance)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "multiLaunchInstanceVisible")]
pub fn multi_launch_instance_visible(instance: String) -> Result<bool, JsError> {
    multi_launch_instance_visible_inner(instance).map_err(|e| JsError::new(&e))
}

/// `multiLaunchInstanceIsMain`.
fn multi_launch_instance_is_main_inner(instance: String) -> Result<bool, String> {
    let instance = crate::js::from_json(&instance)?;
    let out = port::multi_launch_instance_is_main(instance)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "multiLaunchInstanceIsMain")]
pub fn multi_launch_instance_is_main(instance: String) -> Result<bool, JsError> {
    multi_launch_instance_is_main_inner(instance).map_err(|e| JsError::new(&e))
}

/// `multiLaunchInstanceCanStop`.
fn multi_launch_instance_can_stop_inner(instance: String) -> Result<bool, String> {
    let instance = crate::js::from_json(&instance)?;
    let out = port::multi_launch_instance_can_stop(instance)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "multiLaunchInstanceCanStop")]
pub fn multi_launch_instance_can_stop(instance: String) -> Result<bool, JsError> {
    multi_launch_instance_can_stop_inner(instance).map_err(|e| JsError::new(&e))
}

/// `multiLaunchInstanceCanRestart`.
fn multi_launch_instance_can_restart_inner(instance: String) -> Result<bool, String> {
    let instance = crate::js::from_json(&instance)?;
    let out = port::multi_launch_instance_can_restart(instance)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "multiLaunchInstanceCanRestart")]
pub fn multi_launch_instance_can_restart(instance: String) -> Result<bool, JsError> {
    multi_launch_instance_can_restart_inner(instance).map_err(|e| JsError::new(&e))
}

/// `multiLaunchInstanceFailure`.
fn multi_launch_instance_failure_inner(instance: String) -> Result<String, String> {
    let instance = crate::js::from_json(&instance)?;
    let out = port::multi_launch_instance_failure(instance)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "multiLaunchInstanceFailure")]
pub fn multi_launch_instance_failure(instance: String) -> Result<String, JsError> {
    multi_launch_instance_failure_inner(instance).map_err(|e| JsError::new(&e))
}

/// `runningInstancesForProfile`.
fn running_instances_for_profile_inner(
    instances: String,
    profile_id: String,
) -> Result<String, String> {
    let instances = crate::js::from_json(&instances)?;
    let out = port::running_instances_for_profile(instances, profile_id)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "runningInstancesForProfile")]
pub fn running_instances_for_profile(
    instances: String,
    profile_id: String,
) -> Result<String, JsError> {
    running_instances_for_profile_inner(instances, profile_id).map_err(|e| JsError::new(&e))
}

// ---- the binary wire ------------------------------------------------------------------------

thread_local! {
    static WIRE_IN: std::cell::RefCell<Vec<u8>> = const { std::cell::RefCell::new(Vec::new()) };
    static WIRE_OUT: std::cell::RefCell<Vec<u8>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// This module's memory, where `index.cjs` reads and writes the wire buffers.
#[wasm_bindgen(js_name = "rustAnythingWireMemory")]
pub fn rust_anything_wire_memory() -> JsValue {
    wasm_bindgen::memory()
}

/// The address of an argument buffer of `len` bytes.
#[wasm_bindgen(js_name = "rustAnythingWireIn")]
pub fn rust_anything_wire_in(len: u32) -> u32 {
    WIRE_IN.with(|b| {
        let mut b = b.borrow_mut();
        b.resize(len as usize, 0);
        b.as_mut_ptr() as usize as u32
    })
}

/// The address of the last result.
#[wasm_bindgen(js_name = "rustAnythingWireOut")]
pub fn rust_anything_wire_out() -> u32 {
    WIRE_OUT.with(|b| b.borrow().as_ptr() as usize as u32)
}

/// Reads `len` argument bytes, runs `f`, and keeps its result for `rustAnythingWireOut`.
fn wire_io(
    len: u32,
    f: impl FnOnce(&mut wire::Reader, &mut wire::Writer) -> Result<(), String>,
) -> Result<u32, JsError> {
    let input = WIRE_IN.with(|b| std::mem::take(&mut *b.borrow_mut()));
    let mut w = wire::Writer::default();
    let result = f(
        &mut wire::Reader::new(&input[..(len as usize).min(input.len())]),
        &mut w,
    );
    WIRE_IN.with(|b| *b.borrow_mut() = input);
    match result {
        // A result of another shape than the TS type: JS runs the call through JSON.
        Err(e) if e.starts_with(wire::MISMATCH) => return Ok(u32::MAX),
        other => other.map_err(|e| JsError::new(&e))?,
    }
    let n = w.0.len();
    WIRE_OUT.with(|b| *b.borrow_mut() = w.0);
    Ok(u32::try_from(n).unwrap_or(u32::MAX))
}

static S_MULTI_LAUNCH_INSTANCE_LABEL_0: wire::Schema = wire::Schema::Dyn;
static S_MULTI_LAUNCH_INSTANCE_LABEL_1: wire::Schema = wire::Schema::Dyn;
static S_MULTI_LAUNCH_INSTANCE_LABEL_OUT: wire::Schema = wire::Schema::Str16;

/// `multiLaunchInstanceLabel` over the binary wire.
#[wasm_bindgen(js_name = "multiLaunchInstanceLabelWire")]
pub fn multi_launch_instance_label_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let instance: Value = wire::read_value(r, &S_MULTI_LAUNCH_INSTANCE_LABEL_0)?;
        let labels: Value = wire::read_value(r, &S_MULTI_LAUNCH_INSTANCE_LABEL_1)?;
        let out = &port::multi_launch_instance_label(instance, labels)?;
        wire::write_value(w, out, &S_MULTI_LAUNCH_INSTANCE_LABEL_OUT)?;
        Ok(())
    })
}

static S_MULTI_LAUNCH_INSTANCE_STATUS_0: wire::Schema = wire::Schema::Dyn;

/// `multiLaunchInstanceStatus` over the binary wire.
#[wasm_bindgen(js_name = "multiLaunchInstanceStatusWire")]
pub fn multi_launch_instance_status_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let instance: Value = wire::read_value(r, &S_MULTI_LAUNCH_INSTANCE_STATUS_0)?;
        let out = &port::multi_launch_instance_status(instance)?;
        w.string16(out);
        Ok(())
    })
}

static S_MULTI_LAUNCH_INSTANCE_RUNNING_0: wire::Schema = wire::Schema::Dyn;

/// `multiLaunchInstanceRunning` over the binary wire.
#[wasm_bindgen(js_name = "multiLaunchInstanceRunningWire")]
pub fn multi_launch_instance_running_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let instance: Value = wire::read_value(r, &S_MULTI_LAUNCH_INSTANCE_RUNNING_0)?;
        let out = &port::multi_launch_instance_running(instance)?;
        w.bool(*out);
        Ok(())
    })
}

static S_MULTI_LAUNCH_INSTANCE_VISIBLE_0: wire::Schema = wire::Schema::Dyn;

/// `multiLaunchInstanceVisible` over the binary wire.
#[wasm_bindgen(js_name = "multiLaunchInstanceVisibleWire")]
pub fn multi_launch_instance_visible_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let instance: Value = wire::read_value(r, &S_MULTI_LAUNCH_INSTANCE_VISIBLE_0)?;
        let out = &port::multi_launch_instance_visible(instance)?;
        w.bool(*out);
        Ok(())
    })
}

static S_MULTI_LAUNCH_INSTANCE_IS_MAIN_0: wire::Schema = wire::Schema::Dyn;

/// `multiLaunchInstanceIsMain` over the binary wire.
#[wasm_bindgen(js_name = "multiLaunchInstanceIsMainWire")]
pub fn multi_launch_instance_is_main_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let instance: Value = wire::read_value(r, &S_MULTI_LAUNCH_INSTANCE_IS_MAIN_0)?;
        let out = &port::multi_launch_instance_is_main(instance)?;
        w.bool(*out);
        Ok(())
    })
}

static S_MULTI_LAUNCH_INSTANCE_CAN_STOP_0: wire::Schema = wire::Schema::Dyn;

/// `multiLaunchInstanceCanStop` over the binary wire.
#[wasm_bindgen(js_name = "multiLaunchInstanceCanStopWire")]
pub fn multi_launch_instance_can_stop_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let instance: Value = wire::read_value(r, &S_MULTI_LAUNCH_INSTANCE_CAN_STOP_0)?;
        let out = &port::multi_launch_instance_can_stop(instance)?;
        w.bool(*out);
        Ok(())
    })
}

static S_MULTI_LAUNCH_INSTANCE_CAN_RESTART_0: wire::Schema = wire::Schema::Dyn;

/// `multiLaunchInstanceCanRestart` over the binary wire.
#[wasm_bindgen(js_name = "multiLaunchInstanceCanRestartWire")]
pub fn multi_launch_instance_can_restart_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let instance: Value = wire::read_value(r, &S_MULTI_LAUNCH_INSTANCE_CAN_RESTART_0)?;
        let out = &port::multi_launch_instance_can_restart(instance)?;
        w.bool(*out);
        Ok(())
    })
}

static S_MULTI_LAUNCH_INSTANCE_FAILURE_0: wire::Schema = wire::Schema::Dyn;

/// `multiLaunchInstanceFailure` over the binary wire.
#[wasm_bindgen(js_name = "multiLaunchInstanceFailureWire")]
pub fn multi_launch_instance_failure_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let instance: Value = wire::read_value(r, &S_MULTI_LAUNCH_INSTANCE_FAILURE_0)?;
        let out = &port::multi_launch_instance_failure(instance)?;
        w.string16(out);
        Ok(())
    })
}

static S_RUNNING_INSTANCES_FOR_PROFILE_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("instanceId", wire::Schema::Str16),
    ("profileId", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("profileName", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("label", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("isMain", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("pid", wire::Schema::OptJs(&wire::Schema::Num)),
    ("debugPort", wire::Schema::OptJs(&wire::Schema::Num)),
    ("helperPort", wire::Schema::OptJs(&wire::Schema::Num)),
    ("status", wire::Schema::Str16),
    ("running", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("startedAtMs", wire::Schema::OptJs(&wire::Schema::Num)),
    ("startedAt", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("appPath", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "launchResult",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("status", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("message", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("finishedAtMs", wire::Schema::OptJs(&wire::Schema::Num)),
            ("debugPort", wire::Schema::OptJs(&wire::Schema::Num)),
            ("pid", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
]));
static S_RUNNING_INSTANCES_FOR_PROFILE_OUT: wire::Schema =
    wire::Schema::List(&wire::Schema::Obj(&[
        ("instanceId", wire::Schema::Str16),
        ("profileId", wire::Schema::OptJs(&wire::Schema::Str16)),
        ("profileName", wire::Schema::OptJs(&wire::Schema::Str16)),
        ("label", wire::Schema::OptJs(&wire::Schema::Str16)),
        ("isMain", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("pid", wire::Schema::OptJs(&wire::Schema::Num)),
        ("debugPort", wire::Schema::OptJs(&wire::Schema::Num)),
        ("helperPort", wire::Schema::OptJs(&wire::Schema::Num)),
        ("status", wire::Schema::Str16),
        ("running", wire::Schema::OptJs(&wire::Schema::Bool)),
        ("startedAtMs", wire::Schema::OptJs(&wire::Schema::Num)),
        ("startedAt", wire::Schema::OptJs(&wire::Schema::Str16)),
        ("appPath", wire::Schema::OptJs(&wire::Schema::Str16)),
        (
            "launchResult",
            wire::Schema::OptJs(&wire::Schema::Obj(&[
                ("status", wire::Schema::OptJs(&wire::Schema::Str16)),
                ("message", wire::Schema::OptJs(&wire::Schema::Str16)),
                ("finishedAtMs", wire::Schema::OptJs(&wire::Schema::Num)),
                ("debugPort", wire::Schema::OptJs(&wire::Schema::Num)),
                ("pid", wire::Schema::OptJs(&wire::Schema::Num)),
            ])),
        ),
    ]));

/// `runningInstancesForProfile` over the binary wire.
#[wasm_bindgen(js_name = "runningInstancesForProfileWire")]
pub fn running_instances_for_profile_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let instances: Vec<Value> =
            r.list(|r| wire::read_value(r, S_RUNNING_INSTANCES_FOR_PROFILE_0.inner()))?;
        let profile_id: String = r.string16()?;
        let out = &port::running_instances_for_profile(instances, profile_id)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_RUNNING_INSTANCES_FOR_PROFILE_OUT.inner())?;
        }
        Ok(())
    })
}
