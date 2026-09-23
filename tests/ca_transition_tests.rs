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

#[test]
fn catransition_filter_accepts_only_core_image_transitions() {
    let transition = Transition::new().expect("transition");
    assert_eq!(transition.filter_name(), None);

    transition
        .set_filter_name(Some("CIDissolveTransition"))
        .expect("dissolve transition");
    assert_eq!(
        transition.filter_name().as_deref(),
        Some("CIDissolveTransition")
    );

    for rejected in ["CIGaussianBlur", "NoSuchFilter", "CI\0DissolveTransition"] {
        assert!(
            transition.set_filter_name(Some(rejected)).is_err(),
            "{rejected}"
        );
    }
    assert_eq!(
        transition.filter_name().as_deref(),
        Some("CIDissolveTransition")
    );

    transition.set_filter_name(None).expect("clear filter");
    assert_eq!(transition.filter_name(), None);
}
