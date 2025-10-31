use crate::error::GivError;
use crate::uuid::format::UuidFormat;
use crate::uuid::output::UuidOutput;
use crate::uuid::version::UuidVersion;
use uuid::Uuid;

/// Generate a UUID with specified options.
///
/// # Arguments
///
/// * `version` - Optional UUID version (defaults to v7)
/// * `format` - Optional formatting style (defaults to standard)
/// * `uppercase` - Whether to use uppercase hex digits (defaults to false)
///
/// # Returns
///
/// Returns a [`UuidOutput`] containing the generated UUID with formatting information.
///
/// # Errors
///
/// This function does not currently return errors, but returns a Result for consistency.
///
/// # Examples
///
/// ```
/// use giv::uuid::{generate_uuid, UuidVersion, UuidFormat};
/// use giv::GivError;
///
/// # fn main() -> Result<(), GivError> {
/// // Generate a default UUID (v7, standard format, lowercase)
/// let uuid = generate_uuid(None, None, false)?;
/// assert_eq!(uuid.version, "v7");
///
/// // Generate a v4 UUID
/// let uuid_v4 = generate_uuid(Some(UuidVersion::V4), None, false)?;
/// assert_eq!(uuid_v4.version, "v4");
///
/// // Generate a simple format UUID (no dashes)
/// let uuid_simple = generate_uuid(None, Some(UuidFormat::Simple), false)?;
/// # Ok(())
/// # }
/// ```
pub fn generate_uuid(
    version: Option<UuidVersion>,
    format: Option<UuidFormat>,
    uppercase: bool,
) -> Result<UuidOutput, GivError> {
    // Get version, defaulting to v7
    let version = version.unwrap_or_else(UuidVersion::default);

    // Generate UUID based on version
    let uuid = match version {
        UuidVersion::V4 => Uuid::new_v4(),
        UuidVersion::V7 => Uuid::now_v7(),
    };

    // Get format, defaulting to standard
    let format = format.unwrap_or_else(UuidFormat::default);

    // Format the UUID according to the specified format
    let formatted = format_uuid(&uuid, format, uppercase);

    // Create output with the UUID
    Ok(UuidOutput {
        uuid: formatted,
        version: version.as_str().to_string(),
        format: format.as_str().to_string(),
        uppercase,
    })
}

