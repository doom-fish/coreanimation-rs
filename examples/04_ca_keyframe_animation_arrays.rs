#![allow(clippy::float_cmp)]

use coreanimation::{KeyframeAnimation, TimingFunctionName};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let animation =
        KeyframeAnimation::new(Some("position.x")).ok_or("failed to create keyframes")?;
    animation.set_timing_function_names(&[TimingFunctionName::EaseIn, TimingFunctionName::EaseOut]);
    animation.set_tension_values(&[0.1, 0.2, 0.3]);
    animation.set_continuity_values(&[-0.1, 0.0, 0.1]);
    animation.set_bias_values(&[0.25, 0.5, 0.75]);

    assert_eq!(
        animation.timing_function_names(),
        vec![TimingFunctionName::EaseIn, TimingFunctionName::EaseOut]
    );
    assert_eq!(animation.tension_values(), vec![0.1, 0.2, 0.3]);
    assert_eq!(animation.continuity_values(), vec![-0.1, 0.0, 0.1]);
    assert_eq!(animation.bias_values(), vec![0.25, 0.5, 0.75]);
    println!("✅ CAKeyframeAnimation arrays OK");
    Ok(())
}
