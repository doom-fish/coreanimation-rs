use std::error::Error;

use coreanimation::{CGColorSpace, EDRMetadata, Layer, MetalLayer};

fn main() -> Result<(), Box<dyn Error>> {
    let layer = MetalLayer::new().ok_or("failed to create metal layer")?;
    let color_space = CGColorSpace::display_p3();
    layer.set_colorspace(Some(&color_space));
    let round_tripped = layer
        .colorspace()
        .ok_or("failed to round-trip colorspace")?;
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

    if let Some(hdr10) = EDRMetadata::hdr10(0.05, 1_000.0, 1.0) {
        layer.set_edr_metadata(Some(&hdr10));
        assert!(layer.edr_metadata().is_some());
        assert!(EDRMetadata::hdr10_with_display_info(None, None, 1.0).is_some());
        if EDRMetadata::is_available() {
            assert!(EDRMetadata::default_hlg().is_some());
        }
        println!("✅ CAEDRMetadata / CAMetalLayer HDR surface OK");
    } else {
        println!("ℹ️ CAEDRMetadata unavailable on this macOS version");
    }

    Ok(())
}
