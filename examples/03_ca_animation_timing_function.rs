use coreanimation::{Animation, TimingFunctionName};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let animation = Animation::new().ok_or("failed to create animation")?;
    animation.set_timing_function_name(Some(TimingFunctionName::EaseInEaseOut));
    assert_eq!(
        animation.timing_function_name(),
        Some(TimingFunctionName::EaseInEaseOut)
    );
    animation.set_timing_function_name(None);
    assert_eq!(animation.timing_function_name(), None);
    println!("✅ CAAnimation timing function OK");
    Ok(())
}
