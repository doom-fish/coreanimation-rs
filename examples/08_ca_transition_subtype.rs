use coreanimation::{Transition, TransitionSubtype};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let transition = Transition::new().ok_or("failed to create transition")?;
    assert!(!transition.has_subtype());
    transition.set_subtype(TransitionSubtype::FromLeft);
    assert!(transition.has_subtype());
    transition.clear_subtype();
    assert!(!transition.has_subtype());
    println!("✅ CATransition subtype helpers OK");
    Ok(())
}
