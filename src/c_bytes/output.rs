use super::BytesEncoding;
use crate::app::output::Output;
use base64::{engine::general_purpose, Engine};
use serde::Serialize;

/// The output from the bytes command.
#[derive(Debug, Serialize)]
pub struct BytesOutput {
    /// The encoded bytes as a string (hex, base64, or raw).
    pub bytes: String,
    /// The encoding format used.
    pub encoding: BytesEncoding,
    /// The number of bytes generated.
    pub length: usize,
    /// Whether padding is used (base64 only).
    #[serde(skip_serializing_if = "Option::is_none")]
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
