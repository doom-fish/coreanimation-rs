use apple_cf::cg::CGSize;
use apple_metal::{
    pixel_format, storage_mode, texture_type, texture_usage, MetalDevice, MetalTexture,
    TextureDescriptor,
};
use coreanimation::{read_texture_bytes, MetalLayer};

const WIDTH: usize = 5;
const HEIGHT: usize = 3;

fn texture(device: &MetalDevice, descriptor: TextureDescriptor) -> Option<MetalTexture> {
    let texture = device.new_texture(descriptor);
    if texture.is_none() {
        eprintln!("skipping: device cannot create {descriptor:?}");
    }
    texture
}

fn shared_2d(device: &MetalDevice, format: usize) -> Option<MetalTexture> {
    let mut descriptor = TextureDescriptor::new_2d(WIDTH, HEIGHT, format);
    descriptor.usage = texture_usage::SHADER_READ | texture_usage::RENDER_TARGET;
    texture(device, descriptor)
}

fn round_trip(format: usize, bytes_per_pixel: usize, channel: impl Fn(usize) -> u16) {
    let Some(device) = MetalDevice::system_default() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let Some(texture) = shared_2d(&device, format) else {
        return;
    };
    let pattern: Vec<u8> = (0..WIDTH * HEIGHT * bytes_per_pixel / 2)
        .flat_map(|index| channel(index).to_le_bytes())
        .collect();
    unsafe {
        texture.replace_region_2d(
            &pattern,
            WIDTH * bytes_per_pixel,
            (0, 0),
            (WIDTH, HEIGHT),
            0,
        )
    }
    .expect("upload");

    let bytes = read_texture_bytes(&texture).expect("read back");

    assert_eq!(bytes.len(), WIDTH * HEIGHT * bytes_per_pixel);
    assert_eq!(bytes, pattern);
}

#[test]
fn rgba16_float_rows_use_eight_bytes_per_pixel() {
    round_trip(pixel_format::RGBA16FLOAT, 8, |index| {
        0x3C00 + u16::try_from(index % 1024).expect("half-float mantissa")
    });
}

#[test]
fn bgra10_xr_rows_use_eight_bytes_per_pixel() {
    round_trip(pixel_format::BGRA10_XR, 8, |index| {
        u16::try_from((index * 29 % 1024) << 6).expect("10-bit channel")
    });
}

#[test]
fn bgra8_rows_use_four_bytes_per_pixel() {
    round_trip(pixel_format::BGRA8UNORM, 4, |index| {
        u16::try_from(index * 257 % 65_536).expect("two 8-bit channels")
    });
}

#[test]
fn formats_without_a_cpu_layout_are_rejected() {
    let Some(device) = MetalDevice::system_default() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let mut depth = TextureDescriptor::new_2d(4, 4, pixel_format::DEPTH32FLOAT);
    depth.usage = texture_usage::RENDER_TARGET;
    depth.storage_mode = storage_mode::PRIVATE;
    let mut compressed = TextureDescriptor::new_2d(4, 4, pixel_format::BC1_RGBA);
    compressed.usage = texture_usage::SHADER_READ;

    let mut checked = 0;
    for descriptor in [depth, compressed] {
        if let Some(texture) = texture(&device, descriptor) {
            let error = read_texture_bytes(&texture).expect_err("no CPU layout");
            assert!(
                error.to_string().contains("no CPU byte layout"),
                "unexpected error: {error}"
            );
            checked += 1;
        }
    }
    assert!(
        checked > 0,
        "no texture without a CPU layout could be created"
    );
}

#[test]
fn private_storage_is_rejected() {
    let Some(device) = MetalDevice::system_default() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let mut descriptor = TextureDescriptor::new_2d(WIDTH, HEIGHT, pixel_format::RGBA16FLOAT);
    descriptor.usage = texture_usage::RENDER_TARGET;
    descriptor.storage_mode = storage_mode::PRIVATE;
    let Some(texture) = texture(&device, descriptor) else {
        return;
    };

    let error = read_texture_bytes(&texture).expect_err("private storage");

    assert!(
        error.to_string().contains("not CPU-accessible"),
        "unexpected error: {error}"
    );
}

#[test]
fn array_textures_are_rejected() {
    let Some(device) = MetalDevice::system_default() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let descriptor = TextureDescriptor::new_2d(WIDTH, HEIGHT, pixel_format::BGRA8UNORM)
        .with_texture_type(texture_type::TYPE_2D_ARRAY)
        .with_array_length(2);
    let Some(texture) = texture(&device, descriptor) else {
        return;
    };

    let error = read_texture_bytes(&texture).expect_err("array texture");

    assert!(
        error.to_string().contains("not a plain 2D texture"),
        "unexpected error: {error}"
    );
}

#[test]
fn framebuffer_only_drawables_are_rejected() {
    let Some(device) = MetalDevice::system_default() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let layer = MetalLayer::new().expect("metal layer");
    layer.set_device(Some(&device));
    layer
        .set_pixel_format(pixel_format::BGRA8UNORM)
        .expect("pixel format");
    layer.set_drawable_size(CGSize::new(8.0, 8.0));
    assert!(layer.framebuffer_only());
    let Some(drawable) = layer.next_drawable() else {
        eprintln!("skipping: the layer vended no drawable");
        return;
    };
    let texture = drawable.texture().expect("drawable texture");

    let error = read_texture_bytes(&texture).expect_err("framebuffer-only drawable");
    assert!(
        error.to_string().contains("framebuffer-only"),
        "unexpected error: {error}"
    );

    layer.set_framebuffer_only(false);
    let Some(readable) = layer.next_drawable() else {
        eprintln!("skipping: the layer vended no second drawable");
        return;
    };
    let bytes = read_texture_bytes(&readable.texture().expect("drawable texture"))
        .expect("readable drawable");
    assert_eq!(bytes.len(), 8 * 8 * 4);
}
