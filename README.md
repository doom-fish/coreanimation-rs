# coreanimation-rs

Safe Rust bindings for Apple's [QuartzCore / Core Animation](https://developer.apple.com/documentation/quartzcore) framework on macOS.

> **Status:** v0.2.0 preserves the original v0.1.0 public API and adds additive coverage for `CALayer`, `CAAnimation`, `CAKeyframeAnimation`, `CABasicAnimation`, `CAAnimationGroup`, `CASpringAnimation`, `CATransition`, `CAMediaTiming`, `CATransaction`, `CADisplayLink`, `CAMetalLayer`, `CAGradientLayer`, `CATextLayer`, `CAShapeLayer`, `CATransformLayer`, `CAReplicatorLayer`, `CAEmitterLayer`, `CAScrollLayer`, and `CATiledLayer`.
>
> See [`COVERAGE.md`](COVERAGE.md) for the current coverage matrix and remaining QuartzCore gaps.

## Quick start

```rust,no_run
use apple_cf::cg::CGRect;
use apple_metal::{pixel_format, storage_mode, texture_usage, MetalDevice, TextureDescriptor};
use coreanimation::{read_texture_bytes, Color, Layer, Renderer, Transaction};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = MetalDevice::system_default().expect("no Metal device");
    let queue = device.new_command_queue().expect("no command queue");
    let texture = device
        .new_texture(TextureDescriptor {
            pixel_format: pixel_format::BGRA8UNORM,
            width: 100,
            height: 100,
            mipmapped: false,
            usage: texture_usage::RENDER_TARGET | texture_usage::SHADER_READ,
            storage_mode: storage_mode::SHARED,
        })
        .expect("texture");

    let layer = Layer::new().expect("layer");
    layer.set_frame(CGRect::new(0.0, 0.0, 100.0, 100.0));
    layer.set_background_color(Some(&Color::red()));

    let renderer = Renderer::new(&texture, Some(&queue)).expect("renderer");
    renderer.set_layer(Some(&layer));
    renderer.set_bounds(CGRect::new(0.0, 0.0, 100.0, 100.0));
    renderer.render_at_time(0.0);

    let marker = queue.new_command_buffer().expect("marker");
    marker.commit();
    marker.wait_until_completed();

    Transaction::flush();
    let pixels = read_texture_bytes(&texture)?;
    assert!(pixels.iter().any(|&byte| byte != 0));
    Ok(())
}
```

## Highlights

- `Layer`, `ShapeLayer`, `TextLayer`, `GradientLayer`, `TransformLayer`, `ReplicatorLayer`, `ScrollLayer`, `TiledLayer`, `EmitterLayer`, `EmitterCell`, and `MetalLayer`
- `Animation`, `BasicAnimation`, `KeyframeAnimation`, `SpringAnimation`, `AnimationGroup`, and `Transition`, plus `TimingFunctionName` and `MediaTimingFillMode`
- `Transaction` completion blocks, animation defaults, and a `TransactionLockGuard`
- `DisplayLink` for `CVDisplayLink` and `QuartzDisplayLink` for `CADisplayLink` (`macOS 14+`, headless-safe)
- `Renderer` for offscreen `CARenderer` rendering into `apple-metal` textures
- `Color` and `Path` helpers for Core Animation content types that `apple-cf` does not wrap yet

## Examples

The crate now ships with 20 runnable examples: the original offscreen renderer smoke test plus one focused example for each v0.2.0 logical area.

Run any example with:

```bash
cargo run --example 11_ca_display_link_smoke
```

Representative examples:

- `01_layer_render_smoke` — offscreen `CARenderer` + Metal texture validation
- `09_ca_media_timing_roundtrip` — `CAMediaTiming` fields and fill modes
- `11_ca_display_link_smoke` — `QuartzDisplayLink` / `CADisplayLink`
- `17_ca_replicator_layer_properties` — `CAReplicatorLayer` instance properties
- `20_ca_tiled_layer_properties` — `CATiledLayer` geometry controls

Each requested v0.2.0 logical area also has a matching integration test under `tests/`.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
