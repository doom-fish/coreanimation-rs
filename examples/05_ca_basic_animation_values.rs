#![allow(clippy::float_cmp)]

use coreanimation::BasicAnimation;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let animation =
        BasicAnimation::new(Some("opacity")).ok_or("failed to create basic animation")?;
    animation.set_from_number(0.2);
    animation.set_to_number(0.9);
    animation.set_by_number(0.7);
    animation.set_additive(true);
    animation.set_cumulative(true);

    assert_eq!(animation.from_number(), Some(0.2));
    assert_eq!(animation.to_number(), Some(0.9));
    assert_eq!(animation.by_number(), Some(0.7));
    assert!(animation.additive());
    assert!(animation.cumulative());
    println!("✅ CABasicAnimation value round-trip OK");
    Ok(())
}
