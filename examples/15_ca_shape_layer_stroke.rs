#![allow(clippy::float_cmp)]

use coreanimation::{ShapeFillRule, ShapeLayer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let layer = ShapeLayer::new().ok_or("failed to create shape layer")?;
    layer.set_fill_rule(ShapeFillRule::EvenOdd);
    layer.set_stroke_start(0.2);
    layer.set_stroke_end(0.8);
    layer.set_line_dash_phase(1.5);

    assert_eq!(layer.fill_rule(), ShapeFillRule::EvenOdd);
    assert_eq!(layer.stroke_start(), 0.2);
    assert_eq!(layer.stroke_end(), 0.8);
    assert_eq!(layer.line_dash_phase(), 1.5);
    println!("✅ CAShapeLayer stroke helpers OK");
    Ok(())
}
