use coreanimation::{AnimationGroup, BasicAnimation};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let group = AnimationGroup::new().ok_or("failed to create group")?;
    let first = BasicAnimation::new(Some("opacity")).ok_or("failed to create animation")?;
    let second = BasicAnimation::new(Some("position.x")).ok_or("failed to create animation")?;

    assert!(group.is_empty());
    group.push(&first);
    group.push(&second);
    assert_eq!(group.len(), 2);
    group.clear();
    assert!(group.is_empty());
    println!("✅ CAAnimationGroup collection helpers OK");
    Ok(())
}
