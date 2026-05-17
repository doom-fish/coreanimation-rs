use crate::private::handle_type;

handle_type!(EDRMetadata);

impl EDRMetadata {
    #[must_use]
    pub fn is_available() -> bool {
        unsafe { crate::ffi::ca_edr_metadata_is_available() }
    }

    #[must_use]
    pub fn hdr10_with_display_info(
        display_info: Option<&[u8]>,
        content_info: Option<&[u8]>,
        optical_output_scale: f32,
    ) -> Option<Self> {
        unsafe {
            Self::from_raw(crate::ffi::ca_edr_metadata_new_hdr10_with_display_info(
                display_info.map_or(core::ptr::null(), |value| value.as_ptr().cast()),
                display_info.map_or(0, <[u8]>::len),
                content_info.map_or(core::ptr::null(), |value| value.as_ptr().cast()),
                content_info.map_or(0, <[u8]>::len),
                optical_output_scale,
            ))
        }
    }

    #[must_use]
    pub fn hdr10(
        min_luminance: f32,
        max_luminance: f32,
        optical_output_scale: f32,
    ) -> Option<Self> {
        unsafe {
            Self::from_raw(crate::ffi::ca_edr_metadata_new_hdr10(
                min_luminance,
                max_luminance,
                optical_output_scale,
            ))
        }
    }

    #[must_use]
    pub fn hlg(ambient_viewing_environment: &[u8]) -> Option<Self> {
        unsafe {
            Self::from_raw(crate::ffi::ca_edr_metadata_new_hlg(
                ambient_viewing_environment.as_ptr().cast(),
                ambient_viewing_environment.len(),
            ))
        }
    }

    #[must_use]
    pub fn default_hlg() -> Option<Self> {
        unsafe { Self::from_raw(crate::ffi::ca_edr_metadata_get_hlg()) }
    }
}
