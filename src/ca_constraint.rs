use std::ffi::CStr;
use std::ops::Deref;

use crate::private::{cstring_from_str, handle_type};

handle_type!(Constraint);
handle_type!(LayoutManager);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ConstraintAttribute {
    MinX = 0,
    MidX = 1,
    MaxX = 2,
    Width = 3,
    MinY = 4,
    MidY = 5,
    MaxY = 6,
    Height = 7,
}

impl ConstraintAttribute {
    pub(crate) const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::MidX,
            2 => Self::MaxX,
            3 => Self::Width,
            4 => Self::MinY,
            5 => Self::MidY,
            6 => Self::MaxY,
            7 => Self::Height,
            _ => Self::MinX,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConstraintLayoutManager {
    inner: LayoutManager,
}

impl ConstraintLayoutManager {
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { LayoutManager::from_raw(crate::ffi::ca_constraint_layout_manager_new()) }
            .map(|inner| Self { inner })
    }
}

impl Deref for ConstraintLayoutManager {
    type Target = LayoutManager;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Constraint {
    #[must_use]
    pub fn new(
        attribute: ConstraintAttribute,
        source_name: &str,
        source_attribute: ConstraintAttribute,
        scale: f64,
        offset: f64,
    ) -> Option<Self> {
        let source_name = cstring_from_str(source_name)?;
        unsafe {
            Self::from_raw(crate::ffi::ca_constraint_new(
                attribute as i32,
                source_name.as_ptr(),
                source_attribute as i32,
                scale,
                offset,
            ))
        }
    }

    #[must_use]
    pub fn with_offset(
        attribute: ConstraintAttribute,
        source_name: &str,
        source_attribute: ConstraintAttribute,
        offset: f64,
    ) -> Option<Self> {
        Self::new(attribute, source_name, source_attribute, 1.0, offset)
    }

    #[must_use]
    pub fn relative_to(
        attribute: ConstraintAttribute,
        source_name: &str,
        source_attribute: ConstraintAttribute,
    ) -> Option<Self> {
        Self::new(attribute, source_name, source_attribute, 1.0, 0.0)
    }

    #[must_use]
    pub fn attribute(&self) -> ConstraintAttribute {
        ConstraintAttribute::from_raw(unsafe { crate::ffi::ca_constraint_get_attribute(self.as_ptr()) })
    }

    #[must_use]
    pub fn source_name(&self) -> Option<String> {
        take_c_string(unsafe { crate::ffi::ca_constraint_get_source_name(self.as_ptr()) })
    }

    #[must_use]
    pub fn source_attribute(&self) -> ConstraintAttribute {
        ConstraintAttribute::from_raw(unsafe {
            crate::ffi::ca_constraint_get_source_attribute(self.as_ptr())
        })
    }

    #[must_use]
    pub fn scale(&self) -> f64 {
        unsafe { crate::ffi::ca_constraint_get_scale(self.as_ptr()) }
    }

    #[must_use]
    pub fn offset(&self) -> f64 {
        unsafe { crate::ffi::ca_constraint_get_offset(self.as_ptr()) }
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
