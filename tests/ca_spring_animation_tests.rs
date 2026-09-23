#![allow(clippy::float_cmp)]

use coreanimation::SpringAnimation;

#[test]
fn caspringanimation_configure_updates_fields() {
    let animation = SpringAnimation::new(Some("position.y")).expect("animation");
    animation.configure(2.0, 120.0, 12.0, 1.5);

    assert_eq!(animation.mass(), 2.0);
    assert_eq!(animation.stiffness(), 120.0);
    assert_eq!(animation.damping(), 12.0);
    assert_eq!(animation.initial_velocity(), 1.5);
}

#[test]
fn perceptual_spring_reports_its_parameters() {
    if !SpringAnimation::supports_perceptual_parameters() {
        assert!(SpringAnimation::with_perceptual_duration(Some("position.y"), 0.5, 0.25).is_err());
        return;
    }
    let animation = SpringAnimation::with_perceptual_duration(Some("position.y"), 0.5, 0.25)
        .expect("perceptual spring");

    assert_eq!(animation.key_path().as_deref(), Some("position.y"));
    assert!(
        (animation
            .perceptual_duration()
            .expect("perceptual duration")
            - 0.5)
            .abs()
            < 1e-9
    );
    assert!((animation.bounce().expect("bounce") - 0.25).abs() < 1e-9);
    let expected_stiffness = (2.0 * std::f64::consts::PI / 0.5).powi(2);
    assert!((animation.stiffness() - expected_stiffness).abs() < 1e-6);
    assert!(animation.allows_overdamping());
    animation
        .set_allows_overdamping(false)
        .expect("allows overdamping");
    assert!(!animation.allows_overdamping());
}

#[test]
fn perceptual_spring_rejects_invalid_parameters() {
    for (duration, bounce) in [
        (0.0, 0.0),
        (-1.0, 0.0),
        (f64::NAN, 0.0),
        (f64::INFINITY, 0.0),
        (0.5, f64::NAN),
        (0.5, f64::NEG_INFINITY),
    ] {
        assert!(
            SpringAnimation::with_perceptual_duration(None, duration, bounce).is_err(),
            "accepted duration {duration} bounce {bounce}"
        );
    }
    assert!(SpringAnimation::with_perceptual_duration(Some("posi\0tion"), 0.5, 0.0).is_err());
}
