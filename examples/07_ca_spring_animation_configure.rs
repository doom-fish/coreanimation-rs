#![allow(clippy::float_cmp)]

use coreanimation::SpringAnimation;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let animation = SpringAnimation::new(Some("position.y")).ok_or("failed to create spring")?;
    animation.configure(2.0, 120.0, 16.0, 1.5);

    assert_eq!(animation.mass(), 2.0);
    assert_eq!(animation.stiffness(), 120.0);
    assert_eq!(animation.damping(), 16.0);
    assert_eq!(animation.initial_velocity(), 1.5);
    println!("✅ CASpringAnimation configure OK");
    Ok(())
}
