use coreanimation::{Layer, MetalLayer, ToneMapMode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let layer = Layer::new().ok_or("failed to create layer")?;
    let metal_layer = MetalLayer::new().ok_or("failed to create metal layer")?;

    if Layer::supports_tone_map_mode() {
        layer.set_tone_map_mode(ToneMapMode::IfSupported);
        metal_layer.set_tone_map_mode(ToneMapMode::Never);

        assert_eq!(layer.tone_map_mode(), ToneMapMode::IfSupported);
        assert_eq!(metal_layer.tone_map_mode(), ToneMapMode::Never);
        println!("✅ CAToneMapMode OK");
    } else {
        println!("ℹ️ CAToneMapMode unavailable on this macOS version");
    }

    Ok(())
}
