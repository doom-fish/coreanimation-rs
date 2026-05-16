use crate::animation::{AnimationLike, KeyframeAnimation};
use crate::ca_media_timing::TimingFunctionName;

impl KeyframeAnimation {
    pub fn set_timing_function_names(&self, values: &[TimingFunctionName]) {
        let raw: Vec<i32> = values.iter().map(|value| value.raw()).collect();
        unsafe {
            crate::ffi::ca_keyframe_animation_set_timing_function_names(
                self.as_animation_ptr(),
                raw.as_ptr(),
                raw.len(),
            )
        };
    }

    #[must_use]
    pub fn timing_function_names(&self) -> Vec<TimingFunctionName> {
        let count = unsafe {
            crate::ffi::ca_keyframe_animation_timing_function_name_count(self.as_animation_ptr())
        };
        (0..count)
            .filter_map(|index| {
                TimingFunctionName::from_raw(unsafe {
                    crate::ffi::ca_keyframe_animation_timing_function_name_at(
                        self.as_animation_ptr(),
                        index,
                    )
                })
            })
            .collect()
    }

    pub fn set_tension_values(&self, values: &[f64]) {
        unsafe {
            crate::ffi::ca_keyframe_animation_set_tension_values(
                self.as_animation_ptr(),
                values.as_ptr(),
                values.len(),
            )
        };
    }

    #[must_use]
    pub fn tension_values(&self) -> Vec<f64> {
        let count = unsafe {
            crate::ffi::ca_keyframe_animation_tension_value_count(self.as_animation_ptr())
        };
        (0..count)
            .map(|index| unsafe {
                crate::ffi::ca_keyframe_animation_tension_value_at(self.as_animation_ptr(), index)
            })
            .collect()
    }

    pub fn set_continuity_values(&self, values: &[f64]) {
        unsafe {
            crate::ffi::ca_keyframe_animation_set_continuity_values(
                self.as_animation_ptr(),
                values.as_ptr(),
                values.len(),
            )
        };
    }

    #[must_use]
    pub fn continuity_values(&self) -> Vec<f64> {
        let count = unsafe {
            crate::ffi::ca_keyframe_animation_continuity_value_count(self.as_animation_ptr())
        };
        (0..count)
            .map(|index| unsafe {
                crate::ffi::ca_keyframe_animation_continuity_value_at(
                    self.as_animation_ptr(),
                    index,
                )
            })
            .collect()
    }

    pub fn set_bias_values(&self, values: &[f64]) {
        unsafe {
            crate::ffi::ca_keyframe_animation_set_bias_values(
                self.as_animation_ptr(),
                values.as_ptr(),
                values.len(),
            )
        };
    }

    #[must_use]
    pub fn bias_values(&self) -> Vec<f64> {
        let count =
            unsafe { crate::ffi::ca_keyframe_animation_bias_value_count(self.as_animation_ptr()) };
        (0..count)
            .map(|index| unsafe {
                crate::ffi::ca_keyframe_animation_bias_value_at(self.as_animation_ptr(), index)
            })
            .collect()
    }
}
