use coreanimation::TextLayer;

#[test]
fn catextlayer_flags_round_trip() {
    let layer = TextLayer::new().expect("text layer");
    layer.set_wrapped(true);
    layer.set_allows_font_subpixel_quantization(true);

    assert!(layer.is_wrapped());
    assert!(layer.allows_font_subpixel_quantization());
}
