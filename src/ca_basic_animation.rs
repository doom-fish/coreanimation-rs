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
}
