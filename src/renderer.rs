use apple_cf::cg::{CGColorSpace, CGRect};
use apple_metal::{pixel_format, CommandQueue, MetalTexture};

use crate::display_link::CVTimeStamp;
use crate::error::CoreAnimationError;
use crate::layer::LayerLike;
use crate::private::handle_type;

handle_type!(Renderer);

impl Renderer {
    #[must_use]
    /// Creates a `CARenderer` targeting a Metal texture.
    pub fn new(texture: &MetalTexture, queue: Option<&CommandQueue>) -> Option<Self> {
        Self::new_with_color_space(texture, queue, None)
    }

    #[must_use]
    /// Creates a `CARenderer` targeting a Metal texture with an optional color space.
    pub fn new_with_color_space(
        texture: &MetalTexture,
        queue: Option<&CommandQueue>,
        color_space: Option<&CGColorSpace>,
    ) -> Option<Self> {
        unsafe {
            Self::from_raw(crate::ffi::ca_renderer_new_with_options(
                texture.as_ptr(),
                queue.map_or(core::ptr::null_mut(), CommandQueue::as_ptr),
                color_space.map_or(core::ptr::null_mut(), CGColorSpace::as_ptr),
            ))
        }
    }

    /// Sets the root layer rendered by the renderer.
    pub fn set_layer<L: LayerLike>(&self, layer: Option<&L>) {
        unsafe {
            crate::ffi::ca_renderer_set_layer(
                self.as_ptr(),
                layer.map_or(core::ptr::null_mut(), LayerLike::as_layer_ptr),
            )
        };
    }

    #[must_use]
    /// Returns the renderer's bounds.
    pub fn bounds(&self) -> CGRect {
        let mut rect = CGRect::zero();
        let ok = unsafe {
            crate::ffi::ca_renderer_get_bounds(
                self.as_ptr(),
                (&mut rect as *mut CGRect).cast::<core::ffi::c_void>(),
            )
        };
        if ok {
            rect
        } else {
            CGRect::zero()
        }
    }

    /// Sets the renderer's bounds.
    pub fn set_bounds(&self, rect: CGRect) {
        unsafe {
            crate::ffi::ca_renderer_set_bounds(
                self.as_ptr(),
                rect.origin.x,
                rect.origin.y,
                rect.size.width,
                rect.size.height,
            )
        };
    }

    /// Begins a renderer frame at the supplied media time.
    pub fn begin_frame(&self, time: f64, time_stamp: Option<&CVTimeStamp>) {
        unsafe {
            crate::ffi::ca_renderer_begin_frame(
                self.as_ptr(),
                time,
                time_stamp.map_or(core::ptr::null_mut(), |time_stamp| {
                    (time_stamp as *const CVTimeStamp)
                        .cast_mut()
                        .cast::<core::ffi::c_void>()
                }),
            )
        };
    }

    pub fn add_update_rect(&self, rect: CGRect) {
        unsafe {
            crate::ffi::ca_renderer_add_update_rect(
                self.as_ptr(),
                rect.origin.x,
                rect.origin.y,
                rect.size.width,
                rect.size.height,
            )
        };
    }

    #[must_use]
    /// Returns the bounds updated by the current renderer frame.
    pub fn update_bounds(&self) -> CGRect {
        let mut rect = CGRect::zero();
        let ok = unsafe {
            crate::ffi::ca_renderer_update_bounds(
                self.as_ptr(),
                (&mut rect as *mut CGRect).cast::<core::ffi::c_void>(),
            )
        };
        if ok {
            rect
        } else {
            CGRect::zero()
        }
    }

    /// Renders the current layer tree into the destination texture.
    pub fn render(&self) {
        unsafe { crate::ffi::ca_renderer_render(self.as_ptr()) };
    }

    /// Ends the current renderer frame.
    pub fn end_frame(&self) {
        unsafe { crate::ffi::ca_renderer_end_frame(self.as_ptr()) };
    }

    #[must_use]
    /// Returns the renderer-reported time for the next frame.
    pub fn next_frame_time(&self) -> f64 {
        unsafe { crate::ffi::ca_renderer_next_frame_time(self.as_ptr()) }
    }

    /// Sets the renderer destination texture.
    pub fn set_destination(&self, texture: &MetalTexture) {
        unsafe { crate::ffi::ca_renderer_set_destination(self.as_ptr(), texture.as_ptr()) };
    }

    /// Renders the current layer tree at the supplied media time.
    pub fn render_at_time(&self, time: f64) {
        unsafe { crate::ffi::ca_renderer_render_at_time(self.as_ptr(), time) };
    }
}

/// Copies the texel bytes of a CPU-readable 2D Metal texture, `width * bytes_per_pixel` per row.
pub fn read_texture_bytes(texture: &MetalTexture) -> Result<Vec<u8>, CoreAnimationError> {
    let format = texture.pixel_format();
    let bytes_per_pixel = match format {
        pixel_format::DEPTH16UNORM | pixel_format::DEPTH32FLOAT | pixel_format::STENCIL8 => None,
        format => apple_metal::bytes_per_pixel(format),
    }
    .ok_or_else(|| {
        CoreAnimationError::new(format!(
            "pixel format {format} has no CPU byte layout (compressed, depth, stencil or unknown)"
        ))
    })?;
    let bytes_per_row = texture
        .width()
        .checked_mul(bytes_per_pixel)
        .ok_or_else(|| CoreAnimationError::new("texture bytes_per_row overflowed"))?;
    let byte_count = bytes_per_row
        .checked_mul(texture.height())
        .filter(|count| isize::try_from(*count).is_ok())
        .ok_or_else(|| CoreAnimationError::new("texture byte count overflowed"))?;
    let mut bytes = vec![0_u8; byte_count];

    let status = unsafe {
        crate::ffi::ca_texture_copy_bytes(
            texture.as_ptr(),
            bytes.as_mut_ptr().cast::<core::ffi::c_void>(),
            bytes.len(),
            bytes_per_row,
            bytes_per_pixel,
        )
    };

    match status {
        0 => Ok(bytes),
        2 => Err(CoreAnimationError::new(format!(
            "texture type {} is not a plain 2D texture",
            texture.texture_type()
        ))),
        3 => Err(CoreAnimationError::new(format!(
            "texture storage mode {} is not CPU-accessible; blit it to a shared texture first",
            texture.storage_mode()
        ))),
        4 => Err(CoreAnimationError::new(
            "framebuffer-only textures cannot be read; disable framebuffer_only on the layer",
        )),
        _ => Err(CoreAnimationError::new(
            "failed to copy bytes from Metal texture",
        )),
    }
}