/// Format a UUID according to the specified format and case.
///
/// # Arguments
///
/// * `uuid` - The UUID to format
/// * `format` - The formatting style to use
/// * `uppercase` - Whether to use uppercase hex digits
///
/// # Returns
///
/// A formatted string representation of the UUID.
fn format_uuid(uuid: &Uuid, format: UuidFormat, uppercase: bool) -> String {
    let base = match format {
        UuidFormat::Standard => uuid.hyphenated().to_string(),
        UuidFormat::Simple => uuid.simple().to_string(),
        UuidFormat::Braced => uuid.braced().to_string(),
        UuidFormat::URN => uuid.urn().to_string(),
        UuidFormat::Hex => format!("0x{}", uuid.simple()),
    };

    if uppercase { base.to_uppercase() } else { base }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test UUID generation with default parameters.
    ///
    /// Verifies that `generate_uuid()` successfully generates UUIDs with
    /// default settings (v7, standard format, lowercase).
    #[test]
    fn test_generate_uuid_default() {
        let output = generate_uuid(None, None, false).unwrap();
        // UUID should be 36 characters (32 hex + 4 hyphens)
        assert_eq!(output.uuid.len(), 36);
        // Should have hyphens in the right places
        assert_eq!(output.uuid.chars().nth(8).unwrap(), '-');
        assert_eq!(output.uuid.chars().nth(13).unwrap(), '-');
        assert_eq!(output.uuid.chars().nth(18).unwrap(), '-');
        assert_eq!(output.uuid.chars().nth(23).unwrap(), '-');
        // Version should be v7
        assert_eq!(output.version, "v7");
        // Format should be standard
        assert_eq!(output.format, "standard");
        // Should be lowercase
        assert!(!output.uppercase);
    }

    /// Test UUID v4 generation.
    ///
    /// Verifies that v4 UUIDs can be generated.
    #[test]
    fn test_generate_uuid_v4() {
        let output = generate_uuid(Some(UuidVersion::V4), None, false).unwrap();
        assert_eq!(output.version, "v4");
        assert_eq!(output.uuid.len(), 36);
    }

    /// Test UUID generation produces unique values.
    ///
    /// Verifies that calling `generate_uuid()` multiple times produces
    /// different UUIDs.
    #[test]
    fn test_uuid_uniqueness() {
        let uuid1 = generate_uuid(None, None, false).unwrap();
        let uuid2 = generate_uuid(None, None, false).unwrap();
        // Two generated UUIDs should be different
        assert_ne!(uuid1.uuid, uuid2.uuid);
    }

    /// Test UUID simple format (no dashes).
    ///
    /// Verifies that simple format produces a 32-character string
    /// without dashes.
    #[test]
    fn test_uuid_simple_format() {
        let output = generate_uuid(None, Some(UuidFormat::Simple), false).unwrap();
        assert_eq!(output.uuid.len(), 32);
        assert!(!output.uuid.contains('-'));
        assert_eq!(output.format, "simple");
    }

    /// Test UUID braced format.
    ///
    /// Verifies that braced format includes braces around the UUID.
    #[test]
    fn test_uuid_braced_format() {
        let output = generate_uuid(None, Some(UuidFormat::Braced), false).unwrap();
        assert_eq!(output.uuid.len(), 38); // 32 hex + 4 hyphens + 2 braces
        assert!(output.uuid.starts_with('{'));
        assert!(output.uuid.ends_with('}'));
        assert_eq!(output.format, "braced");
    }

    /// Test UUID URN format.
    ///
    /// Verifies that URN format includes the "urn:uuid:" prefix.
    #[test]
    fn test_uuid_urn_format() {
        let output = generate_uuid(None, Some(UuidFormat::URN), false).unwrap();
        assert!(output.uuid.starts_with("urn:uuid:"));
        assert_eq!(output.format, "urn");
    }

    /// Test UUID hex format.
    ///
    /// Verifies that hex format includes the "0x" prefix and no dashes.
    #[test]
    fn test_uuid_hex_format() {
        let output = generate_uuid(None, Some(UuidFormat::Hex), false).unwrap();
        assert!(output.uuid.starts_with("0x"));
        assert_eq!(output.uuid.len(), 34); // "0x" + 32 hex digits
        assert_eq!(output.format, "hex");
    }

    /// Test UUID uppercase formatting.
    ///
    /// Verifies that uppercase flag produces uppercase hex digits.
    #[test]
    fn test_uuid_uppercase() {
        let output = generate_uuid(None, None, true).unwrap();
        assert!(output.uppercase);
        // Should contain at least some uppercase hex digits (A-F)
        // We can't guarantee all will be uppercase since it depends on the UUID value
        assert!(output.uuid.chars().any(|c| c.is_ascii_uppercase()));
    }

    /// Test UUID format is parseable by uuid crate.
    ///
    /// Verifies that standard format UUIDs can be successfully parsed.
    #[test]
    fn test_uuid_format_parseable() {
        let output = generate_uuid(None, Some(UuidFormat::Standard), false).unwrap();
        // Should be parseable as a UUID
        let parsed = Uuid::parse_str(&output.uuid);
        assert!(parsed.is_ok());
    }

    /// Test combined options: v4 + simple + uppercase.
    ///
    /// Verifies that multiple options can be combined correctly.
    #[test]
    fn test_uuid_combined_options() {
        let output = generate_uuid(Some(UuidVersion::V4), Some(UuidFormat::Simple), true).unwrap();
        assert_eq!(output.version, "v4");
        assert_eq!(output.format, "simple");
        assert!(output.uppercase);
        assert_eq!(output.uuid.len(), 32);
        assert!(!output.uuid.contains('-'));
    }
}
