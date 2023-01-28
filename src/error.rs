/// Error type for puid.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PuidError {
    /// The prefix has an invalid format.
    InvalidPrefix,
}

/// ...
pub type PuidResult<T> = Result<T, PuidError>;

impl std::fmt::Display for PuidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PuidError::InvalidPrefix => {
                write!(f, "Prefix cannot be longer than 4 characters and with non-alphanumeric characters.")
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
            "Prefix cannot be longer than 4 characters and with non-alphanumeric characters."
        )
    }
}
