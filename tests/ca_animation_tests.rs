use coreanimation::{Animation, TimingFunctionName};

#[test]
fn caanimation_timing_function_round_trip() {
    let animation = Animation::new().expect("animation");
    animation.set_timing_function_name(Some(TimingFunctionName::EaseOut));
    assert_eq!(
        animation.timing_function_name(),
        Some(TimingFunctionName::EaseOut)
    );
    animation.set_timing_function_name(None);
    assert_eq!(animation.timing_function_name(), None);
}
