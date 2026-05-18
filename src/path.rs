use apple_cf::cg::CGRect;

use crate::private::handle_type;

handle_type!(Path);

impl Path {
    #[must_use]
    /// Creates a new mutable path.
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(crate::ffi::ca_path_new_mutable()) }
    }

    /// Moves the current point to the supplied coordinates.
    pub fn move_to(&self, x: f64, y: f64) {
        unsafe { crate::ffi::ca_path_move_to(self.as_ptr(), x, y) };
    }

    /// Appends a line segment to the path.
    pub fn add_line_to(&self, x: f64, y: f64) {
        unsafe { crate::ffi::ca_path_add_line_to(self.as_ptr(), x, y) };
    }

    /// Appends a rectangle to the path.
    pub fn add_rect(&self, rect: CGRect) {
        unsafe {
            crate::ffi::ca_path_add_rect(
                self.as_ptr(),
                rect.origin.x,
                rect.origin.y,
                rect.size.width,
                rect.size.height,
            )
        };
    }

    /// Appends an ellipse to the path.
    pub fn add_ellipse(&self, rect: CGRect) {
        unsafe {
            crate::ffi::ca_path_add_ellipse(
                self.as_ptr(),
                rect.origin.x,
                rect.origin.y,
                rect.size.width,
                rect.size.height,
            )
        };
    }

    /// Closes the current subpath.
    pub fn close_subpath(&self) {
        unsafe { crate::ffi::ca_path_close_subpath(self.as_ptr()) };
    }

    #[must_use]
    /// Returns the path bounding box.
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
