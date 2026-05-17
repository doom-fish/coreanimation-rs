#![allow(clippy::float_cmp)]

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use coreanimation::{
    Action, AutoresizingMask, CGRect, Constraint, ConstraintAttribute, ConstraintLayoutManager,
    ContentsFilter, ContentsFormat, CornerCurve, CornerMask, DynamicRange, EdgeAntialiasingMask,
    Layer, LayerActionKeys, LayerDelegate, Transition,
};

#[test]
fn calayer_advanced_surface_round_trip() {
    let layer = Layer::new().expect("layer");
    layer.set_contents_format(ContentsFormat::Gray8Uint);
    layer.set_minification_filter(ContentsFilter::Linear);
    layer.set_magnification_filter(ContentsFilter::Trilinear);
    layer.set_edge_antialiasing_mask(
        EdgeAntialiasingMask::LEFT_EDGE | EdgeAntialiasingMask::TOP_EDGE,
    );
    layer.set_masked_corners(CornerMask::MIN_X_MIN_Y | CornerMask::MAX_X_MAX_Y);
    layer.set_corner_curve(CornerCurve::Continuous);
    layer.set_autoresizing_mask(AutoresizingMask::WIDTH_SIZABLE | AutoresizingMask::MIN_Y_MARGIN);
    layer.set_name("advanced-layer");

    assert_eq!(layer.contents_format(), ContentsFormat::Gray8Uint);
    assert_eq!(layer.minification_filter(), ContentsFilter::Linear);
    assert_eq!(layer.magnification_filter(), ContentsFilter::Trilinear);
    assert!(layer
        .edge_antialiasing_mask()
        .contains(EdgeAntialiasingMask::LEFT_EDGE));
    assert!(layer
        .edge_antialiasing_mask()
        .contains(EdgeAntialiasingMask::TOP_EDGE));
    assert!(layer.masked_corners().contains(CornerMask::MIN_X_MIN_Y));
    assert!(layer.masked_corners().contains(CornerMask::MAX_X_MAX_Y));
    assert_eq!(layer.corner_curve(), CornerCurve::Continuous);
    assert!(Layer::corner_curve_expansion_factor(CornerCurve::Continuous) > 0.0);
    assert!(layer
        .autoresizing_mask()
        .contains(AutoresizingMask::WIDTH_SIZABLE));
    assert!(layer
        .autoresizing_mask()
        .contains(AutoresizingMask::MIN_Y_MARGIN));
    assert_eq!(layer.name().as_deref(), Some("advanced-layer"));

    if Layer::supports_preferred_dynamic_range() {
        layer.set_preferred_dynamic_range(DynamicRange::High);
        assert_eq!(layer.preferred_dynamic_range(), DynamicRange::High);
    }

    let layout_manager = ConstraintLayoutManager::new().expect("layout manager");
    layer.set_layout_manager(Some(&*layout_manager));
    assert!(layer.layout_manager().is_some());

    let constraint = Constraint::with_offset(
        ConstraintAttribute::Width,
        "superlayer",
        ConstraintAttribute::Width,
        -10.0,
    )
    .expect("constraint");
    layer.set_constraints(&[&constraint]);
    let constraints = layer.constraints();
    assert_eq!(constraints.len(), 1);
    assert_eq!(constraints[0].attribute(), ConstraintAttribute::Width);
    assert_eq!(constraints[0].source_name().as_deref(), Some("superlayer"));
    assert_eq!(
        constraints[0].source_attribute(),
        ConstraintAttribute::Width
    );
    assert_eq!(constraints[0].scale(), 1.0);
    assert_eq!(constraints[0].offset(), -10.0);

    let display_called = Arc::new(AtomicBool::new(false));
    let child = Layer::new().expect("child layer");
    child.set_frame(CGRect::new(0.0, 0.0, 4.0, 4.0));
    layer.add_sublayer(&child);

    let mut delegate = LayerDelegate::new().expect("delegate");
    delegate.set_display_callback({
        let display_called = Arc::clone(&display_called);
        move |_layer| {
            display_called.store(true, Ordering::SeqCst);
        }
    });
    layer.set_delegate(Some(&delegate));

    layer.display();

    assert!(display_called.load(Ordering::SeqCst));

    let transition = Transition::new().expect("transition");
    layer.set_action_for_key(LayerActionKeys::TRANSITION, Some(&transition));
    assert!(layer.action_for_key(LayerActionKeys::TRANSITION).is_some());

    let retained_transition = Action::retained_from(&transition);
    layer.set_action_handle_for_key(LayerActionKeys::TRANSITION, Some(&retained_transition));
    assert!(layer
        .action_handle_for_key(LayerActionKeys::TRANSITION)
        .is_some());

    let null_action = Action::null().expect("null action");
    layer.set_action_handle_for_key(LayerActionKeys::ON_ORDER_OUT, Some(&null_action));
    null_action.run_for_key(LayerActionKeys::ON_ORDER_OUT, &layer);

    layer.clear_action_for_key(LayerActionKeys::ON_ORDER_OUT);
    assert!(layer
        .action_handle_for_key(LayerActionKeys::ON_ORDER_OUT)
        .is_none());
}
