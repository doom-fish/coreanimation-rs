#![allow(clippy::float_cmp)]

use coreanimation::{Animation, TimingFunction, TimingFunctionName, Transaction};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let animation = Animation::new().ok_or("failed to create animation")?;
    let custom =
        TimingFunction::with_control_points(0.2, 0.4, 0.6, 0.8).ok_or("timing function")?;
    animation.set_timing_function(Some(&custom));

    let roundtrip = animation
        .timing_function()
        .ok_or("missing timing function")?;
    assert_eq!(roundtrip.name(), None);
    assert_eq!(roundtrip.control_point(1), Some((0.2, 0.4)));
    assert_eq!(roundtrip.control_point(2), Some((0.6, 0.8)));

    let named = TimingFunction::with_name(TimingFunctionName::EaseIn).ok_or("named function")?;
    Transaction::set_animation_timing_function(Some(&named));
    assert_eq!(
        Transaction::animation_timing_function().and_then(|value| value.name()),
        Some(TimingFunctionName::EaseIn)
    );
    Transaction::set_animation_timing_function(None);

    println!("✅ CAMediaTimingFunction objects OK");
    Ok(())
}
