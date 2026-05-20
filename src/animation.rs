use std::ffi::CStr;
use std::ops::Deref;

use crate::path::Path;
use crate::private::{cstring_from_str, handle_type};

handle_type!(Animation);

/// Trait for wrappers that can yield a `CAAnimation` handle.
pub trait AnimationLike {
    fn as_animation_ptr(&self) -> *mut core::ffi::c_void;
}

impl AnimationLike for Animation {
    fn as_animation_ptr(&self) -> *mut core::ffi::c_void {
        self.as_ptr()
    }
}

macro_rules! animation_wrapper {
    ($name:ident, $ctor:expr) => {
        #[derive(Debug, Clone)]
        /// Safe wrapper around the corresponding `Core Animation` animation type.
        pub struct $name {
            inner: Animation,
        }

        impl Deref for $name {
            type Target = Animation;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl AnimationLike for $name {
            fn as_animation_ptr(&self) -> *mut core::ffi::c_void {
                self.inner.as_ptr()
            }
        }

        impl $name {
            #[must_use]
            /// Creates a new wrapper instance.
            pub fn new(key_path: Option<&str>) -> Option<Self> {
                let key_path = key_path.and_then(cstring_from_str);
                unsafe {
                    Animation::from_raw($ctor(
                        key_path
                            .as_ref()
                            .map_or(core::ptr::null(), |value| value.as_ptr()),
                    ))
                }
                .map(|inner| Self { inner })
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
/// Mirrors `CAAnimationCalculationMode` values.
pub enum AnimationCalculationMode {
    Linear = 0,
    Discrete = 1,
    Paced = 2,
    Cubic = 3,
    CubicPaced = 4,
}

impl AnimationCalculationMode {
    const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::Discrete,
            2 => Self::Paced,
            3 => Self::Cubic,
            4 => Self::CubicPaced,
            _ => Self::Linear,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
/// Mirrors `CAAnimationRotationMode` values.
pub enum RotationMode {
    None = 0,
    Auto = 1,
    AutoReverse = 2,
}

impl RotationMode {
    const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::Auto,
            2 => Self::AutoReverse,
            _ => Self::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
/// Mirrors `CATransitionType` values.
pub enum TransitionType {
    Fade = 0,
    MoveIn = 1,
    Push = 2,
    Reveal = 3,
}

impl TransitionType {
    const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::MoveIn,
            2 => Self::Push,
            3 => Self::Reveal,
            _ => Self::Fade,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
/// Mirrors `CATransitionSubtype` values.
pub enum TransitionSubtype {
    None = 0,
    FromRight = 1,
    FromLeft = 2,
    FromTop = 3,
    FromBottom = 4,
}

impl TransitionSubtype {
    const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::FromRight,
            2 => Self::FromLeft,
            3 => Self::FromTop,
            4 => Self::FromBottom,
            _ => Self::None,
        }
    }
}

animation_wrapper!(BasicAnimation, crate::ffi::ca_basic_animation_new);
animation_wrapper!(KeyframeAnimation, crate::ffi::ca_keyframe_animation_new);
animation_wrapper!(SpringAnimation, crate::ffi::ca_spring_animation_new);

#[derive(Debug, Clone)]
/// Safe wrapper around `PropertyAnimation`.
pub struct PropertyAnimation {
    inner: Animation,
}

#[derive(Debug, Clone)]
/// Safe wrapper around `AnimationGroup`.
pub struct AnimationGroup {
    inner: Animation,
}

#[derive(Debug, Clone)]
/// Safe wrapper around `Transition`.
pub struct Transition {
    inner: Animation,
}

impl Animation {
    #[must_use]
    /// Creates a new `CAAnimation` wrapper.
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(crate::ffi::ca_animation_new()) }
    }

    #[must_use]
    /// Returns the animation's duration.
    pub fn duration(&self) -> f64 {
        unsafe { crate::ffi::ca_animation_get_duration(self.as_ptr()) }
    }

    /// Sets the animation's duration.
    pub fn set_duration(&self, value: f64) {
        unsafe { crate::ffi::ca_animation_set_duration(self.as_ptr(), value) };
    }

    #[must_use]
    /// Returns the animation's repeat count.
    pub fn repeat_count(&self) -> f32 {
        unsafe { crate::ffi::ca_animation_get_repeat_count(self.as_ptr()) }
    }

    /// Sets the animation's repeat count.
    pub fn set_repeat_count(&self, value: f32) {
        unsafe { crate::ffi::ca_animation_set_repeat_count(self.as_ptr(), value) };
    }

    #[must_use]
    /// Returns whether the animation autoreverses.
    pub fn autoreverses(&self) -> bool {
        unsafe { crate::ffi::ca_animation_get_autoreverses(self.as_ptr()) }
    }

    /// Sets the animation's autoreverses.
    pub fn set_autoreverses(&self, value: bool) {
        unsafe { crate::ffi::ca_animation_set_autoreverses(self.as_ptr(), value) };
    }

    #[must_use]
    /// Returns whether the animation removed on completion.
    pub fn removed_on_completion(&self) -> bool {
        unsafe { crate::ffi::ca_animation_get_removed_on_completion(self.as_ptr()) }
    }

    /// Sets the animation's removed on completion.
    pub fn set_removed_on_completion(&self, value: bool) {
        unsafe { crate::ffi::ca_animation_set_removed_on_completion(self.as_ptr(), value) };
    }
}

impl PropertyAnimation {
    #[must_use]
    /// Creates a new `CAPropertyAnimation` wrapper.
    pub fn new(key_path: Option<&str>) -> Option<Self> {
        let key_path = key_path.and_then(cstring_from_str);
        unsafe {
            Animation::from_raw(crate::ffi::ca_property_animation_new(
                key_path
                    .as_ref()
                    .map_or(core::ptr::null(), |value| value.as_ptr()),
            ))
        }
        .map(|inner| Self { inner })
    }

    #[must_use]
    /// Returns the property animation's key path.
    pub fn key_path(&self) -> Option<String> {
        take_c_string(unsafe {
            crate::ffi::ca_property_animation_get_key_path(self.as_animation_ptr())
        })
    }

    /// Sets the property animation's key path.
    pub fn set_key_path(&self, key_path: &str) {
        if let Some(key_path) = cstring_from_str(key_path) {
            unsafe {
                crate::ffi::ca_property_animation_set_key_path(
                    self.as_animation_ptr(),
                    key_path.as_ptr(),
                )
            };
        }
    }
}

impl Deref for PropertyAnimation {
    type Target = Animation;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AnimationLike for PropertyAnimation {
    fn as_animation_ptr(&self) -> *mut core::ffi::c_void {
        self.inner.as_ptr()
    }
}

impl BasicAnimation {
    #[must_use]
    /// Returns the basic animation's key path.
    pub fn key_path(&self) -> Option<String> {
        take_c_string(unsafe {
            crate::ffi::ca_property_animation_get_key_path(self.as_animation_ptr())
        })
    }

    /// Sets the basic animation's key path.
    pub fn set_key_path(&self, key_path: &str) {
        if let Some(key_path) = cstring_from_str(key_path) {
            unsafe {
                crate::ffi::ca_property_animation_set_key_path(
                    self.as_animation_ptr(),
                    key_path.as_ptr(),
                )
            };
        }
    }

    /// Sets the basic animation's from number.
    pub fn set_from_number(&self, value: f64) {
        unsafe { crate::ffi::ca_basic_animation_set_from_number(self.as_animation_ptr(), value) };
    }

    /// Sets the basic animation's to number.
    pub fn set_to_number(&self, value: f64) {
        unsafe { crate::ffi::ca_basic_animation_set_to_number(self.as_animation_ptr(), value) };
    }

    /// Sets the basic animation's by number.
    pub fn set_by_number(&self, value: f64) {
        unsafe { crate::ffi::ca_basic_animation_set_by_number(self.as_animation_ptr(), value) };
    }
}

impl KeyframeAnimation {
    #[must_use]
    /// Returns the keyframe animation's key path.
    pub fn key_path(&self) -> Option<String> {
        take_c_string(unsafe {
            crate::ffi::ca_property_animation_get_key_path(self.as_animation_ptr())
        })
    }

    /// Sets the keyframe animation's key path.
    pub fn set_key_path(&self, key_path: &str) {
        if let Some(key_path) = cstring_from_str(key_path) {
            unsafe {
                crate::ffi::ca_property_animation_set_key_path(
                    self.as_animation_ptr(),
                    key_path.as_ptr(),
                )
            };
        }
    }

    /// Sets the keyframe animation's values.
    pub fn set_values(&self, values: &[f64]) {
        unsafe {
            crate::ffi::ca_keyframe_animation_set_values(
                self.as_animation_ptr(),
                values.as_ptr(),
                values.len(),
            )
        };
    }

    #[must_use]
    /// Returns the keyframe animation's values.
    pub fn values(&self) -> Vec<f64> {
        let count =
            unsafe { crate::ffi::ca_keyframe_animation_value_count(self.as_animation_ptr()) };
        (0..count)
            .map(|index| unsafe {
                crate::ffi::ca_keyframe_animation_value_at(self.as_animation_ptr(), index)
            })
            .collect()
    }

    /// Sets the keyframe animation's path.
    pub fn set_path(&self, path: Option<&Path>) {
        unsafe {
            crate::ffi::ca_keyframe_animation_set_path(
                self.as_animation_ptr(),
                path.map_or(core::ptr::null_mut(), Path::as_ptr),
            )
        };
    }

    #[must_use]
    /// Returns the keyframe animation's path.
    pub fn path(&self) -> Option<Path> {
        unsafe {
            Path::from_raw(crate::ffi::ca_keyframe_animation_get_path(
                self.as_animation_ptr(),
            ))
        }
    }

    /// Sets the keyframe animation's key times.
    pub fn set_key_times(&self, values: &[f64]) {
        unsafe {
            crate::ffi::ca_keyframe_animation_set_key_times(
                self.as_animation_ptr(),
                values.as_ptr(),
                values.len(),
            )
        };
    }

    #[must_use]
    /// Returns the keyframe animation's key times.
    pub fn key_times(&self) -> Vec<f64> {
        let count =
            unsafe { crate::ffi::ca_keyframe_animation_key_time_count(self.as_animation_ptr()) };
        (0..count)
            .map(|index| unsafe {
                crate::ffi::ca_keyframe_animation_key_time_at(self.as_animation_ptr(), index)
            })
            .collect()
    }

    #[must_use]
    /// Returns the keyframe animation's calculation mode.
    pub fn calculation_mode(&self) -> AnimationCalculationMode {
        AnimationCalculationMode::from_raw(unsafe {
            crate::ffi::ca_keyframe_animation_get_calculation_mode(self.as_animation_ptr())
        })
    }

    /// Sets the keyframe animation's calculation mode.
    pub fn set_calculation_mode(&self, value: AnimationCalculationMode) {
        unsafe {
            crate::ffi::ca_keyframe_animation_set_calculation_mode(
                self.as_animation_ptr(),
                value as i32,
            )
        };
    }

    #[must_use]
    /// Returns the keyframe animation's rotation mode.
    pub fn rotation_mode(&self) -> RotationMode {
        RotationMode::from_raw(unsafe {
            crate::ffi::ca_keyframe_animation_get_rotation_mode(self.as_animation_ptr())
        })
    }

    /// Sets the keyframe animation's rotation mode.
    pub fn set_rotation_mode(&self, value: RotationMode) {
        unsafe {
            crate::ffi::ca_keyframe_animation_set_rotation_mode(
                self.as_animation_ptr(),
                value as i32,
            )
        };
    }
}

impl SpringAnimation {
    #[must_use]
    /// Returns the spring animation's key path.
    pub fn key_path(&self) -> Option<String> {
        take_c_string(unsafe {
            crate::ffi::ca_property_animation_get_key_path(self.as_animation_ptr())
        })
    }

    /// Sets the spring animation's key path.
    pub fn set_key_path(&self, key_path: &str) {
        if let Some(key_path) = cstring_from_str(key_path) {
            unsafe {
                crate::ffi::ca_property_animation_set_key_path(
                    self.as_animation_ptr(),
                    key_path.as_ptr(),
                )
            };
        }
    }

    #[must_use]
    /// Returns the spring animation's mass.
    pub fn mass(&self) -> f64 {
        unsafe { crate::ffi::ca_spring_animation_get_mass(self.as_animation_ptr()) }
    }

    /// Sets the spring animation's mass.
    pub fn set_mass(&self, value: f64) {
        unsafe { crate::ffi::ca_spring_animation_set_mass(self.as_animation_ptr(), value) };
    }

    #[must_use]
    /// Returns the spring animation's stiffness.
    pub fn stiffness(&self) -> f64 {
        unsafe { crate::ffi::ca_spring_animation_get_stiffness(self.as_animation_ptr()) }
    }

    /// Sets the spring animation's stiffness.
    pub fn set_stiffness(&self, value: f64) {
        unsafe { crate::ffi::ca_spring_animation_set_stiffness(self.as_animation_ptr(), value) };
    }

    #[must_use]
    /// Returns the spring animation's damping.
    pub fn damping(&self) -> f64 {
        unsafe { crate::ffi::ca_spring_animation_get_damping(self.as_animation_ptr()) }
    }

    /// Sets the spring animation's damping.
    pub fn set_damping(&self, value: f64) {
        unsafe { crate::ffi::ca_spring_animation_set_damping(self.as_animation_ptr(), value) };
    }

    #[must_use]
    /// Returns the spring animation's initial velocity.
    pub fn initial_velocity(&self) -> f64 {
        unsafe { crate::ffi::ca_spring_animation_get_initial_velocity(self.as_animation_ptr()) }
    }

    /// Sets the spring animation's initial velocity.
    pub fn set_initial_velocity(&self, value: f64) {
        unsafe {
            crate::ffi::ca_spring_animation_set_initial_velocity(self.as_animation_ptr(), value)
        };
    }

    #[must_use]
    /// Returns the spring animation's settling duration.
    pub fn settling_duration(&self) -> f64 {
        unsafe { crate::ffi::ca_spring_animation_get_settling_duration(self.as_animation_ptr()) }
    }
}

impl AnimationGroup {
    #[must_use]
    /// Creates a new `CAAnimationGroup` wrapper.
    pub fn new() -> Option<Self> {
        unsafe { Animation::from_raw(crate::ffi::ca_animation_group_new()) }
            .map(|inner| Self { inner })
    }

    /// Sets the animations contained in the animation group.
    pub fn set_animations(&self, animations: &[&dyn AnimationLike]) {
        let raw: Vec<*mut core::ffi::c_void> = animations
            .iter()
            .map(|animation| animation.as_animation_ptr())
            .collect();
        unsafe {
            crate::ffi::ca_animation_group_set_animations(
                self.as_animation_ptr(),
                raw.as_ptr(),
                raw.len(),
            )
        };
    }

    #[must_use]
    /// Returns the animations contained in the animation group.
    pub fn animations(&self) -> Vec<Animation> {
        let count =
            unsafe { crate::ffi::ca_animation_group_animation_count(self.as_animation_ptr()) };
        (0..count)
            .filter_map(|index| unsafe {
                Animation::from_raw(crate::ffi::ca_animation_group_animation_at(
                    self.as_animation_ptr(),
                    index,
                ))
            })
            .collect()
    }
}

impl Deref for AnimationGroup {
    type Target = Animation;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AnimationLike for AnimationGroup {
    fn as_animation_ptr(&self) -> *mut core::ffi::c_void {
        self.inner.as_ptr()
    }
}

impl Transition {
    #[must_use]
    /// Creates a new `CATransition` wrapper.
    pub fn new() -> Option<Self> {
        unsafe { Animation::from_raw(crate::ffi::ca_transition_new()) }.map(|inner| Self { inner })
    }

    #[must_use]
    /// Returns the transition's transition type.
    pub fn transition_type(&self) -> TransitionType {
        TransitionType::from_raw(unsafe {
            crate::ffi::ca_transition_get_type(self.as_animation_ptr())
        })
    }

    /// Sets the transition's transition type.
    pub fn set_transition_type(&self, value: TransitionType) {
        unsafe { crate::ffi::ca_transition_set_type(self.as_animation_ptr(), value as i32) };
    }

    #[must_use]
    /// Returns the transition's subtype.
    pub fn subtype(&self) -> TransitionSubtype {
        TransitionSubtype::from_raw(unsafe {
            crate::ffi::ca_transition_get_subtype(self.as_animation_ptr())
        })
    }

    /// Sets the transition's subtype.
    pub fn set_subtype(&self, value: TransitionSubtype) {
        unsafe { crate::ffi::ca_transition_set_subtype(self.as_animation_ptr(), value as i32) };
    }

    #[must_use]
    /// Returns the transition's start progress.
    pub fn start_progress(&self) -> f32 {
        unsafe { crate::ffi::ca_transition_get_start_progress(self.as_animation_ptr()) }
    }

    /// Sets the transition's start progress.
    pub fn set_start_progress(&self, value: f32) {
        unsafe { crate::ffi::ca_transition_set_start_progress(self.as_animation_ptr(), value) };
    }

    #[must_use]
    /// Returns the transition's end progress.
    pub fn end_progress(&self) -> f32 {
        unsafe { crate::ffi::ca_transition_get_end_progress(self.as_animation_ptr()) }
    }

    /// Sets the transition's end progress.
    pub fn set_end_progress(&self, value: f32) {
        unsafe { crate::ffi::ca_transition_set_end_progress(self.as_animation_ptr(), value) };
    }
}

impl Deref for Transition {
    type Target = Animation;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AnimationLike for Transition {
    fn as_animation_ptr(&self) -> *mut core::ffi::c_void {
        self.inner.as_ptr()
    }
}

fn take_c_string(ptr: *mut libc::c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    let value = unsafe { CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned();
    unsafe { libc::free(ptr.cast()) };
    Some(value)
}

#[cfg(test)]
mod tests {
    use super::{
        AnimationCalculationMode, RotationMode, TransitionSubtype, TransitionType,
    };

    #[test]
    fn animation_calculation_mode_from_raw_covers_all_variants() {
        assert_eq!(AnimationCalculationMode::from_raw(-1), AnimationCalculationMode::Linear);
        assert_eq!(AnimationCalculationMode::from_raw(1), AnimationCalculationMode::Discrete);
        assert_eq!(AnimationCalculationMode::from_raw(2), AnimationCalculationMode::Paced);
        assert_eq!(AnimationCalculationMode::from_raw(3), AnimationCalculationMode::Cubic);
        assert_eq!(AnimationCalculationMode::from_raw(4), AnimationCalculationMode::CubicPaced);
    }

    #[test]
    fn rotation_mode_from_raw_covers_all_variants() {
        assert_eq!(RotationMode::from_raw(-1), RotationMode::None);
        assert_eq!(RotationMode::from_raw(1), RotationMode::Auto);
        assert_eq!(RotationMode::from_raw(2), RotationMode::AutoReverse);
    }

    #[test]
    fn transition_type_from_raw_covers_all_variants() {
        assert_eq!(TransitionType::from_raw(-1), TransitionType::Fade);
        assert_eq!(TransitionType::from_raw(1), TransitionType::MoveIn);
        assert_eq!(TransitionType::from_raw(2), TransitionType::Push);
        assert_eq!(TransitionType::from_raw(3), TransitionType::Reveal);
    }

    #[test]
    fn transition_subtype_from_raw_covers_all_variants() {
        assert_eq!(TransitionSubtype::from_raw(-1), TransitionSubtype::None);
        assert_eq!(TransitionSubtype::from_raw(1), TransitionSubtype::FromRight);
        assert_eq!(TransitionSubtype::from_raw(2), TransitionSubtype::FromLeft);
        assert_eq!(TransitionSubtype::from_raw(3), TransitionSubtype::FromTop);
        assert_eq!(TransitionSubtype::from_raw(4), TransitionSubtype::FromBottom);
    }
}
