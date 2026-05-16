use crate::animation::Animation;
use crate::private::handle_type;
use crate::transaction::Transaction;

handle_type!(TimingFunction);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TimingFunctionName {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
    Default = 4,
}

impl TimingFunctionName {
    pub(crate) const fn from_raw(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Linear),
            1 => Some(Self::EaseIn),
            2 => Some(Self::EaseOut),
            3 => Some(Self::EaseInEaseOut),
            4 => Some(Self::Default),
            _ => None,
        }
    }

    pub(crate) const fn raw(self) -> i32 {
        self as i32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MediaTimingFillMode {
    Removed = 0,
    Forwards = 1,
    Backwards = 2,
    Both = 3,
}

impl MediaTimingFillMode {
    pub(crate) const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::Forwards,
            2 => Self::Backwards,
            3 => Self::Both,
            _ => Self::Removed,
        }
    }
}

impl TimingFunction {
    #[must_use]
    pub fn with_name(name: TimingFunctionName) -> Option<Self> {
        unsafe { Self::from_raw(crate::ffi::ca_timing_function_new_named(name.raw())) }
    }

    #[must_use]
    pub fn with_control_points(c1x: f32, c1y: f32, c2x: f32, c2y: f32) -> Option<Self> {
        unsafe {
            Self::from_raw(crate::ffi::ca_timing_function_new_control_points(
                c1x, c1y, c2x, c2y,
            ))
        }
    }

    #[must_use]
    pub fn name(&self) -> Option<TimingFunctionName> {
        TimingFunctionName::from_raw(unsafe {
            crate::ffi::ca_timing_function_get_name(self.as_ptr())
        })
    }

    #[must_use]
    pub fn control_point(&self, index: usize) -> Option<(f32, f32)> {
        let mut values = [0.0_f32; 2];
        let ok = unsafe {
            crate::ffi::ca_timing_function_get_control_point(
                self.as_ptr(),
                index,
                values.as_mut_ptr().cast::<core::ffi::c_void>(),
            )
        };
        ok.then_some((values[0], values[1]))
    }
}

impl Animation {
    #[must_use]
    pub fn timing_function(&self) -> Option<TimingFunction> {
        unsafe {
            TimingFunction::from_raw(crate::ffi::ca_animation_get_timing_function(self.as_ptr()))
        }
    }

    pub fn set_timing_function(&self, value: Option<&TimingFunction>) {
        unsafe {
            crate::ffi::ca_animation_set_timing_function(
                self.as_ptr(),
                value.map_or(core::ptr::null_mut(), TimingFunction::as_ptr),
            )
        };
    }

    #[must_use]
    pub fn begin_time(&self) -> f64 {
        unsafe { crate::ffi::ca_animation_get_begin_time(self.as_ptr()) }
    }

    pub fn set_begin_time(&self, value: f64) {
        unsafe { crate::ffi::ca_animation_set_begin_time(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn speed(&self) -> f32 {
        unsafe { crate::ffi::ca_animation_get_speed(self.as_ptr()) }
    }

    pub fn set_speed(&self, value: f32) {
        unsafe { crate::ffi::ca_animation_set_speed(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn time_offset(&self) -> f64 {
        unsafe { crate::ffi::ca_animation_get_time_offset(self.as_ptr()) }
    }

    pub fn set_time_offset(&self, value: f64) {
        unsafe { crate::ffi::ca_animation_set_time_offset(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn repeat_duration(&self) -> f64 {
        unsafe { crate::ffi::ca_animation_get_repeat_duration(self.as_ptr()) }
    }

    pub fn set_repeat_duration(&self, value: f64) {
        unsafe { crate::ffi::ca_animation_set_repeat_duration(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn fill_mode(&self) -> MediaTimingFillMode {
        MediaTimingFillMode::from_raw(unsafe {
            crate::ffi::ca_animation_get_fill_mode(self.as_ptr())
        })
    }

    pub fn set_fill_mode(&self, value: MediaTimingFillMode) {
        unsafe { crate::ffi::ca_animation_set_fill_mode(self.as_ptr(), value as i32) };
    }
}

impl Transaction {
    #[must_use]
    pub fn animation_timing_function() -> Option<TimingFunction> {
        unsafe {
            TimingFunction::from_raw(crate::ffi::ca_transaction_get_animation_timing_function())
        }
    }

    pub fn set_animation_timing_function(value: Option<&TimingFunction>) {
        unsafe {
            crate::ffi::ca_transaction_set_animation_timing_function(
                value.map_or(core::ptr::null_mut(), TimingFunction::as_ptr),
            )
        };
    }
}
