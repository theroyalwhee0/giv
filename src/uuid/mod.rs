/// Output formatting for uuid generation.
pub mod output;

use crate::error::GivError;
pub use output::UuidOutput;
use uuid::Uuid;

/// Generate a UUID v7.
///
/// # Returns
///
/// Returns a [`UuidOutput`] containing the generated UUID and version information.
///
/// # Errors
///
/// This function does not currently return errors, but returns a Result for consistency.
///
/// # Examples
///
/// ```
/// use giv::uuid::generate_uuid;
/// use giv::GivError;
///
/// # fn main() -> Result<(), GivError> {
/// let uuid = generate_uuid()?;
/// println!("Generated UUID: {}", uuid.uuid);
/// assert_eq!(uuid.version, "v7");
/// # Ok(())
/// # }
/// ```
pub fn generate_uuid() -> Result<UuidOutput, GivError> {
    // Generate a UUID version 7.
    let uuid = Uuid::now_v7();
    // Create output with the UUID.
    Ok(UuidOutput {
        uuid: uuid.to_string(),
        version: "v7".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test UUID generation returns valid UUIDs.
    ///
    /// Verifies that `generate_uuid()` successfully generates UUIDs and that
    /// they follow the expected format (36 characters with hyphens at correct positions).
    #[test]
    fn test_generate_uuid() {
        let output = generate_uuid().unwrap();
        // UUID should be 36 characters (32 hex + 4 hyphens)
        assert_eq!(output.uuid.len(), 36);
        // Should have hyphens in the right places
        assert_eq!(output.uuid.chars().nth(8).unwrap(), '-');
        assert_eq!(output.uuid.chars().nth(13).unwrap(), '-');
        assert_eq!(output.uuid.chars().nth(18).unwrap(), '-');
        assert_eq!(output.uuid.chars().nth(23).unwrap(), '-');
        // Version should be v7
        assert_eq!(output.version, "v7");
    }

    /// Test UUID generation produces unique values.
    ///
    /// Verifies that calling `generate_uuid()` multiple times produces
    /// different UUIDs, as expected from a random generation function.
    #[test]
    fn test_uuid_uniqueness() {
        let uuid1 = generate_uuid().unwrap();
        let uuid2 = generate_uuid().unwrap();
        // Two generated UUIDs should be different
        assert_ne!(uuid1.uuid, uuid2.uuid);
    }

    /// Test UUID format is parseable by uuid crate.
    ///
    /// Verifies that the generated UUID string can be successfully parsed
    /// by the uuid crate, confirming it follows the UUID specification.
    #[test]
    fn test_uuid_format() {
        let output = generate_uuid().unwrap();
        // Should be parseable as a UUID
        let parsed = Uuid::parse_str(&output.uuid);
        assert!(parsed.is_ok());
    }
}
