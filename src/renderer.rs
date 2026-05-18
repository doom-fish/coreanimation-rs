use apple_cf::cg::{CGColorSpace, CGRect};
use apple_metal::{CommandQueue, MetalTexture};

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

/// Copies 8-bit RGBA or BGRA texel bytes from a Metal texture.
pub fn read_texture_bytes(texture: &MetalTexture) -> Result<Vec<u8>, CoreAnimationError> {
    let width = texture.width();
    let height = texture.height();
    let bytes_per_row = width
        .checked_mul(4)
        .ok_or_else(|| CoreAnimationError::new("texture bytes_per_row overflowed"))?;
    let mut bytes = vec![
        0_u8;
        bytes_per_row.checked_mul(height).ok_or_else(
            || CoreAnimationError::new("texture byte count overflowed")
        )?
    ];

    let ok = unsafe {
        crate::ffi::ca_texture_copy_bytes(
            texture.as_ptr(),
            bytes.as_mut_ptr().cast::<core::ffi::c_void>(),
            bytes_per_row,
        )
    };

    if ok {
        Ok(bytes)
    } else {
        Err(CoreAnimationError::new(
            "failed to copy bytes from Metal texture; use an 8-bit RGBA/BGRA texture",
        ))
    }
}
