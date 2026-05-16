#![allow(clippy::float_cmp)]

use coreanimation::{ShapeFillRule, ShapeLayer};

#[test]
fn cashapelayer_stroke_round_trip() {
    let layer = ShapeLayer::new().expect("shape layer");
    layer.set_fill_rule(ShapeFillRule::EvenOdd);
    layer.set_stroke_start(0.15);
    layer.set_stroke_end(0.85);
    layer.set_line_dash_phase(3.0);

    assert_eq!(layer.fill_rule(), ShapeFillRule::EvenOdd);
    assert_eq!(layer.stroke_start(), 0.15);
    assert_eq!(layer.stroke_end(), 0.85);
    assert_eq!(layer.line_dash_phase(), 3.0);
}
