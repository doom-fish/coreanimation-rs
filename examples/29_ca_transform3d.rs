use coreanimation::{CGAffineTransform, Transform3D};

fn main() {
    let affine = CGAffineTransform::new(1.0, 0.25, -0.5, 2.0, 3.0, 4.0);
    let affine_transform = Transform3D::from_affine(affine);
    assert!(affine_transform.is_affine());
    assert_eq!(affine_transform.to_affine(), Some(affine));

    let rotation = Transform3D::rotation(0.25, 0.0, 0.0, 1.0);
    let combined = Transform3D::translation(5.0, 6.0, 0.0)
        .concat(rotation)
        .scaled(2.0, 3.0, 1.0)
        .translated(-1.0, 2.0, 0.0);
    let identity = combined.concat(combined.inverted());
    for (left, right) in identity
        .as_array()
        .into_iter()
        .zip(Transform3D::identity().as_array())
    {
        assert!((left - right).abs() < 1e-9, "{left} != {right}");
    }

    println!("✅ CATransform3D advanced helpers OK");
}
