use std::ops::Deref;

use apple_cf::cg::{CGPoint, CGRect};

use crate::error::CoreAnimationError;
use crate::layer::{bridge_result, ensure_finite, ensure_finite_rect, Layer, LayerLike};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ScrollMode {
    None = 0,
    Vertically = 1,
    Horizontally = 2,
    Both = 3,
}

impl ScrollMode {
    const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::Vertically,
            2 => Self::Horizontally,
            3 => Self::Both,
            _ => Self::None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScrollLayer {
    inner: Layer,
}

impl ScrollLayer {
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Layer::from_raw(crate::ffi::ca_scroll_layer_new()) }.map(|inner| Self { inner })
    }

    #[must_use]
    pub fn scroll_mode(&self) -> ScrollMode {
        ScrollMode::from_raw(unsafe {
            crate::ffi::ca_scroll_layer_get_scroll_mode(self.as_layer_ptr())
        })
    }

    pub fn set_scroll_mode(&self, value: ScrollMode) {
        unsafe { crate::ffi::ca_scroll_layer_set_scroll_mode(self.as_layer_ptr(), value as i32) };
    }

    #[must_use]
    pub fn visible_rect(&self) -> CGRect {
        let mut rect = CGRect::zero();
        let ok = unsafe {
            crate::ffi::ca_scroll_layer_get_visible_rect(
                self.as_layer_ptr(),
                (&mut rect as *mut CGRect).cast::<core::ffi::c_void>(),
            )
        };
        if ok {
            rect
        } else {
            CGRect::zero()
        }
    }

    pub fn scroll_to_point(&self, point: CGPoint) -> Result<(), CoreAnimationError> {
        ensure_finite("scroll point", &[point.x, point.y])?;
        let mut error = core::ptr::null_mut();
        let accepted = unsafe {
            crate::ffi::ca_scroll_layer_scroll_to_point(
                self.as_layer_ptr(),
                point.x,
                point.y,
                &raw mut error,
            )
        };
        bridge_result(accepted, error, "CAScrollLayer rejected the scroll point")
    }

    pub fn scroll_to_rect(&self, rect: CGRect) -> Result<(), CoreAnimationError> {
        ensure_finite_rect("scroll rect", rect)?;
        let mut error = core::ptr::null_mut();
        let accepted = unsafe {
            crate::ffi::ca_scroll_layer_scroll_to_rect(
                self.as_layer_ptr(),
                rect.origin.x,
                rect.origin.y,
                rect.size.width,
                rect.size.height,
                &raw mut error,
            )
        };
        bridge_result(accepted, error, "CAScrollLayer rejected the scroll rect")
    }
}

impl Deref for ScrollLayer {
    type Target = Layer;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl crate::private::Sealed for ScrollLayer {}

impl LayerLike for ScrollLayer {
    fn as_layer_ptr(&self) -> *mut core::ffi::c_void {
        self.inner.as_ptr()
    }
}
