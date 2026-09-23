#![allow(clippy::float_cmp)]

use coreanimation::{CGPoint, CGRect, Layer, Transform3D};

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
    layer
        .set_frame(coreanimation::CGRect::new(0.0, 0.0, 10.0, 10.0))
        .expect("frame");
    layer.set_needs_display();
    layer.display();

    let contents = layer.contents();

    assert!(
        contents.is_none(),
        "backing store surfaced as a CGImage of width {:?}",
        contents.map(|image| image.width())
    );
}

#[test]
fn non_finite_geometry_is_refused_and_leaves_the_layer_unchanged() {
    let layer = Layer::new().expect("layer");
    layer
        .set_frame(CGRect::new(1.0, 2.0, 30.0, 40.0))
        .expect("finite frame");
    let frame = layer.frame();

    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(layer.set_frame(CGRect::new(value, 0.0, 1.0, 1.0)).is_err());
        assert!(layer.set_frame(CGRect::new(0.0, 0.0, value, 1.0)).is_err());
        assert!(layer.set_bounds(CGRect::new(0.0, 0.0, 1.0, value)).is_err());
        assert!(layer.set_position(CGPoint::new(0.0, value)).is_err());
        assert!(layer.set_anchor_point(CGPoint::new(value, 0.5)).is_err());
        let mut transform = Transform3D::identity();
        transform.m11 = value;
        assert!(layer.set_transform(transform).is_err());
    }

    assert_eq!(layer.frame(), frame);
    assert_eq!(layer.anchor_point(), CGPoint::new(0.5, 0.5));
    assert_eq!(layer.transform(), Transform3D::identity());
    layer
        .set_position(CGPoint::new(5.0, 6.0))
        .expect("finite position");
    assert_eq!(layer.position(), CGPoint::new(5.0, 6.0));
}

#[test]
fn sublayer_cycles_are_refused() {
    let root = Layer::new().expect("root");
    let child = Layer::new().expect("child");
    let grandchild = Layer::new().expect("grandchild");
    root.add_sublayer(&child).expect("child");
    child.add_sublayer(&grandchild).expect("grandchild");

    assert!(root.add_sublayer(&root).is_err());
    assert!(child.add_sublayer(&root).is_err());
    assert!(grandchild.add_sublayer(&root).is_err());
    assert!(grandchild.add_sublayer(&child).is_err());

    assert_eq!(root.sublayers().len(), 1);
    assert_eq!(child.sublayers().len(), 1);
    assert!(grandchild.sublayers().is_empty());
    let sibling = Layer::new().expect("sibling");
    root.add_sublayer(&sibling).expect("sibling");
    assert_eq!(root.sublayers().len(), 2);
}

#[test]
fn masks_must_be_detached_and_outside_the_ancestry() {
    let root = Layer::new().expect("root");
    let child = Layer::new().expect("child");
    let other = Layer::new().expect("other");
    root.add_sublayer(&child).expect("child");

    assert!(child.set_mask(Some(&root)).is_err());
    assert!(root.set_mask(Some(&root)).is_err());
    assert!(root.set_mask(Some(&child)).is_err());
    assert!(root.mask().is_none());
    assert!(child.mask().is_none());

    let mask = Layer::new().expect("mask");
    root.set_mask(Some(&mask)).expect("detached mask");
    root.set_mask(Some(&mask)).expect("same mask again");
    assert!(other.set_mask(Some(&mask)).is_err());
    assert!(root.mask().is_some());
    root.set_mask(None::<&Layer>).expect("clear mask");
    assert!(root.mask().is_none());
    other.set_mask(Some(&mask)).expect("mask is detached again");
}
