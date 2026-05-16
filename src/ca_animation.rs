use crate::animation::Animation;
use crate::ca_media_timing::TimingFunctionName;

impl Animation {
    #[must_use]
    pub fn timing_function_name(&self) -> Option<TimingFunctionName> {
        TimingFunctionName::from_raw(unsafe {
            crate::ffi::ca_animation_get_timing_function_name(self.as_ptr())
        })
    }

    pub fn set_timing_function_name(&self, value: Option<TimingFunctionName>) {
        unsafe {
            crate::ffi::ca_animation_set_timing_function_name(
                self.as_ptr(),
                value.map_or(-1, TimingFunctionName::raw),
            )
        };
    }
}
