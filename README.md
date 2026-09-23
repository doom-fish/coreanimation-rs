# coreanimation-rs

Safe Rust bindings for Apple's [QuartzCore / Core Animation](https://developer.apple.com/documentation/quartzcore) framework on macOS.

The crate wraps layers, animations, transactions, the three display links
(`CVDisplayLink`, `CADisplayLink`, `CAMetalDisplayLink`), HDR/EDR metadata,
remote layers, and offscreen `CARenderer` rendering into Metal textures.

[`COVERAGE_AUDIT.md`](COVERAGE_AUDIT.md) names a Rust item for each of the 193
non-deprecated top-level `CA*.h` symbols (classes, protocols, typealiases,
constants and functions). That count does not cover class members, and many
members are not wrapped yet (for example `CALayer` hit testing, coordinate
conversion, `presentationLayer`, `shadowPath`, `filters` and rasterization, and
most `CAEmitterCell` ranges). [`COVERAGE.md`](COVERAGE.md) lists the known
gaps.

## Requirements

- macOS 11 or later and a Swift toolchain (Xcode or the Command Line Tools) to
  build the bridge. Rust 1.82 or later.
- Some APIs need a newer system and say so at run time instead of aborting:
  `MetalDisplayLink`, `QuartzDisplayLink` and perceptual spring animations need
  macOS 14 (constructors return `None` or an error), `MetalLayer`
  developer-HUD properties need macOS 13, tone mapping needs macOS 15 and
  `preferred_dynamic_range` needs macOS 26 (the `supports_*` functions report
  availability).

```toml
[dependencies]
coreanimation-rs = "0.6"
```

The library is imported as `coreanimation`.

## Threading

- Every wrapper is `!Send` and `!Sync`, so a Rust value stays on the thread that
  created it. Core Animation keeps its own data structures consistent when
  several threads touch layers, but changes made on a thread go into that
  thread's implicit transaction, which is only committed when the thread's run
  loop turns. On threads without a run loop (worker and test threads, offscreen
  `Renderer` use) wrap changes in `Transaction::begin`/`commit` or call
  `Transaction::flush`.
- Layers that back `AppKit` views must only be changed on the main thread, as
  `AppKit` requires. `QuartzDisplayLink` only works on the main thread and returns
  `None` elsewhere.
- Handlers run on the thread the framework chooses: `DisplayLink` output
  handlers on the `CoreVideo` display-link thread, `Transaction` completion
  handlers and `AnimationDelegate` callbacks on the main thread (only once the
  main run loop runs), and `MetalDisplayLink` updates on the run loop it was
  added to. That is why the first three must be `Send`. `LayerDelegate`
  callbacks run on the thread that displays or lays out the layer.
- Replacing or clearing a running `DisplayLink` output handler, and dropping its
  last handle, stops the link and waits for a handler that is still running.
  Don't do that on a thread the handler is waiting for.

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
            ..TextureDescriptor::new_2d(100, 100, pixel_format::BGRA8UNORM)
        })
        .expect("texture");

    let layer = Layer::new().expect("layer");
    layer.set_frame(CGRect::new(0.0, 0.0, 100.0, 100.0))?;
    layer.set_background_color(Some(&Color::red()));

    let renderer = Renderer::new(&texture, Some(&queue)).expect("renderer");
    renderer.set_layer(Some(&layer));
    renderer.set_bounds(CGRect::new(0.0, 0.0, 100.0, 100.0));
    renderer.render_at_time(0.0);

    let marker = queue.new_command_buffer().expect("marker");
    marker.commit()?;
    marker.wait_until_completed()?;

    Transaction::flush();
    let pixels = read_texture_bytes(&texture)?;
    assert!(pixels.iter().any(|&byte| byte != 0));
    Ok(())
}
```

## Highlights

- `Layer`, `ShapeLayer`, `TextLayer`, `GradientLayer`, `TransformLayer`, `ReplicatorLayer`, `ScrollLayer`, `TiledLayer`, `EmitterLayer`, `EmitterCell`, `MetalLayer`, `ToneMapMode`, `DynamicRange`, `Action`, `LayerDelegate`, `Constraint`, and `ConstraintLayoutManager`
- `Animation`, `AnimationDelegate`, `PropertyAnimation`, `BasicAnimation`, `KeyframeAnimation`, `SpringAnimation`, `AnimationGroup`, `Transition`, `FrameRateRange`, `TimingFunction`, `TimingFunctionName`, `ValueFunction`, `ValueFunctionName`, `MediaTimingFillMode`, and `current_media_time`
- `DisplayLink` for `CVDisplayLink` (closure output handler), `QuartzDisplayLink` for `CADisplayLink`, and `MetalDisplayLink` / `MetalDisplayLinkUpdate` for `CAMetalDisplayLink`
- `Transaction::set_completion_handler` for `CATransaction` completion blocks
- `read_texture_bytes`, which copies any uncompressed CPU-readable 2D Metal texture, sized from `apple_metal::bytes_per_pixel`
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
