use super::BytesEncoding;
use crate::output::Output;
use base64::{Engine, engine::general_purpose};
#[cfg(feature = "json")]
use serde::Serialize;

/// The output from the bytes command.
#[cfg_attr(feature = "json", derive(Serialize))]
#[derive(Debug)]
pub struct BytesOutput {
    /// The encoded bytes as a string (hex, base64, or raw).
    pub bytes: String,
    /// The encoding format used.
    pub encoding: BytesEncoding,
    /// The number of bytes generated.
    pub length: usize,
    /// Whether padding is used (base64 only).
    #[cfg_attr(feature = "json", serde(skip_serializing_if = "Option::is_none"))]
    pub padding: Option<bool>,
}

impl BytesOutput {
    /// Create a new bytes output from raw bytes.
    ///
    /// # Arguments
    ///
    /// - `raw_bytes` The raw bytes to encode.
    /// - `encoding` The encoding format to use.
    /// - `padding` Whether to use padding for base64 encoding.
    ///
    /// # Returns
    ///
    /// A new `BytesOutput` instance with the bytes encoded according to the format.
    pub fn new(raw_bytes: &[u8], encoding: BytesEncoding, padding: bool) -> Self {
        let bytes = match encoding {
            BytesEncoding::Hex => hex::encode(raw_bytes),
            BytesEncoding::Base64 => {
                if padding {
                    general_purpose::STANDARD.encode(raw_bytes)
                } else {
                    general_purpose::STANDARD_NO_PAD.encode(raw_bytes)
                }
            }
            BytesEncoding::Raw => {
                // SAFETY: This is safe because we convert bytes to string using lossy conversion.
                // Invalid UTF-8 sequences will be replaced with the replacement character.
                // This may produce unexpected output, but that's documented as acceptable.
                String::from_utf8_lossy(raw_bytes).to_string()
            }
            BytesEncoding::Rust => format_rust_array(raw_bytes),
            BytesEncoding::JavaScript => format_js_array(raw_bytes),
            BytesEncoding::TypeScript => format_ts_array(raw_bytes),
        };

        Self {
            bytes,
            encoding,
            length: raw_bytes.len(),
            padding: match encoding {
                BytesEncoding::Base64 => Some(padding),
                _ => None,
            },
        }
    }
}

/// Format bytes as a Rust array literal.
///
/// # Arguments
///
/// - `bytes` The bytes to format.
///
/// # Returns
///
/// A string in the format `[u8; N] = [0x00, 0x01, ...]`.
fn format_rust_array(bytes: &[u8]) -> String {
    let length = bytes.len();
    let formatted_length = format_number_with_underscores(length);
    let array_content = format_byte_array(bytes);
    format!("[u8; {formatted_length}] = [{array_content}]")
}

/// Format bytes as a JavaScript array literal.
///
/// # Arguments
///
/// - `bytes` The bytes to format.
///
/// # Returns
///
/// A string in the format `[0x00, 0x01, ...]`.
fn format_js_array(bytes: &[u8]) -> String {
    let array_content = format_byte_array(bytes);
    format!("[{array_content}]")
}

/// Format bytes as a TypeScript array literal with type annotation.
///
/// # Arguments
///
/// - `bytes` The bytes to format.
///
/// # Returns
///
/// A string in the format `number[] = [0x00, 0x01, ...]`.
fn format_ts_array(bytes: &[u8]) -> String {
    let array_content = format_byte_array(bytes);
    format!("number[] = [{array_content}]")
}

