use std::ops::Deref;

use apple_cf::cg::{CGPoint, CGRect};

use crate::layer::{Layer, LayerLike};

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

    pub fn scroll_to_point(&self, point: CGPoint) {
        unsafe {
            crate::ffi::ca_scroll_layer_scroll_to_point(self.as_layer_ptr(), point.x, point.y)
        };
    }

    pub fn scroll_to_rect(&self, rect: CGRect) {
        unsafe {
            crate::ffi::ca_scroll_layer_scroll_to_rect(
                self.as_layer_ptr(),
                rect.x,
                rect.y,
                rect.width,
                rect.height,
            )
        };
    }
}

impl Deref for ScrollLayer {
    type Target = Layer;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl LayerLike for ScrollLayer {
    fn as_layer_ptr(&self) -> *mut core::ffi::c_void {
        self.inner.as_ptr()
    }
}
