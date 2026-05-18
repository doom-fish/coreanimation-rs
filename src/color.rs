use crate::private::handle_type;

handle_type!(Color);

impl Color {
    #[must_use]
    /// Creates a color from RGBA components.
    pub fn rgba(red: f64, green: f64, blue: f64, alpha: f64) -> Option<Self> {
        unsafe { Self::from_raw(crate::ffi::ca_color_new_rgba(red, green, blue, alpha)) }
    }

    #[must_use]
    /// Returns an opaque red color.
    pub fn red() -> Self {
        Self::rgba(1.0, 0.0, 0.0, 1.0).expect("failed to create red color")
    }

    #[must_use]
    /// Returns an opaque green color.
    pub fn green() -> Self {
        Self::rgba(0.0, 1.0, 0.0, 1.0).expect("failed to create green color")
    }

    #[must_use]
    /// Returns an opaque blue color.
    pub fn blue() -> Self {
        Self::rgba(0.0, 0.0, 1.0, 1.0).expect("failed to create blue color")
    }

    #[must_use]
    /// Returns an opaque black color.
    pub fn black() -> Self {
        Self::rgba(0.0, 0.0, 0.0, 1.0).expect("failed to create black color")
    }

    #[must_use]
    /// Returns an opaque white color.
    pub fn white() -> Self {
        Self::rgba(1.0, 1.0, 1.0, 1.0).expect("failed to create white color")
    }

    #[must_use]
    /// Returns a fully transparent color.
    pub fn clear() -> Self {
        Self::rgba(0.0, 0.0, 0.0, 0.0).expect("failed to create transparent color")
    }

    #[must_use]
    /// Returns the color RGBA components.
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
    /// Returns the color's red component.
    pub fn red_component(&self) -> f64 {
        self.components().0
    }

    #[must_use]
    /// Returns the color's green component.
    pub fn green_component(&self) -> f64 {
        self.components().1
    }

    #[must_use]
    /// Returns the color's blue component.
    pub fn blue_component(&self) -> f64 {
        self.components().2
    }

    #[must_use]
    /// Returns the color's alpha component.
    pub fn alpha_component(&self) -> f64 {
        self.components().3
    }
}
