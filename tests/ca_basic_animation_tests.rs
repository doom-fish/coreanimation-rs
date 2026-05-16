#![allow(clippy::float_cmp)]

use coreanimation::BasicAnimation;

#[test]
fn cabasicanimation_numeric_values_round_trip() {
    let animation = BasicAnimation::new(Some("opacity")).expect("animation");
    animation.set_from_number(0.1);
    animation.set_to_number(0.8);
    animation.set_by_number(0.7);
    animation.set_additive(true);
    animation.set_cumulative(true);

    assert_eq!(animation.from_number(), Some(0.1));
    assert_eq!(animation.to_number(), Some(0.8));
    assert_eq!(animation.by_number(), Some(0.7));
    assert!(animation.additive());
    assert!(animation.cumulative());
}
