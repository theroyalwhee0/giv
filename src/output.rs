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
