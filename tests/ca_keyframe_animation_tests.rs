#![allow(clippy::float_cmp)]

use coreanimation::{KeyframeAnimation, TimingFunctionName};

#[test]
fn cakeyframeanimation_extended_arrays_round_trip() {
    let animation = KeyframeAnimation::new(Some("position.x")).expect("animation");
    animation.set_timing_function_names(&[TimingFunctionName::EaseIn, TimingFunctionName::EaseOut]);
    animation.set_tension_values(&[0.1, 0.2]);
    animation.set_continuity_values(&[-0.2, 0.3]);
    animation.set_bias_values(&[0.4, 0.5]);

    assert_eq!(
        animation.timing_function_names(),
        vec![TimingFunctionName::EaseIn, TimingFunctionName::EaseOut]
    );
    assert_eq!(animation.tension_values(), vec![0.1, 0.2]);
    assert_eq!(animation.continuity_values(), vec![-0.2, 0.3]);
    assert_eq!(animation.bias_values(), vec![0.4, 0.5]);
}
