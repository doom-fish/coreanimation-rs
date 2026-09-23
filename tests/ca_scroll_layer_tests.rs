#![allow(clippy::float_cmp)]

use coreanimation::{CGPoint, CGRect, ScrollLayer, ScrollMode};

#[test]
fn cascrolllayer_visible_rect_round_trip() {
    let layer = ScrollLayer::new().expect("scroll layer");
    layer
        .set_bounds(CGRect::new(0.0, 0.0, 100.0, 80.0))
        .expect("bounds");
    layer.set_scroll_mode(ScrollMode::Both);
    layer
        .scroll_to_point(CGPoint::new(10.0, 20.0))
        .expect("scroll point");
    assert_eq!(layer.scroll_mode(), ScrollMode::Both);
    assert_eq!(layer.visible_rect(), CGRect::new(10.0, 20.0, 100.0, 80.0));
    layer
        .scroll_to_rect(CGRect::new(2.0, 4.0, 50.0, 25.0))
        .expect("scroll rect");
    assert_eq!(layer.visible_rect(), CGRect::new(2.0, 4.0, 50.0, 25.0));
}

#[test]
fn non_finite_scroll_targets_are_refused() {
    let layer = ScrollLayer::new().expect("scroll layer");
    layer
        .set_bounds(CGRect::new(0.0, 0.0, 100.0, 80.0))
        .expect("bounds");

    assert!(layer.scroll_to_point(CGPoint::new(f64::NAN, 0.0)).is_err());
    assert!(layer
        .scroll_to_rect(CGRect::new(0.0, f64::INFINITY, 10.0, 10.0))
        .is_err());
    assert_eq!(layer.visible_rect(), CGRect::new(0.0, 0.0, 100.0, 80.0));
}
