use crate::animation::{
    AnimationLike, BasicAnimation, KeyframeAnimation, PropertyAnimation, SpringAnimation,
};
use crate::ca_value_function::ValueFunction;

macro_rules! property_animation_ext {
    ($ty:ty) => {
        impl $ty {
            #[must_use]
            pub fn additive(&self) -> bool {
                unsafe { crate::ffi::ca_property_animation_get_additive(self.as_animation_ptr()) }
            }

            pub fn set_additive(&self, value: bool) {
                unsafe {
                    crate::ffi::ca_property_animation_set_additive(self.as_animation_ptr(), value)
                };
            }

            #[must_use]
            pub fn cumulative(&self) -> bool {
                unsafe { crate::ffi::ca_property_animation_get_cumulative(self.as_animation_ptr()) }
            }

            pub fn set_cumulative(&self, value: bool) {
                unsafe {
                    crate::ffi::ca_property_animation_set_cumulative(self.as_animation_ptr(), value)
                };
            }

            #[must_use]
            pub fn value_function(&self) -> Option<ValueFunction> {
                unsafe {
                    ValueFunction::from_raw(crate::ffi::ca_property_animation_get_value_function(
                        self.as_animation_ptr(),
                    ))
                }
            }

            pub fn set_value_function(&self, value: Option<&ValueFunction>) {
                unsafe {
                    crate::ffi::ca_property_animation_set_value_function(
                        self.as_animation_ptr(),
                        value.map_or(core::ptr::null_mut(), ValueFunction::as_ptr),
                    )
                };
            }
        }
    };
}

property_animation_ext!(PropertyAnimation);
property_animation_ext!(BasicAnimation);
property_animation_ext!(KeyframeAnimation);
property_animation_ext!(SpringAnimation);
