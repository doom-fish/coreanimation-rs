#![allow(clippy::float_cmp)]

use coreanimation::{Color, GradientLayer};

#[test]
fn cagradientlayer_reads_color_components() {
    let layer = GradientLayer::new().expect("gradient");
    let red = Color::red();
    let blue = Color::blue();
    layer.set_colors(&[&red, &blue]);

    assert_eq!(layer.color_components_at(0), Some(red.components()));
    assert_eq!(layer.color_components_at(1), Some(blue.components()));
}
