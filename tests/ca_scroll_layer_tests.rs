#![allow(clippy::float_cmp)]

use coreanimation::{CGPoint, CGRect, ScrollLayer, ScrollMode};

#[test]
fn cascrolllayer_visible_rect_round_trip() {
    let layer = ScrollLayer::new().expect("scroll layer");
    layer.set_bounds(CGRect::new(0.0, 0.0, 100.0, 80.0));
    layer.set_scroll_mode(ScrollMode::Both);
    layer.scroll_to_point(CGPoint::new(10.0, 20.0));
    assert_eq!(layer.scroll_mode(), ScrollMode::Both);
    assert_eq!(layer.visible_rect(), CGRect::new(10.0, 20.0, 100.0, 80.0));
    layer.scroll_to_rect(CGRect::new(2.0, 4.0, 50.0, 25.0));
    assert_eq!(layer.visible_rect(), CGRect::new(2.0, 4.0, 50.0, 25.0));
}
