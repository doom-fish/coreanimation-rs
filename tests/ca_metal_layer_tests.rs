use std::collections::BTreeMap;

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

#[test]
fn extended_dynamic_range_flag_round_trips() {
    let layer = MetalLayer::new().expect("layer");
    assert!(!layer.wants_extended_dynamic_range_content());

    layer.set_wants_extended_dynamic_range_content(true);
    assert!(layer.wants_extended_dynamic_range_content());
    layer.set_wants_extended_dynamic_range_content(false);
    assert!(!layer.wants_extended_dynamic_range_content());
}

#[test]
fn preferred_device_is_an_owned_metal_device() {
    let layer = MetalLayer::new().expect("layer");
    let devices = apple_metal::copy_all_devices();
    let Some(preferred) = layer.preferred_device() else {
        assert!(
            devices.is_empty(),
            "no preferred device with {} GPUs",
            devices.len()
        );
        return;
    };

    assert!(devices
        .iter()
        .any(|device| device.registry_id() == preferred.registry_id()));
    if devices.len() == 1 {
        let default = MetalDevice::system_default().expect("default device");
        assert_eq!(preferred.registry_id(), default.registry_id());
    }
    assert!(!preferred.name().is_empty());
}

#[test]
fn developer_hud_properties_round_trip() {
    let layer = MetalLayer::new().expect("layer");
    if !MetalLayer::supports_developer_hud_properties() {
        assert!(layer.set_developer_hud_properties(None).is_err());
        assert_eq!(layer.developer_hud_properties(), None);
        return;
    }
    assert_eq!(layer.developer_hud_properties(), None);

    let properties = BTreeMap::from([
        ("mode".to_owned(), "disabled".to_owned()),
        ("doomfish.test".to_owned(), "1".to_owned()),
    ]);
    layer
        .set_developer_hud_properties(Some(&properties))
        .expect("set properties");
    assert_eq!(layer.developer_hud_properties(), Some(properties.clone()));

    let with_nul = BTreeMap::from([("mo\0de".to_owned(), "disabled".to_owned())]);
    assert!(layer.set_developer_hud_properties(Some(&with_nul)).is_err());
    assert_eq!(layer.developer_hud_properties(), Some(properties));

    layer
        .set_developer_hud_properties(Some(&BTreeMap::new()))
        .expect("empty properties");
    assert_eq!(layer.developer_hud_properties(), Some(BTreeMap::new()));
    layer
        .set_developer_hud_properties(None)
        .expect("clear properties");
    assert_eq!(layer.developer_hud_properties(), None);
}

#[test]
fn cloned_drawables_retain_their_own_reference() {
    let device = MetalDevice::system_default().expect("metal device");
    for _ in 0..20 {
        let layer = MetalLayer::new().expect("layer");
        layer.set_device(Some(&device));
        layer.set_pixel_format(apple_metal::pixel_format::BGRA8UNORM);
        layer.set_drawable_size(apple_cf::cg::CGSize::new(8.0, 8.0));
        let Some(drawable) = layer.next_drawable() else {
            eprintln!("skipping: the layer vended no drawable");
            return;
        };
        let copy = drawable.clone();
        drop(drawable);
        assert_eq!(copy.texture().expect("drawable texture").width(), 8);
        drop(copy);
    }
}
