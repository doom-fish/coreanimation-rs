use apple_cf::cg::CGRect;
use apple_metal::{pixel_format, texture_usage, MetalDevice, TextureDescriptor};
use coreanimation::{Color, Layer, Renderer};

#[test]
fn added_update_rects_become_the_update_bounds() {
    let Some(device) = MetalDevice::system_default() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let queue = device.new_command_queue().expect("command queue");
    let mut descriptor = TextureDescriptor::new_2d(64, 64, pixel_format::BGRA8UNORM);
    descriptor.usage = texture_usage::RENDER_TARGET | texture_usage::SHADER_READ;
    let texture = device.new_texture(descriptor).expect("texture");
    let layer = Layer::new().expect("layer");
    layer
        .set_frame(CGRect::new(0.0, 0.0, 64.0, 64.0))
        .expect("frame");
    layer.set_background_color(Some(&Color::red()));
    let renderer = Renderer::new(&texture, Some(&queue)).expect("renderer");
    renderer.set_layer(Some(&layer));
    renderer.set_bounds(CGRect::new(0.0, 0.0, 64.0, 64.0));
    renderer.render_at_time(0.0);

    renderer.begin_frame(1.0, None);
    assert!(renderer.update_bounds().size.width <= 0.0);
    renderer.add_update_rect(CGRect::new(8.0, 8.0, 16.0, 16.0));
    let bounds = renderer.update_bounds();
    renderer.render();
    renderer.end_frame();

    assert_eq!(bounds, CGRect::new(8.0, 8.0, 16.0, 16.0));
}
