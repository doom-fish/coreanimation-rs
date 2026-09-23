#![allow(clippy::float_cmp)]

use coreanimation::{Layer, Transform3D};

#[test]
fn calayer_round_trip_properties() {
    let layer = Layer::new().expect("layer");
    layer.set_z_position(3.5);
    layer.set_anchor_point_z(1.25);
    layer.set_double_sided(false);
    layer.set_geometry_flipped(true);
    layer.set_sublayer_transform(Transform3D::translation(1.0, 2.0, 3.0));

    assert_eq!(layer.z_position(), 3.5);
    assert_eq!(layer.anchor_point_z(), 1.25);
    assert!(!layer.is_double_sided());
    assert!(layer.is_geometry_flipped());
    assert_eq!(
        layer.sublayer_transform(),
        Transform3D::translation(1.0, 2.0, 3.0)
    );
}

#[test]
fn contents_that_are_not_images_read_as_none() {
    let layer = Layer::new().expect("layer");
    layer.set_frame(coreanimation::CGRect::new(0.0, 0.0, 10.0, 10.0));
    layer.set_needs_display();
    layer.display();

    let contents = layer.contents();

    assert!(
        contents.is_none(),
        "backing store surfaced as a CGImage of width {:?}",
        contents.map(|image| image.width())
    );
}
