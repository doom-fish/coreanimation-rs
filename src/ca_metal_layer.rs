use std::collections::BTreeMap;
use std::ffi::{CStr, CString};

use apple_cf::cg::CGColorSpace;
use apple_metal::MetalDevice;

use crate::ca_edr_metadata::EDRMetadata;
use crate::error::CoreAnimationError;
use crate::layer::{LayerLike, MetalLayer};

impl MetalLayer {
    #[must_use]
    /// Returns whether the Metal layer is framebuffer only.
    pub fn framebuffer_only(&self) -> bool {
        unsafe { crate::ffi::ca_metal_layer_get_framebuffer_only(self.as_layer_ptr()) }
    }

    /// Sets the Metal layer's framebuffer only.
    pub fn set_framebuffer_only(&self, value: bool) {
        unsafe { crate::ffi::ca_metal_layer_set_framebuffer_only(self.as_layer_ptr(), value) };
    }

    #[must_use]
    /// Returns the Metal layer's maximum drawable count.
    pub fn maximum_drawable_count(&self) -> usize {
        unsafe { crate::ffi::ca_metal_layer_get_maximum_drawable_count(self.as_layer_ptr()) }
    }

    /// Sets the Metal layer's maximum drawable count.
    pub fn set_maximum_drawable_count(&self, value: usize) -> Result<(), CoreAnimationError> {
        if !(2..=3).contains(&value) {
            return Err(CoreAnimationError::new(format!(
                "maximum drawable count must be 2 or 3, got {value}"
            )));
        }
        let accepted = unsafe {
            crate::ffi::ca_metal_layer_set_maximum_drawable_count(self.as_layer_ptr(), value)
        };
        if accepted {
            Ok(())
        } else {
            Err(CoreAnimationError::new(
                "CAMetalLayer rejected the maximum drawable count",
            ))
        }
    }

    #[must_use]
    /// Returns whether the Metal layer presents with transaction.
    pub fn presents_with_transaction(&self) -> bool {
        unsafe { crate::ffi::ca_metal_layer_get_presents_with_transaction(self.as_layer_ptr()) }
    }

    /// Sets the Metal layer's presents with transaction.
    pub fn set_presents_with_transaction(&self, value: bool) {
        unsafe {
            crate::ffi::ca_metal_layer_set_presents_with_transaction(self.as_layer_ptr(), value)
        };
    }

    #[must_use]
    /// Returns whether the Metal layer is display sync enabled.
    pub fn display_sync_enabled(&self) -> bool {
        unsafe { crate::ffi::ca_metal_layer_get_display_sync_enabled(self.as_layer_ptr()) }
    }

    /// Sets the Metal layer's display sync enabled.
    pub fn set_display_sync_enabled(&self, value: bool) {
        unsafe { crate::ffi::ca_metal_layer_set_display_sync_enabled(self.as_layer_ptr(), value) };
    }

    #[must_use]
    /// Returns whether the Metal layer allows next drawable timeout.
    pub fn allows_next_drawable_timeout(&self) -> bool {
        unsafe { crate::ffi::ca_metal_layer_get_allows_next_drawable_timeout(self.as_layer_ptr()) }
    }

    /// Sets the Metal layer's allows next drawable timeout.
    pub fn set_allows_next_drawable_timeout(&self, value: bool) {
        unsafe {
            crate::ffi::ca_metal_layer_set_allows_next_drawable_timeout(self.as_layer_ptr(), value)
        };
    }

    #[must_use]
    /// Returns the Metal layer's colorspace.
    pub fn colorspace(&self) -> Option<CGColorSpace> {
        let ptr = unsafe { crate::ffi::ca_metal_layer_get_colorspace(self.as_layer_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { CGColorSpace::from_raw(ptr) })
        }
    }

    /// Sets the Metal layer's colorspace.
    pub fn set_colorspace(&self, value: Option<&CGColorSpace>) {
        unsafe {
            crate::ffi::ca_metal_layer_set_colorspace(
                self.as_layer_ptr(),
                value.map_or(core::ptr::null_mut(), CGColorSpace::as_ptr),
            )
        };
    }

