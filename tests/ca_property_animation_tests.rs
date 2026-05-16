use coreanimation::{BasicAnimation, PropertyAnimation, ValueFunction, ValueFunctionName};

#[test]
fn capropertyanimation_value_function_round_trip() {
    let value_function = ValueFunction::new(ValueFunctionName::RotateZ).expect("value function");
    let animation = PropertyAnimation::new(Some("transform")).expect("property animation");
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

    let basic = BasicAnimation::new(Some("transform")).expect("basic animation");
    basic.set_additive(true);
    basic.set_cumulative(true);
    basic.set_value_function(Some(&value_function));

    assert!(basic.additive());
    assert!(basic.cumulative());
    assert_eq!(
        basic.value_function().and_then(|value| value.name()),
        Some(ValueFunctionName::RotateZ)
    );
}
