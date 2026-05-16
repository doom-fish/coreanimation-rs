#![allow(clippy::float_cmp)]

use coreanimation::TransformLayer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let layer = TransformLayer::new().ok_or("failed to create transform layer")?;
    layer.set_z_position(5.0);
    assert_eq!(layer.z_position(), 5.0);
    println!("✅ CATransformLayer creation OK");
    Ok(())
}
