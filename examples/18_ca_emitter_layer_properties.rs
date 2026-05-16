#![allow(clippy::float_cmp)]

use coreanimation::EmitterLayer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let layer = EmitterLayer::new().ok_or("failed to create emitter layer")?;
    layer.set_emitter_z_position(3.0);
    layer.set_emitter_depth(5.0);
    layer.set_preserves_depth(true);
    layer.set_spin(0.5);
    layer.set_seed(42);

    assert_eq!(layer.emitter_z_position(), 3.0);
    assert_eq!(layer.emitter_depth(), 5.0);
    assert!(layer.preserves_depth());
    assert_eq!(layer.spin(), 0.5);
    assert_eq!(layer.seed(), 42);
    println!("✅ CAEmitterLayer properties OK");
    Ok(())
}
