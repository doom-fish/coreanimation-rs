use coreanimation::{Transition, TransitionSubtype};

#[test]
fn catransition_has_and_clears_subtype() {
    let transition = Transition::new().expect("transition");
    assert!(!transition.has_subtype());
    transition.set_subtype(TransitionSubtype::FromRight);
    assert!(transition.has_subtype());
    transition.clear_subtype();
    assert!(!transition.has_subtype());
}
