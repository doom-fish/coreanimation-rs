use apple_metal::MetalDevice;
use coreanimation::MetalLayer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let layer = MetalLayer::new().ok_or("failed to create metal layer")?;
    let device = MetalDevice::system_default().ok_or("no Metal device available")?;
    layer.set_device(Some(&device));
    layer.set_framebuffer_only(false);
    layer.set_maximum_drawable_count(3);
    layer.set_presents_with_transaction(true);
    layer.set_display_sync_enabled(false);
    layer.set_allows_next_drawable_timeout(false);

    assert!(!layer.framebuffer_only());
    assert_eq!(layer.maximum_drawable_count(), 3);
    assert!(layer.presents_with_transaction());
    assert!(!layer.display_sync_enabled());
    assert!(!layer.allows_next_drawable_timeout());
    println!("✅ CAMetalLayer properties OK");
    Ok(())
}
