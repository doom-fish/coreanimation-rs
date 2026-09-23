use crate::animation::{Animation, AnimationLike, SpringAnimation};
use crate::error::CoreAnimationError;
use crate::private::cstring_from_str;

impl SpringAnimation {
    pub fn with_perceptual_duration(
        key_path: Option<&str>,
        perceptual_duration: f64,
        bounce: f64,
    ) -> Result<Self, CoreAnimationError> {
        if !Self::supports_perceptual_parameters() {
            return Err(CoreAnimationError::new(
                "CASpringAnimation(perceptualDuration:bounce:) requires macOS 14.0 or later",
            ));
        }
        if !(perceptual_duration.is_finite() && perceptual_duration > 0.0 && bounce.is_finite()) {
            return Err(CoreAnimationError::new(
                "perceptual duration must be finite and positive, and bounce finite",
            ));
        }
        let key_path = key_path
            .map(|key_path| {
                cstring_from_str(key_path)
                    .ok_or_else(|| CoreAnimationError::new("key path contains NUL"))
            })
            .transpose()?;
        let ptr = unsafe {
            crate::ffi::ca_spring_animation_new_perceptual(
                key_path
                    .as_ref()
                    .map_or(core::ptr::null(), |key_path| key_path.as_ptr()),
                perceptual_duration,
                bounce,
            )
        };
        unsafe { Animation::from_raw(ptr) }
            .map(|inner| Self { inner })
            .ok_or_else(|| CoreAnimationError::new("failed to create CASpringAnimation"))
    }

    #[must_use]
    pub fn supports_perceptual_parameters() -> bool {
        unsafe { crate::ffi::ca_spring_animation_supports_perceptual_parameters() }
    }

    #[must_use]
    pub fn perceptual_duration(&self) -> Option<f64> {
        Some(unsafe {
            crate::ffi::ca_spring_animation_get_perceptual_duration(self.as_animation_ptr())
        })
        .filter(|value| !value.is_nan())
    }

    #[must_use]
    pub fn bounce(&self) -> Option<f64> {
        Some(unsafe { crate::ffi::ca_spring_animation_get_bounce(self.as_animation_ptr()) })
            .filter(|value| !value.is_nan())
    }

    #[must_use]
    pub fn allows_overdamping(&self) -> bool {
        unsafe { crate::ffi::ca_spring_animation_get_allows_overdamping(self.as_animation_ptr()) }
    }

    pub fn set_allows_overdamping(&self, value: bool) -> Result<(), CoreAnimationError> {
        if unsafe {
            crate::ffi::ca_spring_animation_set_allows_overdamping(self.as_animation_ptr(), value)
        } {
            Ok(())
        } else {
            Err(CoreAnimationError::new(
                "CASpringAnimation.allowsOverdamping requires macOS 14.0 or later",
            ))
        }
    }

    pub fn configure(&self, mass: f64, stiffness: f64, damping: f64, initial_velocity: f64) {
        unsafe {
            crate::ffi::ca_spring_animation_configure(
                self.as_animation_ptr(),
                mass,
                stiffness,
                damping,
                initial_velocity,
            )
        };
    }
}
