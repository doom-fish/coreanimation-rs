#![allow(clippy::float_cmp)]

use coreanimation::{Color, ReplicatorLayer, Transform3D};

#[test]
fn careplicatorlayer_properties_round_trip() {
    let layer = ReplicatorLayer::new().expect("replicator");
    let green = Color::green();
    layer.set_instance_count(5);
    layer.set_preserves_depth(true);
    layer.set_instance_delay(0.5);
    layer.set_instance_transform(Transform3D::translation(1.0, 0.0, 0.0));
    layer.set_instance_color(Some(&green));
    layer.set_instance_red_offset(0.1);
    layer.set_instance_green_offset(0.2);
    layer.set_instance_blue_offset(0.3);
    layer.set_instance_alpha_offset(-0.1);

    assert_eq!(layer.instance_count(), 5);
    assert!(layer.preserves_depth());
    assert_eq!(layer.instance_delay(), 0.5);
    assert_eq!(
        layer.instance_transform(),
        Transform3D::translation(1.0, 0.0, 0.0)
    );
    assert_eq!(
        layer.instance_color().expect("color").components(),
        green.components()
    );
    assert_eq!(layer.instance_red_offset(), 0.1);
    assert_eq!(layer.instance_green_offset(), 0.2);
    assert_eq!(layer.instance_blue_offset(), 0.3);
    assert_eq!(layer.instance_alpha_offset(), -0.1);
}
