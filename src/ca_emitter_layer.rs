use crate::emitter::EmitterLayer;
use crate::layer::LayerLike;

impl EmitterLayer {
    #[must_use]
    /// Returns the emitter layer's emitter z position.
    pub fn emitter_z_position(&self) -> f64 {
        unsafe { crate::ffi::ca_emitter_layer_get_emitter_z_position(self.as_layer_ptr()) }
    }

    /// Sets the emitter layer's emitter z position.
    pub fn set_emitter_z_position(&self, value: f64) {
        unsafe { crate::ffi::ca_emitter_layer_set_emitter_z_position(self.as_layer_ptr(), value) };
    }

    #[must_use]
    /// Returns the emitter layer's emitter depth.
    pub fn emitter_depth(&self) -> f64 {
        unsafe { crate::ffi::ca_emitter_layer_get_emitter_depth(self.as_layer_ptr()) }
    }

    /// Sets the emitter layer's emitter depth.
    pub fn set_emitter_depth(&self, value: f64) {
        unsafe { crate::ffi::ca_emitter_layer_set_emitter_depth(self.as_layer_ptr(), value) };
    }

    #[must_use]
    /// Returns whether the emitter layer preserves depth.
    pub fn preserves_depth(&self) -> bool {
        unsafe { crate::ffi::ca_emitter_layer_get_preserves_depth(self.as_layer_ptr()) }
    }

    /// Sets the emitter layer's preserves depth.
    pub fn set_preserves_depth(&self, value: bool) {
        unsafe { crate::ffi::ca_emitter_layer_set_preserves_depth(self.as_layer_ptr(), value) };
    }

    #[must_use]
    /// Returns the emitter layer's spin.
    pub fn spin(&self) -> f32 {
        unsafe { crate::ffi::ca_emitter_layer_get_spin(self.as_layer_ptr()) }
    }

    /// Sets the emitter layer's spin.
    pub fn set_spin(&self, value: f32) {
        unsafe { crate::ffi::ca_emitter_layer_set_spin(self.as_layer_ptr(), value) };
    }

    #[must_use]
    /// Returns the emitter layer's seed.
    pub fn seed(&self) -> u32 {
        unsafe { crate::ffi::ca_emitter_layer_get_seed(self.as_layer_ptr()) }
    }

    /// Sets the emitter layer's seed.
    pub fn set_seed(&self, value: u32) {
        unsafe { crate::ffi::ca_emitter_layer_set_seed(self.as_layer_ptr(), value) };
    }
}
