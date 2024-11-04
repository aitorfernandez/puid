/// Type for work with errors produced by Puid.
#[derive(Clone, Debug)]
pub enum PuidError {
    /// Error occurred when the prefix has an invalid format or empty.
    InvalidPrefix,
}

/// A `Result` alias type for Puid.
pub type PuidResult<T> = Result<T, PuidError>;

impl std::fmt::Display for PuidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PuidError::InvalidPrefix => {
                write!(f, "Prefix cannot be longer than 8 characters with non-alphanumeric characters or non empty.")
            }
        }
    }
}

impl std::error::Error for PuidError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puid_error_invalid_prefix_test() {
        let err = PuidError::InvalidPrefix;
        assert_eq!(
            err.to_string(),
            "Prefix cannot be longer than 8 characters with non-alphanumeric characters or non empty."
        )
    }
}
