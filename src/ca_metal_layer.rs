use apple_cf::cg::CGColorSpace;

use crate::ca_edr_metadata::EDRMetadata;
use crate::layer::{LayerLike, MetalLayer};

impl MetalLayer {
    #[must_use]
    pub fn framebuffer_only(&self) -> bool {
        unsafe { crate::ffi::ca_metal_layer_get_framebuffer_only(self.as_layer_ptr()) }
    }

    pub fn set_framebuffer_only(&self, value: bool) {
        unsafe { crate::ffi::ca_metal_layer_set_framebuffer_only(self.as_layer_ptr(), value) };
    }

    #[must_use]
    pub fn maximum_drawable_count(&self) -> usize {
        unsafe { crate::ffi::ca_metal_layer_get_maximum_drawable_count(self.as_layer_ptr()) }
    }

    pub fn set_maximum_drawable_count(&self, value: usize) {
        unsafe {
            crate::ffi::ca_metal_layer_set_maximum_drawable_count(self.as_layer_ptr(), value)
        };
    }

    #[must_use]
    pub fn presents_with_transaction(&self) -> bool {
        unsafe { crate::ffi::ca_metal_layer_get_presents_with_transaction(self.as_layer_ptr()) }
    }

    pub fn set_presents_with_transaction(&self, value: bool) {
        unsafe {
            crate::ffi::ca_metal_layer_set_presents_with_transaction(self.as_layer_ptr(), value)
        };
    }

    #[must_use]
    pub fn display_sync_enabled(&self) -> bool {
        unsafe { crate::ffi::ca_metal_layer_get_display_sync_enabled(self.as_layer_ptr()) }
    }

    pub fn set_display_sync_enabled(&self, value: bool) {
        unsafe { crate::ffi::ca_metal_layer_set_display_sync_enabled(self.as_layer_ptr(), value) };
    }

    #[must_use]
    pub fn allows_next_drawable_timeout(&self) -> bool {
        unsafe { crate::ffi::ca_metal_layer_get_allows_next_drawable_timeout(self.as_layer_ptr()) }
    }

    pub fn set_allows_next_drawable_timeout(&self, value: bool) {
        unsafe {
            crate::ffi::ca_metal_layer_set_allows_next_drawable_timeout(self.as_layer_ptr(), value)
        };
    }

    #[must_use]
    pub fn colorspace(&self) -> Option<CGColorSpace> {
        let ptr = unsafe { crate::ffi::ca_metal_layer_get_colorspace(self.as_layer_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { CGColorSpace::from_raw(ptr) })
        }
    }

    pub fn set_colorspace(&self, value: Option<&CGColorSpace>) {
        unsafe {
            crate::ffi::ca_metal_layer_set_colorspace(
                self.as_layer_ptr(),
                value.map_or(core::ptr::null_mut(), CGColorSpace::as_ptr),
            )
        };
    }

    #[must_use]
    pub fn edr_metadata(&self) -> Option<EDRMetadata> {
        unsafe {
            EDRMetadata::from_raw(crate::ffi::ca_metal_layer_get_edr_metadata(
                self.as_layer_ptr(),
            ))
        }
    }

    pub fn set_edr_metadata(&self, value: Option<&EDRMetadata>) {
        unsafe {
            crate::ffi::ca_metal_layer_set_edr_metadata(
                self.as_layer_ptr(),
                value.map_or(core::ptr::null_mut(), EDRMetadata::as_ptr),
            )
        };
    }
}
