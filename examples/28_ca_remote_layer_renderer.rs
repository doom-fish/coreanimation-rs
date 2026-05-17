use std::error::Error;

use apple_metal::{pixel_format, storage_mode, texture_usage, MetalDevice, TextureDescriptor};
use coreanimation::{
    read_texture_bytes, CGColorSpace, CGRect, Color, Layer, RemoteLayerClient, RemoteLayerServer,
    Renderer,
};

const WIDTH: usize = 64;
const HEIGHT: usize = 64;
const WIDTH_F64: f64 = 64.0;
const HEIGHT_F64: f64 = 64.0;

fn main() -> Result<(), Box<dyn Error>> {
    let server = RemoteLayerServer::shared().ok_or("failed to create remote layer server")?;
    let client = RemoteLayerClient::new(server.server_port())
        .ok_or("failed to create remote layer client")?;

    let layer = Layer::new().ok_or("failed to create layer")?;
    layer.set_name("remote-root");
    layer.set_frame(CGRect::new(0.0, 0.0, WIDTH_F64, HEIGHT_F64));
    layer.set_background_color(Some(&Color::green()));
    client.set_layer(Some(&layer));
    assert!(RemoteLayerServer::layer_with_client_id(client.client_id()).is_some());

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
        .ok_or("failed to allocate texture")?;
    let color_space = CGColorSpace::display_p3();
    let renderer = Renderer::new_with_color_space(&texture, Some(&queue), Some(&color_space))
        .ok_or("failed to create renderer")?;
    renderer.set_layer(Some(&layer));
    renderer.set_bounds(CGRect::new(0.0, 0.0, WIDTH_F64, HEIGHT_F64));
    renderer.render_at_time(0.0);

    let marker = queue
        .new_command_buffer()
        .ok_or("failed to create marker command buffer")?;
    marker.commit();
    marker.wait_until_completed();

    let pixels = read_texture_bytes(&texture)?;
    assert!(pixels.iter().any(|&byte| byte != 0));
    client.invalidate();

    println!("✅ CARemoteLayerClient / CARemoteLayerServer / CARenderer color-space OK");
    Ok(())
}
