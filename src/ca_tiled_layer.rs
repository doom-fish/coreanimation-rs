use std::ops::Deref;

use apple_cf::cg::CGSize;

use crate::layer::{Layer, LayerLike};

#[derive(Debug, Clone)]
pub struct TiledLayer {
    inner: Layer,
}

impl TiledLayer {
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Layer::from_raw(crate::ffi::ca_tiled_layer_new()) }.map(|inner| Self { inner })
    }

    #[must_use]
    pub fn levels_of_detail(&self) -> usize {
        unsafe { crate::ffi::ca_tiled_layer_get_levels_of_detail(self.as_layer_ptr()) }
    }

    pub fn set_levels_of_detail(&self, value: usize) {
        unsafe { crate::ffi::ca_tiled_layer_set_levels_of_detail(self.as_layer_ptr(), value) };
    }

    #[must_use]
    pub fn levels_of_detail_bias(&self) -> usize {
        unsafe { crate::ffi::ca_tiled_layer_get_levels_of_detail_bias(self.as_layer_ptr()) }
    }

    pub fn set_levels_of_detail_bias(&self, value: usize) {
        unsafe { crate::ffi::ca_tiled_layer_set_levels_of_detail_bias(self.as_layer_ptr(), value) };
    }

    #[must_use]
    pub fn tile_size(&self) -> CGSize {
        let mut size = CGSize::zero();
        let ok = unsafe {
            crate::ffi::ca_tiled_layer_get_tile_size(
                self.as_layer_ptr(),
                (&mut size as *mut CGSize).cast::<core::ffi::c_void>(),
            )
        };
        if ok {
            size
        } else {
            CGSize::zero()
        }
    }

    pub fn set_tile_size(&self, size: CGSize) {
        unsafe {
            crate::ffi::ca_tiled_layer_set_tile_size(self.as_layer_ptr(), size.width, size.height)
        };
    }
}

impl Deref for TiledLayer {
    type Target = Layer;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl LayerLike for TiledLayer {
    fn as_layer_ptr(&self) -> *mut core::ffi::c_void {
        self.inner.as_ptr()
    }
}