/// Format bytes as a comma-separated hex array.
///
/// # Arguments
///
/// - `bytes` The bytes to format.
///
/// # Returns
///
/// A string in the format `0x00, 0x01, 0x02, ...`.
fn format_byte_array(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("0x{b:02x}"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Format a number with underscores for readability.
///
/// # Arguments
///
/// - `num` The number to format.
///
/// # Returns
///
/// A string with underscores every 3 digits from the right.
fn format_number_with_underscores(num: usize) -> String {
    let num_str = num.to_string();
    let mut result = String::new();
    let mut count = 0;

    for (i, ch) in num_str.chars().rev().enumerate() {
        if i > 0 && count == 3 {
            result.push('_');
            count = 0;
        }
        result.push(ch);
        count += 1;
    }

    result.chars().rev().collect()
}

impl Output for BytesOutput {
    fn to_plain(&self) -> String {
        self.bytes.clone()
    }

    #[cfg(feature = "json")]
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test hex encoding of bytes.
    ///
    /// Verifies that `BytesOutput::new()` correctly encodes raw bytes
    /// as hexadecimal representation.
    #[test]
    fn test_hex_encoding() {
        let raw_bytes = vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
        let output = BytesOutput::new(&raw_bytes, BytesEncoding::Hex, false);
        assert_eq!(output.bytes, "0123456789abcdef");
        assert_eq!(output.encoding, BytesEncoding::Hex);
        assert_eq!(output.length, 8);
        assert!(output.padding.is_none());
    }

    /// Test base64 encoding with padding.
    ///
    /// Verifies that `BytesOutput::new()` correctly encodes bytes as
    /// base64 with padding characters when requested.
    #[test]
    fn test_base64_with_padding() {
        let raw_bytes = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello"
        let output = BytesOutput::new(&raw_bytes, BytesEncoding::Base64, true);
        assert_eq!(output.bytes, "SGVsbG8=");
        assert_eq!(output.encoding, BytesEncoding::Base64);
        assert_eq!(output.length, 5);
        assert_eq!(output.padding, Some(true));
    }

    /// Test base64 encoding without padding.
    ///
    /// Verifies that `BytesOutput::new()` correctly encodes bytes as
    /// base64 without padding characters when requested.
    #[test]
    fn test_base64_without_padding() {
        let raw_bytes = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello"
        let output = BytesOutput::new(&raw_bytes, BytesEncoding::Base64, false);
        assert_eq!(output.bytes, "SGVsbG8");
        assert_eq!(output.encoding, BytesEncoding::Base64);
        assert_eq!(output.padding, Some(false));
    }

    /// Test Rust array formatting.
    ///
    /// Verifies that `BytesOutput::new()` correctly formats bytes as
    /// a Rust array literal with type annotation and hex values.
    #[test]
    fn test_rust_array_formatting() {
        let raw_bytes = vec![0x01, 0x02, 0x03];
        let output = BytesOutput::new(&raw_bytes, BytesEncoding::Rust, false);
        assert_eq!(output.bytes, "[u8; 3] = [0x01, 0x02, 0x03]");
        assert!(output.padding.is_none());
    }

    /// Test JavaScript array formatting.
    ///
    /// Verifies that `BytesOutput::new()` correctly formats bytes as
    /// a JavaScript array literal.
    #[test]
    fn test_javascript_array_formatting() {
        let raw_bytes = vec![0xaa, 0xbb, 0xcc];
        let output = BytesOutput::new(&raw_bytes, BytesEncoding::JavaScript, false);
        assert_eq!(output.bytes, "[0xaa, 0xbb, 0xcc]");
    }

    /// Test TypeScript array formatting.
    ///
    /// Verifies that `BytesOutput::new()` correctly formats bytes as
    /// a TypeScript array literal with type annotation.
    #[test]
    fn test_typescript_array_formatting() {
        let raw_bytes = vec![0x11, 0x22];
        let output = BytesOutput::new(&raw_bytes, BytesEncoding::TypeScript, false);
        assert_eq!(output.bytes, "number[] = [0x11, 0x22]");
    }

    /// Test number formatting with underscores.
    ///
    /// Verifies that `format_number_with_underscores()` correctly
    /// inserts underscores every 3 digits for readability.
    #[test]
    fn test_format_number_with_underscores() {
        assert_eq!(format_number_with_underscores(123), "123");
        assert_eq!(format_number_with_underscores(1234), "1_234");
        assert_eq!(format_number_with_underscores(1234567), "1_234_567");
        assert_eq!(format_number_with_underscores(1000000), "1_000_000");
    }

    /// Test plain text output.
    ///
    /// Verifies that `BytesOutput::to_plain()` returns the encoded
    /// bytes string directly, suitable for command-line piping.
    #[test]
    fn test_to_plain() {
        let raw_bytes = vec![0x01, 0x02, 0x03];
        let output = BytesOutput::new(&raw_bytes, BytesEncoding::Hex, false);
        assert_eq!(output.to_plain(), "010203");
    }

    /// Test JSON output.
    ///
    /// Verifies that `BytesOutput::to_json()` produces complete JSON
    /// with bytes, encoding, length, and optional padding information.
    #[test]
    #[cfg(feature = "json")]
    fn test_to_json() {
        let raw_bytes = vec![0xab, 0xcd];
        let output = BytesOutput::new(&raw_bytes, BytesEncoding::Hex, false);
        let json = output.to_json();
        assert_eq!(json["bytes"], "abcd");
        assert_eq!(json["encoding"], "hex");
        assert_eq!(json["length"], 2);
    }

    /// Test JSON output with base64 padding information.
    ///
    /// Verifies that the padding field is included in JSON output
    /// when using base64 encoding.
    #[test]
    #[cfg(feature = "json")]
    fn test_to_json_with_padding() {
        let raw_bytes = vec![0x48, 0x65];
        let output = BytesOutput::new(&raw_bytes, BytesEncoding::Base64, true);
        let json = output.to_json();
        assert_eq!(json["padding"], true);
    }
}
