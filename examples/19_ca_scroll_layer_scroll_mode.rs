#![allow(clippy::float_cmp)]

use coreanimation::{CGPoint, CGRect, ScrollLayer, ScrollMode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let layer = ScrollLayer::new().ok_or("failed to create scroll layer")?;
    layer.set_bounds(CGRect::new(0.0, 0.0, 100.0, 80.0));
    layer.set_scroll_mode(ScrollMode::Both);
    layer.scroll_to_point(CGPoint::new(12.0, 18.0));
    assert_eq!(layer.scroll_mode(), ScrollMode::Both);
    assert_eq!(layer.visible_rect(), CGRect::new(12.0, 18.0, 100.0, 80.0));
    layer.scroll_to_rect(CGRect::new(2.0, 4.0, 50.0, 30.0));
    assert_eq!(layer.visible_rect(), CGRect::new(2.0, 4.0, 50.0, 30.0));
    println!("✅ CAScrollLayer helpers OK");
    Ok(())
}
