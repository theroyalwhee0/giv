use thiserror::Error;

/// Application errors.
#[derive(Error, Debug)]
pub enum GivError {
    /// Error when the requested number of PI decimal places is out of range.
    #[cfg(feature = "pi")]
    #[error(
        "Requested number of PI decimal places '{0}' is not supported please select a value between '1' and '{1}'"
    )]
    DecimalPlacesOutOfRange(usize, usize),
}
