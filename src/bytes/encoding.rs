use clap::ValueEnum;
#[cfg(feature = "json")]
use serde::Serialize;

/// The encoding format for bytes output.
#[cfg_attr(feature = "json", derive(Serialize))]
#[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum BytesEncoding {
    /// Hexadecimal encoding (lowercase).
    Hex,
    /// Base64 encoding.
    Base64,
    /// Raw bytes (plain text only, errors in JSON mode).
    Raw,
    /// Rust array literal format: `[u8; N] = [0x00, 0x01, ...]`.
    Rust,
    /// JavaScript array literal format: `[0x00, 0x01, ...]`.
    #[value(name = "javascript")]
    JavaScript,
    /// TypeScript array literal with type annotation: `number[] = [0x00, 0x01, ...]`.
    #[value(name = "typescript")]
    TypeScript,
}

impl BytesEncoding {
    /// Get the default encoding format.
    ///
    /// # Returns
    ///
    /// The default encoding format (Base64).
    pub const fn default() -> Self {
        Self::Base64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the default encoding format.
    ///
    /// Verifies that `BytesEncoding::default()` returns Base64 as the
    /// default encoding format for bytes output.
    #[test]
    fn test_default_encoding() {
        assert_eq!(BytesEncoding::default(), BytesEncoding::Base64);
    }

    /// Test enum variants exist and are distinct.
    ///
    /// Verifies that all encoding format variants can be created and
    /// are properly distinguished from each other.
    #[test]
    fn test_encoding_variants() {
        let variants = [
            BytesEncoding::Hex,
            BytesEncoding::Base64,
            BytesEncoding::Raw,
            BytesEncoding::Rust,
            BytesEncoding::JavaScript,
            BytesEncoding::TypeScript,
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

    /// Test JSON serialization of encoding variants.
    ///
    /// Verifies that `BytesEncoding` variants serialize correctly to
    /// lowercase strings in JSON format.
    #[test]
    #[cfg(feature = "json")]
    fn test_json_serialization() {
        assert_eq!(
            serde_json::to_string(&BytesEncoding::Hex).unwrap(),
            "\"hex\""
        );
        assert_eq!(
            serde_json::to_string(&BytesEncoding::Base64).unwrap(),
            "\"base64\""
        );
        assert_eq!(
            serde_json::to_string(&BytesEncoding::Raw).unwrap(),
            "\"raw\""
        );
    }
}
