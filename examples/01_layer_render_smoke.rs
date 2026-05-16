use apple_cf::cg::CGRect;
use apple_metal::{pixel_format, storage_mode, texture_usage, MetalDevice, TextureDescriptor};
use coreanimation::{read_texture_bytes, Color, Layer, Renderer};

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const WIDTH_F64: f64 = 100.0;
const HEIGHT_F64: f64 = 100.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = MetalDevice::system_default().ok_or("no Metal device available")?;
    let queue = device
        .new_command_queue()
        .ok_or("failed to create command queue")?;
    let texture = device
        .new_texture(TextureDescriptor {
            pixel_format: pixel_format::BGRA8UNORM,
            width: WIDTH,
            height: HEIGHT,
            mipmapped: false,
            usage: texture_usage::RENDER_TARGET | texture_usage::SHADER_READ,
            storage_mode: storage_mode::SHARED,
        })
        .ok_or("failed to allocate render target texture")?;

    let layer = Layer::new().ok_or("failed to create layer")?;
    layer.set_frame(CGRect::new(0.0, 0.0, WIDTH_F64, HEIGHT_F64));
    layer.set_background_color(Some(&Color::red()));

    let renderer = Renderer::new(&texture, Some(&queue)).ok_or("failed to create renderer")?;
    renderer.set_layer(Some(&layer));
    renderer.set_bounds(CGRect::new(0.0, 0.0, WIDTH_F64, HEIGHT_F64));
    renderer.render_at_time(0.0);

    let marker = queue
        .new_command_buffer()
        .ok_or("failed to create sync command buffer")?;
    marker.commit();
    marker.wait_until_completed();

    let pixels = read_texture_bytes(&texture)?;
    assert!(
        pixels.iter().any(|&byte| byte != 0),
        "rendered texture was all zeroes"
    );
    println!("✅ coreanimation layer + render OK");
    Ok(())
}
