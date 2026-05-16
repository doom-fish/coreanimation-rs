use coreanimation::{AnimationGroup, BasicAnimation};

#[test]
fn caanimationgroup_push_and_clear() {
    let group = AnimationGroup::new().expect("group");
    let first = BasicAnimation::new(Some("opacity")).expect("animation");
    let second = BasicAnimation::new(Some("position.x")).expect("animation");

    assert!(group.is_empty());
    group.push(&first);
    group.push(&second);
    assert_eq!(group.len(), 2);
    group.clear();
    assert!(group.is_empty());
}
