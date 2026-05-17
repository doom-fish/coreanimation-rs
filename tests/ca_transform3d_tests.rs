use coreanimation::{CGAffineTransform, Transform3D};

fn assert_transform_close(lhs: Transform3D, rhs: Transform3D) {
    for (left, right) in lhs.as_array().into_iter().zip(rhs.as_array()) {
        assert!((left - right).abs() < 1e-9, "{left} != {right}");
    }
}

#[test]
fn catransform3d_advanced_helpers_round_trip() {
    let affine = CGAffineTransform::new(1.0, 0.25, -0.5, 2.0, 3.0, 4.0);
    let affine_transform = Transform3D::from_affine(affine);
    assert!(affine_transform.is_affine());
    assert_eq!(affine_transform.to_affine(), Some(affine));

    let rotation = Transform3D::rotation(0.25, 0.0, 0.0, 1.0);
    let rotated = Transform3D::identity().rotated(0.25, 0.0, 0.0, 1.0);
    assert_transform_close(rotation, rotated);

    let combined = Transform3D::translation(5.0, 6.0, 0.0)
        .concat(rotation)
        .scaled(2.0, 3.0, 1.0)
        .translated(-1.0, 2.0, 0.0);
    let identity = combined.concat(combined.inverted());
    assert_transform_close(identity, Transform3D::identity());
}
