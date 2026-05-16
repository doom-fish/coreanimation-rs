#![allow(clippy::float_cmp)]

use coreanimation::{Color, ReplicatorLayer, Transform3D};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let layer = ReplicatorLayer::new().ok_or("failed to create replicator")?;
    let green = Color::green();
    layer.set_instance_count(4);
    layer.set_preserves_depth(true);
    layer.set_instance_delay(0.2);
    layer.set_instance_transform(Transform3D::translation(1.0, 2.0, 3.0));
    layer.set_instance_color(Some(&green));
    layer.set_instance_red_offset(0.1);
    layer.set_instance_green_offset(0.2);
    layer.set_instance_blue_offset(0.3);
    layer.set_instance_alpha_offset(-0.1);

    assert_eq!(layer.instance_count(), 4);
    assert!(layer.preserves_depth());
    assert_eq!(layer.instance_delay(), 0.2);
    assert_eq!(
        layer.instance_transform(),
        Transform3D::translation(1.0, 2.0, 3.0)
    );
    assert_eq!(
        layer.instance_color().ok_or("missing color")?.components(),
        green.components()
    );
    assert_eq!(layer.instance_red_offset(), 0.1);
    assert_eq!(layer.instance_green_offset(), 0.2);
    assert_eq!(layer.instance_blue_offset(), 0.3);
    assert_eq!(layer.instance_alpha_offset(), -0.1);
    println!("✅ CAReplicatorLayer properties OK");
    Ok(())
}
