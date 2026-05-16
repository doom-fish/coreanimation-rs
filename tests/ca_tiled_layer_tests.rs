#![allow(clippy::float_cmp)]

use coreanimation::{CGSize, TiledLayer};

#[test]
fn catiledlayer_properties_round_trip() {
    let layer = TiledLayer::new().expect("tiled layer");
    layer.set_levels_of_detail(4);
    layer.set_levels_of_detail_bias(2);
    layer.set_tile_size(CGSize::new(64.0, 128.0));

    assert_eq!(layer.levels_of_detail(), 4);
    assert_eq!(layer.levels_of_detail_bias(), 2);
    assert_eq!(layer.tile_size(), CGSize::new(64.0, 128.0));
}
