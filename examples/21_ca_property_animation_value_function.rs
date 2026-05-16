use coreanimation::{BasicAnimation, PropertyAnimation, ValueFunction, ValueFunctionName};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let value_function =
        ValueFunction::new(ValueFunctionName::RotateZ).ok_or("failed to create value function")?;
    let animation =
        PropertyAnimation::new(Some("transform")).ok_or("failed to create property animation")?;
    animation.set_additive(true);
    animation.set_cumulative(true);
    animation.set_value_function(Some(&value_function));

    assert_eq!(animation.key_path().as_deref(), Some("transform"));
    assert!(animation.additive());
    assert!(animation.cumulative());
    assert_eq!(
        animation.value_function().and_then(|value| value.name()),
        Some(ValueFunctionName::RotateZ)
    );

    let basic = BasicAnimation::new(Some("transform")).ok_or("failed to create basic animation")?;
    basic.set_value_function(Some(&value_function));
    assert_eq!(
        basic.value_function().and_then(|value| value.name()),
        Some(ValueFunctionName::RotateZ)
    );

    println!("✅ CAPropertyAnimation / CAValueFunction OK");
    Ok(())
}
