use apple_cf::cg::CGRect;

use crate::private::handle_type;

handle_type!(Path);

impl Path {
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(crate::ffi::ca_path_new_mutable()) }
    }

    pub fn move_to(&self, x: f64, y: f64) {
        unsafe { crate::ffi::ca_path_move_to(self.as_ptr(), x, y) };
    }

    pub fn add_line_to(&self, x: f64, y: f64) {
        unsafe { crate::ffi::ca_path_add_line_to(self.as_ptr(), x, y) };
    }

    pub fn add_rect(&self, rect: CGRect) {
        unsafe {
            crate::ffi::ca_path_add_rect(self.as_ptr(), rect.x, rect.y, rect.width, rect.height)
        };
    }

    pub fn add_ellipse(&self, rect: CGRect) {
        unsafe {
            crate::ffi::ca_path_add_ellipse(self.as_ptr(), rect.x, rect.y, rect.width, rect.height)
        };
    }

    pub fn close_subpath(&self) {
        unsafe { crate::ffi::ca_path_close_subpath(self.as_ptr()) };
    }

    #[must_use]
    pub fn bounding_box(&self) -> CGRect {
        let mut rect = CGRect::zero();
        let ok = unsafe {
            crate::ffi::ca_path_get_bounding_box(
                self.as_ptr(),
                (&mut rect as *mut CGRect).cast::<core::ffi::c_void>(),
            )
        };
        if ok {
            rect
        } else {
            CGRect::zero()
        }
    }
}
