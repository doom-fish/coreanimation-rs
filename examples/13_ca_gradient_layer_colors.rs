#![allow(clippy::float_cmp)]

use coreanimation::{Color, GradientLayer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let layer = GradientLayer::new().ok_or("failed to create gradient")?;
    let red = Color::red();
    let blue = Color::blue();
    layer.set_colors(&[&red, &blue]);
    let components = layer
        .color_components_at(0)
        .ok_or("missing gradient color")?;

    assert_eq!(components, red.components());
    println!("✅ CAGradientLayer color helpers OK");
    Ok(())
}
