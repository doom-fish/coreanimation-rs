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
