#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrameRateRange {
    pub minimum: f32,
    pub maximum: f32,
    pub preferred: f32,
}

impl FrameRateRange {
    pub const DEFAULT: Self = Self::new(0.0, 0.0, 0.0);

    #[must_use]
    pub const fn new(minimum: f32, maximum: f32, preferred: f32) -> Self {
        Self {
            minimum,
            maximum,
            preferred,
        }
    }

    #[must_use]
    pub const fn make(minimum: f32, maximum: f32, preferred: f32) -> Self {
        Self::new(minimum, maximum, preferred)
    }

    #[must_use]
    pub fn is_equal_to_range(self, other: Self) -> bool {
        self == other
    }
}

impl Default for FrameRateRange {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[cfg(test)]
mod tests {
    use super::FrameRateRange;

    fn assert_close(left: f32, right: f32) {
        assert!(
            (left - right).abs() < f32::EPSILON,
            "expected {left} to match {right}"
        );
    }

    #[test]
    fn default_matches_default_constant() {
        assert!(FrameRateRange::default().is_equal_to_range(FrameRateRange::DEFAULT));
    }

    #[test]
    fn new_and_make_build_identical_ranges() {
        let constructed = FrameRateRange::new(24.0, 120.0, 60.0);
        let made = FrameRateRange::make(24.0, 120.0, 60.0);

        assert!(constructed.is_equal_to_range(made));
        assert_close(constructed.minimum, 24.0);
        assert_close(constructed.maximum, 120.0);
        assert_close(constructed.preferred, 60.0);
    }

    #[test]
    fn equality_helper_matches_struct_equality() {
        let lhs = FrameRateRange::new(30.0, 120.0, 60.0);
        let rhs = FrameRateRange::new(30.0, 120.0, 60.0);
        let different = FrameRateRange::new(24.0, 60.0, 30.0);

        assert!(lhs.is_equal_to_range(rhs));
        assert!(!lhs.is_equal_to_range(different));
    }
}
