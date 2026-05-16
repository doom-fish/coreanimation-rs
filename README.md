# coreanimation-rs

Safe Rust bindings for Apple's [QuartzCore / Core Animation](https://developer.apple.com/documentation/quartzcore) framework on macOS.

> **Status:** v0.1.0 covers `CALayer` and key layer subclasses, particle emitters, `CAMetalLayer`, `CARenderer`, `CVDisplayLink`, transactions, and the core animation classes needed to drive layer-based rendering from Rust.

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

- `Layer`, `ShapeLayer`, `TextLayer`, `GradientLayer`, `EmitterLayer`, `EmitterCell`, and `MetalLayer`
- `Color` and `Path` helpers for common Core Animation content types that `apple-cf` does not wrap yet
- `Animation`, `BasicAnimation`, `KeyframeAnimation`, `SpringAnimation`, `AnimationGroup`, and `Transition`
- `Transaction` completion blocks and animation defaults
- `DisplayLink` built on `CVDisplayLink` for macOS vsync callbacks
- `Renderer` for offscreen `CARenderer` rendering into `apple-metal` textures

## Smoke example

Run the offscreen render smoke test with:

```bash
cargo run --example 01_layer_render_smoke
```

It creates a 100×100 red `CALayer`, renders it into a Metal texture with `CARenderer`, reads the pixel bytes back, and prints a success banner after verifying non-zero pixel data.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
