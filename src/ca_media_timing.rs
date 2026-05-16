use crate::animation::Animation;

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

impl Animation {
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
