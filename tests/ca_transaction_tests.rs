use coreanimation::{TimingFunctionName, Transaction};

#[test]
fn catransaction_timing_function_round_trip() {
    {
        let _guard = Transaction::lock_guard();
        Transaction::set_animation_duration(0.15);
    }
    Transaction::set_animation_timing_function_name(Some(TimingFunctionName::EaseInEaseOut));
    assert_eq!(
        Transaction::animation_timing_function_name(),
        Some(TimingFunctionName::EaseInEaseOut)
    );
    Transaction::set_animation_timing_function_name(None);
}
