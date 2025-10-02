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
