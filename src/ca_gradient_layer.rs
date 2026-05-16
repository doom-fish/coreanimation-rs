use crate::layer::{GradientLayer, LayerLike};

impl GradientLayer {
    #[must_use]
    pub fn color_components_at(&self, index: usize) -> Option<(f64, f64, f64, f64)> {
        let mut components = [0.0_f64; 4];
        let ok = unsafe {
            crate::ffi::ca_gradient_layer_get_color_components_at(
                self.as_layer_ptr(),
                index,
                components.as_mut_ptr().cast::<core::ffi::c_void>(),
            )
        };
        ok.then_some((components[0], components[1], components[2], components[3]))
    }
}
