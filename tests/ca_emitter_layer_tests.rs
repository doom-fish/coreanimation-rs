#![allow(clippy::float_cmp)]

use coreanimation::EmitterLayer;

#[test]
fn caemitterlayer_extended_properties_round_trip() {
    let layer = EmitterLayer::new().expect("emitter layer");
    layer.set_emitter_z_position(3.0);
    layer.set_emitter_depth(6.0);
    layer.set_preserves_depth(true);
    layer.set_spin(0.75);
    layer.set_seed(7);

    assert_eq!(layer.emitter_z_position(), 3.0);
    assert_eq!(layer.emitter_depth(), 6.0);
    assert!(layer.preserves_depth());
    assert_eq!(layer.spin(), 0.75);
    assert_eq!(layer.seed(), 7);
}
