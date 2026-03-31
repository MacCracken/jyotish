use thiserror::Error;

/// Errors that can occur during jyotish computations.
///
/// # Examples
///
/// ```
/// # use jyotish::*;
/// let err = JyotishError::InvalidParameter("negative longitude".into());
/// assert!(err.to_string().contains("negative longitude"));
///
/// let err = JyotishError::DateError("invalid Julian Day".into());
/// assert!(err.to_string().contains("invalid Julian Day"));
/// ```
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum JyotishError {
    /// An invalid parameter was provided to a computation.
    #[error("invalid parameter: {0}")]
    InvalidParameter(String),

    /// A mathematical computation failed.
    #[error("math error: {0}")]
    MathError(String),

    /// A date/time conversion or validation failed.
    #[error("date error: {0}")]
    DateError(String),

    /// An ephemeris lookup or interpolation failed.
    #[error("ephemeris error: {0}")]
    EphemerisError(String),

    /// An I/O operation failed.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// A convenience type alias for `Result<T, JyotishError>`.
pub type Result<T> = std::result::Result<T, JyotishError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display() {
        let e = JyotishError::InvalidParameter("negative longitude".into());
        assert!(e.to_string().contains("negative longitude"));
    }

    #[test]
    fn error_math() {
        let e = JyotishError::MathError("division by zero".into());
        assert!(e.to_string().contains("division by zero"));
    }

    #[test]
    fn error_date() {
        let e = JyotishError::DateError("invalid Julian Day".into());
        assert!(e.to_string().contains("invalid Julian Day"));
    }

    #[test]
    fn error_ephemeris() {
        let e = JyotishError::EphemerisError("missing data".into());
        assert!(e.to_string().contains("missing data"));
    }

    #[test]
    fn error_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
        let e = JyotishError::from(io_err);
        assert!(e.to_string().contains("file missing"));
    }
}
