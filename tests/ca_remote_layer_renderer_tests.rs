use apple_metal::{pixel_format, storage_mode, texture_usage, MetalDevice, TextureDescriptor};
use coreanimation::{
    read_texture_bytes, CGColorSpace, CGRect, Color, Layer, RemoteLayerClient, RemoteLayerServer,
    Renderer,
};

const WIDTH: usize = 64;
const HEIGHT: usize = 64;
const WIDTH_F64: f64 = 64.0;
const HEIGHT_F64: f64 = 64.0;

#[test]
fn caremotelayer_and_carenderer_color_space_round_trip() {
    let server = RemoteLayerServer::shared().expect("remote layer server");
    assert!(server.server_port() > 0);

    let client = RemoteLayerClient::new(server.server_port()).expect("remote layer client");
    assert!(client.client_id() > 0);

    let layer = Layer::new().expect("layer");
    layer.set_name("remote-root");
    layer.set_frame(CGRect::new(0.0, 0.0, WIDTH_F64, HEIGHT_F64));
    layer.set_background_color(Some(&Color::green()));

    client.set_layer(Some(&layer));
    assert_eq!(
        client.layer().and_then(|layer| layer.name()),
        Some(String::from("remote-root"))
    );
    assert!(RemoteLayerServer::layer_with_client_id(client.client_id()).is_some());

    let device = MetalDevice::system_default().expect("metal device");
    let queue = device.new_command_queue().expect("command queue");
    let texture = device
        .new_texture(TextureDescriptor {
            pixel_format: pixel_format::BGRA8UNORM,
            width: WIDTH,
            height: HEIGHT,
            mipmapped: false,
            usage: texture_usage::RENDER_TARGET | texture_usage::SHADER_READ,
            storage_mode: storage_mode::SHARED,
        })
        .expect("texture");
    let color_space = CGColorSpace::display_p3();
    let renderer = Renderer::new_with_color_space(&texture, Some(&queue), Some(&color_space))
        .expect("renderer");
    renderer.set_layer(Some(&layer));
    renderer.set_bounds(CGRect::new(0.0, 0.0, WIDTH_F64, HEIGHT_F64));
    renderer.render_at_time(0.0);

    let marker = queue.new_command_buffer().expect("marker");
    marker.commit();
    marker.wait_until_completed();

    let pixels = read_texture_bytes(&texture).expect("pixels");
    assert!(pixels.iter().any(|&byte| byte != 0));

    client.invalidate();
}
