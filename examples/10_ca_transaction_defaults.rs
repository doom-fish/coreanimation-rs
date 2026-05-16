use coreanimation::{TimingFunctionName, Transaction};

fn main() {
    {
        let _guard = Transaction::lock_guard();
        Transaction::set_animation_duration(0.3);
        Transaction::set_disable_actions(true);
    }

    Transaction::set_animation_timing_function_name(Some(TimingFunctionName::EaseIn));
    assert_eq!(
        Transaction::animation_timing_function_name(),
        Some(TimingFunctionName::EaseIn)
    );
    Transaction::set_animation_timing_function_name(None);
    println!("✅ CATransaction helpers OK");
}
