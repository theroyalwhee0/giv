use clap::ValueEnum;
#[cfg(feature = "json")]
use serde::Serialize;

/// The formatting style for UUID output.
#[cfg_attr(feature = "json", derive(Serialize))]
#[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum UuidFormat {
    /// Standard UUID format with dashes: `550e8400-e29b-41d4-a716-446655440000`
    Standard,
    /// Simple format without dashes: `550e8400e29b41d4a716446655440000`
    Simple,
    /// Braced format (Microsoft GUID style): `{550e8400-e29b-41d4-a716-446655440000}`
    Braced,
    /// URN format: `urn:uuid:550e8400-e29b-41d4-a716-446655440000`
    #[value(name = "urn")]
    URN,
    /// Hexadecimal literal format: `0x550e8400e29b41d4a716446655440000`
    Hex,
}

impl UuidFormat {
    /// Get the default UUID format.
    ///
    /// # Returns
    ///
    /// The default UUID format (Standard).
    pub const fn default() -> Self {
        Self::Standard
    }

    /// Get the format string representation.
    ///
    /// # Returns
    ///
    /// A string representation of the UUID format.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Simple => "simple",
            Self::Braced => "braced",
            Self::URN => "urn",
            Self::Hex => "hex",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the default UUID format.
    ///
    /// Verifies that `UuidFormat::default()` returns Standard as the
    /// default UUID format.
    #[test]
    fn test_default_format() {
        assert_eq!(UuidFormat::default(), UuidFormat::Standard);
    }

    /// Test enum variants exist and are distinct.
    ///
    /// Verifies that all UUID format variants can be created and
    /// are properly distinguished from each other.
    #[test]
    fn test_format_variants() {
        let variants = [
            UuidFormat::Standard,
            UuidFormat::Simple,
            UuidFormat::Braced,
            UuidFormat::URN,
            UuidFormat::Hex,
        ];

        // Verify all variants are distinct
        for (i, v1) in variants.iter().enumerate() {
            for (j, v2) in variants.iter().enumerate() {
                if i == j {
                    assert_eq!(v1, v2);
                } else {
                    assert_ne!(v1, v2);
                }
            }
        }
    }

    /// Test format string conversion.
    ///
    /// Verifies that `as_str()` returns the correct string representation
    /// for each UUID format.
    #[test]
    fn test_as_str() {
        assert_eq!(UuidFormat::Standard.as_str(), "standard");
        assert_eq!(UuidFormat::Simple.as_str(), "simple");
        assert_eq!(UuidFormat::Braced.as_str(), "braced");
        assert_eq!(UuidFormat::URN.as_str(), "urn");
        assert_eq!(UuidFormat::Hex.as_str(), "hex");
    }

    /// Test JSON serialization of format variants.
    ///
    /// Verifies that `UuidFormat` variants serialize correctly to
    /// lowercase strings in JSON format.
    #[test]
    #[cfg(feature = "json")]
    fn test_json_serialization() {
        assert_eq!(
            serde_json::to_string(&UuidFormat::Standard).unwrap(),
            "\"standard\""
        );
        assert_eq!(
            serde_json::to_string(&UuidFormat::Simple).unwrap(),
            "\"simple\""
        );
        assert_eq!(
            serde_json::to_string(&UuidFormat::Braced).unwrap(),
            "\"braced\""
        );
        assert_eq!(serde_json::to_string(&UuidFormat::URN).unwrap(), "\"urn\"");
        assert_eq!(serde_json::to_string(&UuidFormat::Hex).unwrap(), "\"hex\"");
    }
}
