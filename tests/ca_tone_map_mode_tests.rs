use coreanimation::{Layer, MetalLayer, ToneMapMode};

#[test]
fn catonemapmode_round_trip() {
    let layer = Layer::new().expect("layer");
    let metal_layer = MetalLayer::new().expect("metal layer");

    if !Layer::supports_tone_map_mode() {
        assert_eq!(layer.tone_map_mode(), ToneMapMode::Automatic);
        assert_eq!(metal_layer.tone_map_mode(), ToneMapMode::Automatic);
        return;
    }

    layer.set_tone_map_mode(ToneMapMode::IfSupported);
    metal_layer.set_tone_map_mode(ToneMapMode::Never);

    assert_eq!(layer.tone_map_mode(), ToneMapMode::IfSupported);
    assert_eq!(metal_layer.tone_map_mode(), ToneMapMode::Never);
}
