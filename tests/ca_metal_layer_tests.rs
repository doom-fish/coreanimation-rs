use apple_metal::MetalDevice;
use coreanimation::MetalLayer;

#[test]
fn cametallayer_properties_round_trip() {
    let layer = MetalLayer::new().expect("layer");
    let device = MetalDevice::system_default().expect("metal device");
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
}
