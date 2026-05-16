use crate::animation::{AnimationLike, BasicAnimation};

impl BasicAnimation {
    #[must_use]
    pub fn from_number(&self) -> Option<f64> {
        let mut value = 0.0;
        let ok = unsafe {
            crate::ffi::ca_basic_animation_get_from_number(
                self.as_animation_ptr(),
                (&mut value as *mut f64).cast::<core::ffi::c_void>(),
            )
        };
        ok.then_some(value)
    }

    #[must_use]
    pub fn to_number(&self) -> Option<f64> {
        let mut value = 0.0;
        let ok = unsafe {
            crate::ffi::ca_basic_animation_get_to_number(
                self.as_animation_ptr(),
                (&mut value as *mut f64).cast::<core::ffi::c_void>(),
            )
        };
        ok.then_some(value)
    }

    #[must_use]
    pub fn by_number(&self) -> Option<f64> {
        let mut value = 0.0;
        let ok = unsafe {
            crate::ffi::ca_basic_animation_get_by_number(
                self.as_animation_ptr(),
                (&mut value as *mut f64).cast::<core::ffi::c_void>(),
            )
        };
        ok.then_some(value)
    }

    #[must_use]
    pub fn additive(&self) -> bool {
        unsafe { crate::ffi::ca_property_animation_get_additive(self.as_animation_ptr()) }
    }

    pub fn set_additive(&self, value: bool) {
        unsafe { crate::ffi::ca_property_animation_set_additive(self.as_animation_ptr(), value) };
    }

    #[must_use]
    pub fn cumulative(&self) -> bool {
        unsafe { crate::ffi::ca_property_animation_get_cumulative(self.as_animation_ptr()) }
    }

    pub fn set_cumulative(&self, value: bool) {
        unsafe { crate::ffi::ca_property_animation_set_cumulative(self.as_animation_ptr(), value) };
    }
}
