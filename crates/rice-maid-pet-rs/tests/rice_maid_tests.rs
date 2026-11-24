use rice_maid_pet_rs::{
    FRAME_DURATIONS, PetPosition, RiceMaidBackend, clamp_pet_size, clamp_position, frame_transform,
    pre_model_request,
};
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn contract_invariants_match_rice_maid_pet() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let ev_clone = Rc::clone(&events);
    let mut backend = RiceMaidBackend::new(move |name, st| {
        ev_clone.borrow_mut().push((name.to_string(), st));
    })
    .expect("backend");
    let init = backend.get_state();
    assert_eq!(init.count, 0);
    assert_eq!(init.at, 0);
    assert_eq!(init.epoch.len(), 36);
    assert!(events.borrow().is_empty());

    for i in 1..=128 {
        let s = backend.request_bite(10_000 + i);
        assert_eq!(s.count, i);
        assert_eq!(s.epoch, init.epoch);
    }
    assert_eq!(events.borrow().len(), 128);
    assert_eq!(backend.get_state().count, 128);

    // Distinct backend starts get distinct UUID epochs
    let second = RiceMaidBackend::new(|_, _| {}).expect("second");
    assert_ne!(init.epoch, second.get_state().epoch);

    // PreModelRequest hook ignores payload & swallows service failures cleanly
    assert_eq!(
        pre_model_request(|| Err("service unavailable".into())),
        None
    );
    assert_eq!(pre_model_request(|| Ok(second.get_state())), None);

    // Sprite size & viewport clamping
    assert_eq!(clamp_pet_size(None), 192.0);
    assert_eq!(clamp_pet_size(Some(50.0)), 112.0);
    assert_eq!(clamp_pet_size(Some(500.0)), 280.0);
    let pos = clamp_position(
        Some(PetPosition { x: 9999.0, y: 0.0 }),
        1200.0,
        900.0,
        192.0,
    );
    assert!(pos.x <= 1200.0 - 192.0 - 8.0);
    assert!(pos.y >= 48.0);

    // Non-finite / negative inputs and short-viewport bottom GAP clamp
    let bad_vp = rice_maid_pet_rs::viewport_size(f64::INFINITY, f64::NAN, -50.0);
    assert!(bad_vp.width.is_finite() && bad_vp.height.is_finite());
    assert!(bad_vp.pet_size > 0.0 && bad_vp.pet_height > 0.0);
    let bad_pos = clamp_position(
        Some(PetPosition {
            x: f64::NAN,
            y: f64::INFINITY,
        }),
        -200.0,
        f64::NEG_INFINITY,
        f64::NAN,
    );
    assert!(bad_pos.x.is_finite() && bad_pos.x >= 0.0);
    assert!(bad_pos.y.is_finite() && bad_pos.y >= 0.0);

    // Short viewport where height - pet_height - GAP < TOP must still preserve bottom GAP
    let short_vp = rice_maid_pet_rs::viewport_size(200.0, 55.0, 10.0);
    let short_pos = clamp_position(Some(PetPosition { x: 10.0, y: 999.0 }), 200.0, 55.0, 10.0);
    assert!(
        short_pos.y + short_vp.pet_height <= short_vp.height - rice_maid_pet_rs::GAP + 1e-9,
        "bottom GAP violated: y={} + pet_h={} > h - GAP={}",
        short_pos.y,
        short_vp.pet_height,
        short_vp.height - rice_maid_pet_rs::GAP
    );

    assert_eq!(FRAME_DURATIONS.len(), 11);
    assert_eq!(
        frame_transform(5),
        format!("translate3d(-25%, -{}%, 0)", 100.0 / 3.0)
    );
}
