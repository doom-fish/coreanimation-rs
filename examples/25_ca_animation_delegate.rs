use std::error::Error;

use apple_metal::MetalDevice;
use coreanimation::{
    current_media_time, Animation, AnimationDelegate, CGSize, FrameRateRange, MetalDisplayLink,
    MetalLayer,
};

fn main() -> Result<(), Box<dyn Error>> {
    let animation = Animation::new().ok_or("failed to create animation")?;
    let mut delegate = AnimationDelegate::new().ok_or("failed to create delegate")?;
    delegate.set_did_start(|_animation| println!("animation delegate installed"));
    delegate
        .set_did_stop(|_animation, finished| println!("animation delegate cleared: {finished}"));
    animation.set_delegate(Some(&delegate));

    let range = FrameRateRange::make(30.0, 120.0, 60.0);
    assert!(FrameRateRange::DEFAULT.is_equal_to_range(FrameRateRange::default()));

    if Animation::supports_preferred_frame_rate_range() {
        animation.set_preferred_frame_rate_range(range);
        assert_eq!(animation.preferred_frame_rate_range(), range);
        println!("✅ CAAnimation preferredFrameRateRange OK");
    } else {
        println!("ℹ️ CAAnimation preferredFrameRateRange unavailable on this macOS version");
    }

    if MetalDisplayLink::is_available() {
        if let Some(device) = MetalDevice::system_default() {
            let layer = MetalLayer::new().ok_or("failed to create metal layer")?;
            layer.set_device(Some(&device));
            layer.set_drawable_size(CGSize::new(32.0, 32.0));
            let link = MetalDisplayLink::new(&layer).ok_or("failed to create display link")?;
            link.set_preferred_frame_rate_range(range);
            assert_eq!(link.preferred_frame_rate_range(), range);
            println!("✅ CAMetalDisplayLink preferredFrameRateRange OK");
        }
    }

    println!("current media time: {:.6}", current_media_time());
    Ok(())
}
