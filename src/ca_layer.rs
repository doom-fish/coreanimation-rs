use crate::layer::Layer;
use crate::transform::Transform3D;

impl Layer {
    #[must_use]
    pub fn z_position(&self) -> f64 {
        unsafe { crate::ffi::ca_layer_get_z_position(self.as_ptr()) }
    }

    pub fn set_z_position(&self, value: f64) {
        unsafe { crate::ffi::ca_layer_set_z_position(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn anchor_point_z(&self) -> f64 {
        unsafe { crate::ffi::ca_layer_get_anchor_point_z(self.as_ptr()) }
    }

    pub fn set_anchor_point_z(&self, value: f64) {
        unsafe { crate::ffi::ca_layer_set_anchor_point_z(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn sublayer_transform(&self) -> Transform3D {
        let mut transform = Transform3D::identity();
        let ok = unsafe {
            crate::ffi::ca_layer_get_sublayer_transform(
                self.as_ptr(),
                (&mut transform as *mut Transform3D).cast::<core::ffi::c_void>(),
            )
        };
        if ok {
            transform
        } else {
            Transform3D::identity()
        }
    }

    pub fn set_sublayer_transform(&self, transform: Transform3D) {
        unsafe {
            crate::ffi::ca_layer_set_sublayer_transform(
                self.as_ptr(),
                (&transform as *const Transform3D).cast::<core::ffi::c_void>(),
            )
        };
    }

    #[must_use]
    pub fn is_double_sided(&self) -> bool {
        unsafe { crate::ffi::ca_layer_get_double_sided(self.as_ptr()) }
    }

    pub fn set_double_sided(&self, value: bool) {
        unsafe { crate::ffi::ca_layer_set_double_sided(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn is_geometry_flipped(&self) -> bool {
        unsafe { crate::ffi::ca_layer_get_geometry_flipped(self.as_ptr()) }
    }

    pub fn set_geometry_flipped(&self, value: bool) {
        unsafe { crate::ffi::ca_layer_set_geometry_flipped(self.as_ptr(), value) };
    }
}
