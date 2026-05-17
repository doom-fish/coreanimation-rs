use coreanimation::{CGColorSpace, EDRMetadata, Layer, MetalLayer};

#[test]
fn caedrmetadata_and_cametallayer_hdr_surface_round_trip() {
    let layer = MetalLayer::new().expect("metal layer");
    let color_space = CGColorSpace::display_p3();
    layer.set_colorspace(Some(&color_space));

    let round_tripped = layer.colorspace().expect("colorspace");
    assert_eq!(
        round_tripped.number_of_components(),
        color_space.number_of_components()
    );

    if Layer::supports_preferred_dynamic_range() {
        layer.set_preferred_dynamic_range(coreanimation::DynamicRange::ConstrainedHigh);
        assert_eq!(
            layer.preferred_dynamic_range(),
            coreanimation::DynamicRange::ConstrainedHigh
        );
    }

    let Some(hdr10) = EDRMetadata::hdr10(0.05, 1_000.0, 1.0) else {
        return;
    };
    layer.set_edr_metadata(Some(&hdr10));
    assert!(layer.edr_metadata().is_some());
    assert!(EDRMetadata::hdr10_with_display_info(None, None, 1.0).is_some());

    if EDRMetadata::is_available() {
        assert!(EDRMetadata::default_hlg().is_some());
    }
}
