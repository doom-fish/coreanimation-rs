# coreanimation-rs

Safe Rust bindings for Apple's [QuartzCore / Core Animation](https://developer.apple.com/documentation/quartzcore) framework on macOS.

> **Status:** v0.2.2 preserves the original v0.1.0 public API and now wraps the full non-exempt QuartzCore `CA*.h` audit surface on macOS, including `Action`, `AnimationDelegate`, `FrameRateRange`, constraint/layout-manager APIs, HDR/EDR metadata, remote layers, renderer color-space configuration, and advanced `Transform3D` helpers.
>
> See [`COVERAGE.md`](COVERAGE.md) for the logical-area matrix and [`COVERAGE_AUDIT.md`](COVERAGE_AUDIT.md) for the symbol-by-symbol audit (`0` gaps, `1` deprecated exemption).

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

- `Layer`, `ShapeLayer`, `TextLayer`, `GradientLayer`, `TransformLayer`, `ReplicatorLayer`, `ScrollLayer`, `TiledLayer`, `EmitterLayer`, `EmitterCell`, `MetalLayer`, `ToneMapMode`, `DynamicRange`, `Action`, `LayerDelegate`, `Constraint`, and `ConstraintLayoutManager`
- `Animation`, `AnimationDelegate`, `PropertyAnimation`, `BasicAnimation`, `KeyframeAnimation`, `SpringAnimation`, `AnimationGroup`, `Transition`, `FrameRateRange`, `TimingFunction`, `TimingFunctionName`, `ValueFunction`, `ValueFunctionName`, `MediaTimingFillMode`, and `current_media_time`
- `DisplayLink` for `CVDisplayLink`, `QuartzDisplayLink` for `CADisplayLink`, and `MetalDisplayLink` / `MetalDisplayLinkUpdate` for `CAMetalDisplayLink`
- `EDRMetadata`, `RemoteLayerClient`, `RemoteLayerServer`, `Renderer::new_with_color_space`, `CGColorSpace`, and `CGAffineTransform`
- `Color` and `Path` helpers for Core Animation content types that `apple-cf` does not wrap yet

## Examples

The crate now ships with 29 runnable examples: the original offscreen renderer smoke test, one focused example for each v0.2.0 / v0.2.1 logical area, and five v0.2.2 audit-closing examples.

Run any example with:

```bash
cargo run --example 25_ca_animation_delegate
```

Representative examples:

- `01_layer_render_smoke` — offscreen `CARenderer` + Metal texture validation
- `25_ca_animation_delegate` — `CAAnimationDelegate`, `CAFrameRateRange`, and `CACurrentMediaTime`
- `26_ca_layer_advanced` — `CAAction`, advanced `CALayer` enums, constraints, and delegate hooks
- `27_ca_edr_metadata` — `CAEDRMetadata` plus `CAMetalLayer` HDR / colorspace APIs
- `28_ca_remote_layer_renderer` — remote layers plus `CARenderer` color-space configuration
- `29_ca_transform3d` — advanced `CATransform3D` helpers and `CGAffineTransform` round-tripping

Each logical area has a matching integration test under `tests/`, with extra unit coverage for animation-delegate callbacks.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
