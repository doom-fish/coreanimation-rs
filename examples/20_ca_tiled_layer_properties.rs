#![allow(clippy::float_cmp)]

use coreanimation::{CGSize, TiledLayer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let layer = TiledLayer::new().ok_or("failed to create tiled layer")?;
    layer.set_levels_of_detail(4);
    layer.set_levels_of_detail_bias(2);
    layer.set_tile_size(CGSize::new(128.0, 256.0));

    assert_eq!(layer.levels_of_detail(), 4);
    assert_eq!(layer.levels_of_detail_bias(), 2);
    assert_eq!(layer.tile_size(), CGSize::new(128.0, 256.0));
    println!("✅ CATiledLayer properties OK");
    Ok(())
}
