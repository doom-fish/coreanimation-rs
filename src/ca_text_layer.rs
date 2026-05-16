use crate::layer::{LayerLike, TextLayer};

impl TextLayer {
    #[must_use]
    pub fn is_wrapped(&self) -> bool {
        unsafe { crate::ffi::ca_text_layer_get_wrapped(self.as_layer_ptr()) }
    }

    pub fn set_wrapped(&self, value: bool) {
        unsafe { crate::ffi::ca_text_layer_set_wrapped(self.as_layer_ptr(), value) };
    }

    #[must_use]
    pub fn allows_font_subpixel_quantization(&self) -> bool {
        unsafe {
            crate::ffi::ca_text_layer_get_allows_font_subpixel_quantization(self.as_layer_ptr())
        }
    }

    pub fn set_allows_font_subpixel_quantization(&self, value: bool) {
        unsafe {
            crate::ffi::ca_text_layer_set_allows_font_subpixel_quantization(
                self.as_layer_ptr(),
                value,
            )
        };
    }
}
