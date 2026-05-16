use crate::private::handle_type;

handle_type!(Color);

impl Color {
    #[must_use]
    pub fn rgba(red: f64, green: f64, blue: f64, alpha: f64) -> Option<Self> {
        unsafe { Self::from_raw(crate::ffi::ca_color_new_rgba(red, green, blue, alpha)) }
    }

    #[must_use]
    pub fn red() -> Self {
        Self::rgba(1.0, 0.0, 0.0, 1.0).expect("failed to create red color")
    }

    #[must_use]
    pub fn green() -> Self {
        Self::rgba(0.0, 1.0, 0.0, 1.0).expect("failed to create green color")
    }

    #[must_use]
    pub fn blue() -> Self {
        Self::rgba(0.0, 0.0, 1.0, 1.0).expect("failed to create blue color")
    }

    #[must_use]
    pub fn black() -> Self {
        Self::rgba(0.0, 0.0, 0.0, 1.0).expect("failed to create black color")
    }

    #[must_use]
    pub fn white() -> Self {
        Self::rgba(1.0, 1.0, 1.0, 1.0).expect("failed to create white color")
    }

    #[must_use]
    pub fn clear() -> Self {
        Self::rgba(0.0, 0.0, 0.0, 0.0).expect("failed to create transparent color")
    }

    #[must_use]
    pub fn components(&self) -> (f64, f64, f64, f64) {
        let mut values = [0.0_f64; 4];
        let ok = unsafe {
            crate::ffi::ca_color_get_components(
                self.as_ptr(),
                values.as_mut_ptr().cast::<core::ffi::c_void>(),
            )
        };
        if ok {
            (values[0], values[1], values[2], values[3])
        } else {
            (0.0, 0.0, 0.0, 0.0)
        }
    }

    #[must_use]
    pub fn red_component(&self) -> f64 {
        self.components().0
    }

    #[must_use]
    pub fn green_component(&self) -> f64 {
        self.components().1
    }

    #[must_use]
    pub fn blue_component(&self) -> f64 {
        self.components().2
    }

    #[must_use]
    pub fn alpha_component(&self) -> f64 {
        self.components().3
    }
}
