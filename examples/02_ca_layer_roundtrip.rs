#![allow(clippy::float_cmp)]

use coreanimation::{Layer, Transform3D};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let layer = Layer::new().ok_or("failed to create layer")?;
    layer.set_z_position(3.5);
    layer.set_anchor_point_z(1.25);
    layer.set_double_sided(false);
    layer.set_geometry_flipped(true);
    layer.set_sublayer_transform(Transform3D::translation(4.0, 5.0, 6.0));

    assert_eq!(layer.z_position(), 3.5);
    assert_eq!(layer.anchor_point_z(), 1.25);
    assert!(!layer.is_double_sided());
    assert!(layer.is_geometry_flipped());
    assert_eq!(
        layer.sublayer_transform(),
        Transform3D::translation(4.0, 5.0, 6.0)
    );
    println!("✅ CALayer round-trip OK");
    Ok(())
}
