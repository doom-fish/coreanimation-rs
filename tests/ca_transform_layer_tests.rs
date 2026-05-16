#![allow(clippy::float_cmp)]

use coreanimation::TransformLayer;

#[test]
fn catransformlayer_constructs() {
    let layer = TransformLayer::new().expect("transform layer");
    layer.set_z_position(2.0);
    assert_eq!(layer.z_position(), 2.0);
}
