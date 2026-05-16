#![allow(clippy::float_cmp)]

use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc,
};

use apple_metal::MetalDevice;
use coreanimation::{CGSize, MetalDisplayLink, MetalLayer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !MetalDisplayLink::is_available() {
        println!("ℹ️ CAMetalDisplayLink unavailable on this macOS version");
        return Ok(());
    }

    let layer = MetalLayer::new().ok_or("failed to create metal layer")?;
    let device = MetalDevice::system_default().ok_or("no Metal device available")?;
    layer.set_device(Some(&device));
    layer.set_drawable_size(CGSize::new(64.0, 64.0));

    let mut link = MetalDisplayLink::new(&layer).ok_or("failed to create metal display link")?;
    let callback_count = Arc::new(AtomicUsize::new(0));
    let saw_texture = Arc::new(AtomicBool::new(false));
    let callback_count_clone = Arc::clone(&callback_count);
    let saw_texture_clone = Arc::clone(&saw_texture);

    link.set_delegate(move |update| {
        callback_count_clone.fetch_add(1, Ordering::SeqCst);
        let Some(drawable) = update.drawable() else {
            return;
        };
        let Some(texture) = drawable.texture() else {
            return;
        };
        if texture.width() == 64 {
            saw_texture_clone.store(true, Ordering::SeqCst);
        }
    });
    link.set_preferred_frame_latency(1.0);
    link.add_to_current_run_loop();
    MetalDisplayLink::run_current_run_loop_for(0.1);
    link.set_paused(true);
    link.remove_from_current_run_loop();
    link.clear_delegate();

    assert!(callback_count.load(Ordering::SeqCst) > 0);
    assert!(saw_texture.load(Ordering::SeqCst));
    assert!(link.is_paused());
    assert_eq!(link.preferred_frame_latency(), 1.0);
    println!("✅ CAMetalDisplayLink callbacks OK");
    Ok(())
}
