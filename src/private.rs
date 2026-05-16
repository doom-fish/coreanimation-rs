use std::ffi::CString;

macro_rules! handle_type {
    ($name:ident) => {
        pub struct $name {
            pub(crate) ptr: *mut core::ffi::c_void,
            owned: bool,
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("ptr", &self.ptr)
                    .field("owned", &self.owned)
                    .finish()
            }
        }

        impl Clone for $name {
            fn clone(&self) -> Self {
                let ptr = unsafe { crate::ffi::ca_retain(self.ptr) };
                Self { ptr, owned: true }
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                if self.owned && !self.ptr.is_null() {
                    unsafe { crate::ffi::ca_release(self.ptr) };
                    self.ptr = core::ptr::null_mut();
                }
            }
        }

        #[allow(dead_code)]
        impl $name {
            pub(crate) unsafe fn from_raw(ptr: *mut core::ffi::c_void) -> Option<Self> {
                if ptr.is_null() {
                    None
                } else {
                    Some(Self { ptr, owned: true })
                }
            }

            pub(crate) const unsafe fn from_raw_unchecked(ptr: *mut core::ffi::c_void) -> Self {
                Self { ptr, owned: true }
            }

            pub(crate) const unsafe fn from_raw_borrowed(ptr: *mut core::ffi::c_void) -> Self {
                Self { ptr, owned: false }
            }

            #[must_use]
            pub(crate) const fn as_ptr(&self) -> *mut core::ffi::c_void {
                self.ptr
            }
        }
    };
}

pub(crate) use handle_type;

pub fn cstring_from_str(value: &str) -> Option<CString> {
    CString::new(value).ok()
}
