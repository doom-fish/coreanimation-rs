#![allow(clippy::float_cmp)]

use coreanimation::{Animation, MediaTimingFillMode};

#[test]
fn camediatiming_properties_round_trip() {
    let animation = Animation::new().expect("animation");
    animation.set_begin_time(1.0);
    animation.set_speed(1.5);
    animation.set_time_offset(0.25);
    animation.set_repeat_duration(2.5);
    animation.set_fill_mode(MediaTimingFillMode::Forwards);

    assert_eq!(animation.begin_time(), 1.0);
    assert_eq!(animation.speed(), 1.5);
    assert_eq!(animation.time_offset(), 0.25);
    assert_eq!(animation.repeat_duration(), 2.5);
    assert_eq!(animation.fill_mode(), MediaTimingFillMode::Forwards);
}
