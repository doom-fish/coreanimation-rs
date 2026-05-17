#![allow(clippy::float_cmp)]

use apple_metal::MetalDevice;
use coreanimation::{
    current_media_time, Animation, AnimationDelegate, FrameRateRange, MetalDisplayLink,
    MetalLayer, CGSize,
};

#[test]
fn caanimationdelegate_and_caframeraterange_round_trip() {
    let animation = Animation::new().expect("animation");
    let mut delegate = AnimationDelegate::new().expect("delegate");
    delegate.set_did_start(|_animation| {});
    delegate.set_did_stop(|_animation, _finished| {});
    animation.set_delegate(Some(&delegate));
    animation.set_delegate(None);

    assert!(FrameRateRange::DEFAULT.is_equal_to_range(FrameRateRange::default()));

    let range = FrameRateRange::make(30.0, 120.0, 60.0);
    assert!(range.is_equal_to_range(FrameRateRange::new(30.0, 120.0, 60.0)));

    if Animation::supports_preferred_frame_rate_range() {
        animation.set_preferred_frame_rate_range(range);
        assert_eq!(animation.preferred_frame_rate_range(), range);
    }

    if MetalDisplayLink::is_available() {
        if let Some(device) = MetalDevice::system_default() {
            let layer = MetalLayer::new().expect("metal layer");
            layer.set_device(Some(&device));
            layer.set_drawable_size(CGSize::new(32.0, 32.0));

            let link = MetalDisplayLink::new(&layer).expect("metal display link");
            link.set_preferred_frame_rate_range(range);
            assert_eq!(link.preferred_frame_rate_range(), range);
        }
    }

    assert!(current_media_time() >= 0.0);
}
