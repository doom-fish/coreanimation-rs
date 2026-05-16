#![allow(clippy::float_cmp)]

use coreanimation::{Animation, MediaTimingFillMode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let animation = Animation::new().ok_or("failed to create animation")?;
    animation.set_begin_time(1.5);
    animation.set_speed(2.0);
    animation.set_time_offset(0.25);
    animation.set_repeat_duration(4.0);
    animation.set_fill_mode(MediaTimingFillMode::Both);

    assert_eq!(animation.begin_time(), 1.5);
    assert_eq!(animation.speed(), 2.0);
    assert_eq!(animation.time_offset(), 0.25);
    assert_eq!(animation.repeat_duration(), 4.0);
    assert_eq!(animation.fill_mode(), MediaTimingFillMode::Both);
    println!("✅ CAMediaTiming round-trip OK");
    Ok(())
}
