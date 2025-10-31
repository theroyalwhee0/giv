use clap::ValueEnum;
#[cfg(feature = "json")]
use serde::Serialize;

/// The UUID version to generate.
#[cfg_attr(feature = "json", derive(Serialize))]
#[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum UuidVersion {
    /// UUID version 4 (random).
    #[value(name = "v4")]
    V4,
    /// UUID version 7 (timestamp-based, sortable).
    #[value(name = "v7")]
    V7,
}

impl UuidVersion {
    /// Get the default UUID version.
    ///
    /// # Returns
    ///
    /// The default UUID version (V7).
    pub const fn default() -> Self {
        Self::V7
    }

    /// Get the version string representation.
    ///
    /// # Returns
    ///
    /// A string representation of the UUID version (e.g., "v4", "v7").
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::V4 => "v4",
            Self::V7 => "v7",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the default UUID version.
    ///
    /// Verifies that `UuidVersion::default()` returns V7 as the
    /// default UUID version.
    #[test]
    fn test_default_version() {
        assert_eq!(UuidVersion::default(), UuidVersion::V7);
    }

    /// Test enum variants exist and are distinct.
    ///
    /// Verifies that all UUID version variants can be created and
    /// are properly distinguished from each other.
    #[test]
    fn test_version_variants() {
        let variants = [UuidVersion::V4, UuidVersion::V7];

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

    /// Test version string conversion.
    ///
    /// Verifies that `as_str()` returns the correct string representation
    /// for each UUID version.
    #[test]
    fn test_as_str() {
        assert_eq!(UuidVersion::V4.as_str(), "v4");
        assert_eq!(UuidVersion::V7.as_str(), "v7");
    }

    /// Test JSON serialization of version variants.
    ///
    /// Verifies that `UuidVersion` variants serialize correctly to
    /// lowercase strings in JSON format.
    #[test]
    #[cfg(feature = "json")]
    fn test_json_serialization() {
        assert_eq!(serde_json::to_string(&UuidVersion::V4).unwrap(), "\"v4\"");
        assert_eq!(serde_json::to_string(&UuidVersion::V7).unwrap(), "\"v7\"");
    }
}
