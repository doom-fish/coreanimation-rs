use apple_cf::cg::CGRect;
use apple_metal::{CommandQueue, MetalTexture};

use crate::display_link::CVTimeStamp;
use crate::error::CoreAnimationError;
use crate::layer::LayerLike;
use crate::private::handle_type;

handle_type!(Renderer);

impl Renderer {
    #[must_use]
    pub fn new(texture: &MetalTexture, queue: Option<&CommandQueue>) -> Option<Self> {
        unsafe {
            Self::from_raw(crate::ffi::ca_renderer_new(
                texture.as_ptr(),
                queue.map_or(core::ptr::null_mut(), CommandQueue::as_ptr),
            ))
        }
    }

    pub fn set_layer<L: LayerLike>(&self, layer: Option<&L>) {
        unsafe {
            crate::ffi::ca_renderer_set_layer(
                self.as_ptr(),
                layer.map_or(core::ptr::null_mut(), LayerLike::as_layer_ptr),
            )
        };
    }

    #[must_use]
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

    pub fn set_bounds(&self, rect: CGRect) {
        unsafe {
            crate::ffi::ca_renderer_set_bounds(
                self.as_ptr(),
                rect.x,
                rect.y,
                rect.width,
                rect.height,
            )
        };
    }

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

    pub fn render(&self) {
        unsafe { crate::ffi::ca_renderer_render(self.as_ptr()) };
    }

    pub fn end_frame(&self) {
        unsafe { crate::ffi::ca_renderer_end_frame(self.as_ptr()) };
    }

    #[must_use]
    pub fn next_frame_time(&self) -> f64 {
        unsafe { crate::ffi::ca_renderer_next_frame_time(self.as_ptr()) }
    }

    pub fn set_destination(&self, texture: &MetalTexture) {
        unsafe { crate::ffi::ca_renderer_set_destination(self.as_ptr(), texture.as_ptr()) };
    }

    pub fn render_at_time(&self, time: f64) {
        unsafe { crate::ffi::ca_renderer_render_at_time(self.as_ptr(), time) };
    }
}

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
