use apple_metal::MetalDevice;
use coreanimation::MetalLayer;

#[test]
fn cametallayer_properties_round_trip() {
    let layer = MetalLayer::new().expect("layer");
    let device = MetalDevice::system_default().expect("metal device");
    layer.set_device(Some(&device));
    layer.set_framebuffer_only(false);
    layer
        .set_maximum_drawable_count(3)
        .expect("drawable count 3");
    layer.set_presents_with_transaction(true);
    layer.set_display_sync_enabled(false);
    layer.set_allows_next_drawable_timeout(false);

    assert!(!layer.framebuffer_only());
    assert_eq!(layer.maximum_drawable_count(), 3);
    assert!(layer.presents_with_transaction());
    assert!(!layer.display_sync_enabled());
    assert!(!layer.allows_next_drawable_timeout());
}

#[test]
fn maximum_drawable_count_accepts_only_two_or_three() {
    let layer = MetalLayer::new().expect("layer");

    for rejected in [0, 1, 4, usize::MAX] {
        let error = layer
            .set_maximum_drawable_count(rejected)
            .expect_err("out-of-range drawable count");
        assert!(error.to_string().contains("must be 2 or 3"), "{error}");
        assert_eq!(layer.maximum_drawable_count(), 3);
    }

    layer
        .set_maximum_drawable_count(2)
        .expect("drawable count 2");
    assert_eq!(layer.maximum_drawable_count(), 2);
    layer
        .set_maximum_drawable_count(3)
        .expect("drawable count 3");
    assert_eq!(layer.maximum_drawable_count(), 3);
}
