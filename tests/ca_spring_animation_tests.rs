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
