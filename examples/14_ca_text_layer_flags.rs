use coreanimation::TextLayer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let layer = TextLayer::new().ok_or("failed to create text layer")?;
    layer.set_wrapped(true);
    layer.set_allows_font_subpixel_quantization(true);

    assert!(layer.is_wrapped());
    assert!(layer.allows_font_subpixel_quantization());
    println!("✅ CATextLayer flags OK");
    Ok(())
}
