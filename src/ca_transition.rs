use crate::animation::{AnimationLike, Transition};
use crate::error::CoreAnimationError;
use crate::private::cstring_from_str;

impl Transition {
    #[must_use]
    pub fn filter_name(&self) -> Option<String> {
        let ptr = unsafe { crate::ffi::ca_transition_copy_filter_name(self.as_animation_ptr()) };
        if ptr.is_null() {
            return None;
        }
        let name = unsafe { std::ffi::CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr.cast()) };
        Some(name)
    }

    pub fn set_filter_name(&self, name: Option<&str>) -> Result<(), CoreAnimationError> {
        let name = name
            .map(|name| {
                cstring_from_str(name)
                    .ok_or_else(|| CoreAnimationError::new("filter name contains NUL"))
            })
            .transpose()?;
        let accepted = unsafe {
            crate::ffi::ca_transition_set_filter_name(
                self.as_animation_ptr(),
                name.as_ref()
                    .map_or(core::ptr::null(), |name| name.as_ptr()),
            )
        };
        if accepted {
            Ok(())
        } else {
            Err(CoreAnimationError::new(
                "not a Core Image transition filter (kCICategoryTransition)",
            ))
        }
    }

    #[must_use]
    pub fn has_subtype(&self) -> bool {
        unsafe { crate::ffi::ca_transition_has_subtype(self.as_animation_ptr()) }
    }

    pub fn clear_subtype(&self) {
        unsafe { crate::ffi::ca_transition_clear_subtype(self.as_animation_ptr()) };
    }
}
