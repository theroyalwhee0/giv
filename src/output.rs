/// Trait for types that can be output in both plain and JSON formats.
///
/// Commands should implement this trait for their output types to support
/// both plain text and JSON output modes.
pub trait Output {
    /// Generate plain text representation of the value.
    ///
    /// This is used when the user has not specified the `--json` flag.
    ///
    /// # Returns
    ///
    /// A string representation suitable for human reading.
    fn to_plain(&self) -> String;

    /// Generate JSON representation of the value.
    ///
    /// This is used when the user has specified the `--json` flag.
    ///
    /// # Returns
    ///
    /// A `serde_json::Value` that will be serialized to JSON.
    #[cfg(feature = "json")]
    fn to_json(&self) -> serde_json::Value;
}

/// Default implementation of `Output` for `String`.
impl Output for String {
    fn to_plain(&self) -> String {
        self.clone()
    }

    #[cfg(feature = "json")]
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!(self)
    }
}

/// Default implementation of `Output` for `&str`.
impl Output for &str {
    fn to_plain(&self) -> String {
        (*self).to_string()
    }

    #[cfg(feature = "json")]
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test plain text output for String type.
    ///
    /// Verifies that the `Output` implementation for `String` correctly
    /// returns the string value unchanged in plain text mode.
    #[test]
    fn test_string_to_plain() {
        let value = "test value".to_string();
        assert_eq!(value.to_plain(), "test value");
    }

    /// Test JSON output for String type.
    ///
    /// Verifies that the `Output` implementation for `String` correctly
    /// serializes the string as a JSON string value.
    #[test]
    #[cfg(feature = "json")]
    fn test_string_to_json() {
        let value = "test value".to_string();
        let json = value.to_json();
        assert_eq!(json, "test value");
    }

    /// Test plain text output for &str type.
    ///
    /// Verifies that the `Output` implementation for `&str` correctly
    /// converts the string slice to an owned String.
    #[test]
    fn test_str_to_plain() {
        let value = "test value";
        assert_eq!(value.to_plain(), "test value");
    }

    /// Test JSON output for &str type.
    ///
    /// Verifies that the `Output` implementation for `&str` correctly
    /// serializes the string slice as a JSON string value.
    #[test]
    #[cfg(feature = "json")]
    fn test_str_to_json() {
        let value = "test value";
        let json = value.to_json();
        assert_eq!(json, "test value");
    }
}
