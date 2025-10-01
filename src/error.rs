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

    /// Error when conflicting flags are provided.
    #[cfg(feature = "pi")]
    #[error("Conflicting flags: {0}")]
    ConflictingFlags(String),

    /// Error when an invalid RNG specification is provided.
    #[cfg(feature = "rng")]
    #[error("Invalid RNG specification: '{0}'. Expected formats: 'XdY' or 'dY' for dice, 'X..Y' for ranges")]
    InvalidRngSpec(String),

    /// Error when a numeric overflow or underflow occurs.
    #[cfg(feature = "rng")]
    #[error("Numeric overflow or underflow in calculation: {0}")]
    NumericOverflow(String),

    /// Error when required arguments are not provided.
    #[error("Required arguments not provided. Use '{0}' for usage information")]
    RequiredArgumentsNotProvided(String),
}
