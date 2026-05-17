use core::ffi::c_void;

use crate::animation::AnimationLike;
use crate::layer::LayerLike;
use crate::private::{cstring_from_str, handle_type};

handle_type!(Action);

pub trait ActionLike {
    fn as_action_ptr(&self) -> *mut c_void;
}

impl ActionLike for Action {
    fn as_action_ptr(&self) -> *mut c_void {
        self.as_ptr()
    }
}

impl<T: AnimationLike> ActionLike for T {
    fn as_action_ptr(&self) -> *mut c_void {
        self.as_animation_ptr()
    }
}

impl Action {
    #[must_use]
    pub fn null() -> Option<Self> {
        unsafe { Self::from_raw(crate::ffi::ca_action_null()) }
    }

    #[must_use]
    pub fn retained_from<A: ActionLike>(action: &A) -> Self {
        let ptr = unsafe { crate::ffi::ca_retain(action.as_action_ptr()) };
        debug_assert!(!ptr.is_null());
        unsafe { Self::from_raw_unchecked(ptr) }
    }

    pub fn run_for_key<L: LayerLike>(&self, event: &str, object: &L) {
        if let Some(event) = cstring_from_str(event) {
            unsafe {
                crate::ffi::ca_action_run_for_key(
                    self.as_ptr(),
                    event.as_ptr(),
                    object.as_layer_ptr(),
                )
            };
        }
    }
}