    #[must_use]
    /// Returns the Metal layer's EDR metadata.
    pub fn edr_metadata(&self) -> Option<EDRMetadata> {
        unsafe {
            EDRMetadata::from_raw(crate::ffi::ca_metal_layer_get_edr_metadata(
                self.as_layer_ptr(),
            ))
        }
    }

    /// Sets the Metal layer's EDR metadata.
    pub fn set_edr_metadata(&self, value: Option<&EDRMetadata>) {
        unsafe {
            crate::ffi::ca_metal_layer_set_edr_metadata(
                self.as_layer_ptr(),
                value.map_or(core::ptr::null_mut(), EDRMetadata::as_ptr),
            )
        };
    }

    #[must_use]
    pub fn wants_extended_dynamic_range_content(&self) -> bool {
        unsafe {
            crate::ffi::ca_metal_layer_get_wants_extended_dynamic_range_content(self.as_layer_ptr())
        }
    }

    pub fn set_wants_extended_dynamic_range_content(&self, value: bool) {
        unsafe {
            crate::ffi::ca_metal_layer_set_wants_extended_dynamic_range_content(
                self.as_layer_ptr(),
                value,
            )
        };
    }

    #[must_use]
    pub fn preferred_device(&self) -> Option<MetalDevice> {
        let registry_id = unsafe {
            crate::ffi::ca_metal_layer_get_preferred_device_registry_id(self.as_layer_ptr())
        };
        if registry_id == 0 {
            return None;
        }
        apple_metal::copy_all_devices()
            .into_iter()
            .find(|device| device.registry_id() == registry_id)
    }

    #[must_use]
    pub fn supports_developer_hud_properties() -> bool {
        unsafe { crate::ffi::ca_metal_layer_supports_developer_hud_properties() }
    }

    #[must_use]
    pub fn developer_hud_properties(&self) -> Option<BTreeMap<String, String>> {
        let mut count = 0_usize;
        let entries = unsafe {
            crate::ffi::ca_metal_layer_copy_developer_hud_properties(
                self.as_layer_ptr(),
                &raw mut count,
            )
        };
        if entries.is_null() {
            return None;
        }
        let strings: Vec<Option<String>> = (0..count.saturating_mul(2))
            .map(|index| unsafe {
                let string = *entries.add(index);
                if string.is_null() {
                    return None;
                }
                let owned = CStr::from_ptr(string).to_string_lossy().into_owned();
                libc::free(string.cast());
                Some(owned)
            })
            .collect();
        unsafe { libc::free(entries.cast()) };
        Some(
            strings
                .chunks_exact(2)
                .filter_map(|pair| Some((pair[0].clone()?, pair[1].clone()?)))
                .collect(),
        )
    }

    pub fn set_developer_hud_properties(
        &self,
        properties: Option<&BTreeMap<String, String>>,
    ) -> Result<(), CoreAnimationError> {
        if !Self::supports_developer_hud_properties() {
            return Err(CoreAnimationError::new(
                "CAMetalLayer.developerHUDProperties requires macOS 13.0 or later",
            ));
        }
        let entries = properties
            .map(|properties| {
                properties
                    .iter()
                    .map(|(key, value)| {
                        Ok((CString::new(key.as_str())?, CString::new(value.as_str())?))
                    })
                    .collect::<Result<Vec<_>, std::ffi::NulError>>()
            })
            .transpose()
            .map_err(|_| {
                CoreAnimationError::new("developer HUD property keys and values cannot contain NUL")
            })?;
        let keys: Vec<*const libc::c_char> = entries
            .iter()
            .flatten()
            .map(|(key, _)| key.as_ptr())
            .collect();
        let values: Vec<*const libc::c_char> = entries
            .iter()
            .flatten()
            .map(|(_, value)| value.as_ptr())
            .collect();
        let accepted = unsafe {
            crate::ffi::ca_metal_layer_set_developer_hud_properties(
                self.as_layer_ptr(),
                keys.as_ptr(),
                values.as_ptr(),
                keys.len(),
                entries.is_some(),
            )
        };
        if accepted {
            Ok(())
        } else {
            Err(CoreAnimationError::new(
                "CAMetalLayer rejected the developer HUD properties",
            ))
        }
    }
}
