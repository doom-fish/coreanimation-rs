use std::error::Error;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use coreanimation::{
    Action, AutoresizingMask, Constraint, ConstraintAttribute, ConstraintLayoutManager,
    ContentsFilter, ContentsFormat, CornerCurve, CornerMask, DynamicRange,
    EdgeAntialiasingMask, Layer, LayerActionKeys, LayerDelegate, Transition, CGRect,
};

fn main() -> Result<(), Box<dyn Error>> {
    let layer = Layer::new().ok_or("failed to create layer")?;
    layer.set_contents_format(ContentsFormat::Gray8Uint);
    layer.set_minification_filter(ContentsFilter::Linear);
    layer.set_magnification_filter(ContentsFilter::Trilinear);
    layer.set_edge_antialiasing_mask(
        EdgeAntialiasingMask::LEFT_EDGE | EdgeAntialiasingMask::TOP_EDGE,
    );
    layer.set_masked_corners(CornerMask::MIN_X_MIN_Y | CornerMask::MAX_X_MAX_Y);
    layer.set_corner_curve(CornerCurve::Continuous);
    layer.set_autoresizing_mask(
        AutoresizingMask::WIDTH_SIZABLE | AutoresizingMask::MIN_Y_MARGIN,
    );
    layer.set_name("advanced-layer");

    if Layer::supports_preferred_dynamic_range() {
        layer.set_preferred_dynamic_range(DynamicRange::High);
        assert_eq!(layer.preferred_dynamic_range(), DynamicRange::High);
    }

    let layout_manager = ConstraintLayoutManager::new().ok_or("failed to create layout manager")?;
    layer.set_layout_manager(Some(&*layout_manager));
    let constraint = Constraint::with_offset(
        ConstraintAttribute::Width,
        "superlayer",
        ConstraintAttribute::Width,
        -10.0,
    )
    .ok_or("failed to create constraint")?;
    layer.set_constraints(&[&constraint]);

    let display_called = Arc::new(AtomicBool::new(false));
    let child = Layer::new().ok_or("failed to create child layer")?;
    child.set_frame(CGRect::new(0.0, 0.0, 4.0, 4.0));
    layer.add_sublayer(&child);

    let mut delegate = LayerDelegate::new().ok_or("failed to create delegate")?;
    delegate.set_display_callback({
        let display_called = Arc::clone(&display_called);
        move |_layer| {
            display_called.store(true, Ordering::SeqCst);
        }
    });
    layer.set_delegate(Some(&delegate));
    layer.display();
    assert!(display_called.load(Ordering::SeqCst));

    let transition = Transition::new().ok_or("failed to create transition")?;
    layer.set_action_for_key(LayerActionKeys::TRANSITION, Some(&transition));
    assert!(layer.action_for_key(LayerActionKeys::TRANSITION).is_some());

    let retained_transition = Action::retained_from(&transition);
    layer.set_action_handle_for_key(LayerActionKeys::TRANSITION, Some(&retained_transition));
    assert!(layer.action_handle_for_key(LayerActionKeys::TRANSITION).is_some());

    let null_action = Action::null().ok_or("failed to create null action")?;
    layer.set_action_handle_for_key(LayerActionKeys::ON_ORDER_OUT, Some(&null_action));
    null_action.run_for_key(LayerActionKeys::ON_ORDER_OUT, &layer);
    layer.clear_action_for_key(LayerActionKeys::ON_ORDER_OUT);

    println!("✅ CALayer advanced surface OK");
    Ok(())
}
