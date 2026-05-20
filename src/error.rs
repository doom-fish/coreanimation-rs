use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreAnimationError {
    message: String,
}

impl CoreAnimationError {
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for CoreAnimationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for CoreAnimationError {}

#[cfg(test)]
mod tests {
    use super::CoreAnimationError;

    #[test]
    fn new_and_display_round_trip_message() {
        let error = CoreAnimationError::new("layer failure");

        assert_eq!(error.to_string(), "layer failure");
    }

    #[test]
    fn cloned_errors_preserve_message_and_have_no_source() {
        let error = CoreAnimationError::new("animation failure");
        let cloned = error.clone();
        let as_std_error: &dyn std::error::Error = &cloned;

        assert_eq!(cloned, error);
        assert!(as_std_error.source().is_none());
    }
}
