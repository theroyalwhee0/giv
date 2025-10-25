use crate::output::Output;
#[cfg(feature = "json")]
use serde::Serialize;

/// The output from the key command.
#[cfg_attr(feature = "json", derive(Serialize))]
#[derive(Debug)]
pub struct KeyOutput {
    /// The generated key
    pub key: String,
}

impl Output for KeyOutput {
    fn to_plain(&self) -> String {
        self.key.clone()
    }

    #[cfg(feature = "json")]
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test plain text output formatting for key generation.
    ///
    /// Verifies that `KeyOutput::to_plain()` returns the generated key
    /// as a plain string without any additional formatting.
    #[test]
    fn test_to_plain() {
        let output = KeyOutput {
            key: "key_abc123XYZ".to_string(),
        };
        assert_eq!(output.to_plain(), "key_abc123XYZ");
    }

    /// Test JSON output formatting for key generation.
    ///
    /// Verifies that `KeyOutput::to_json()` produces valid JSON with
    /// the generated key in a "key" field, matching the project's
    /// output format conventions.
    #[test]
    #[cfg(feature = "json")]
    fn test_to_json() {
        let output = KeyOutput {
            key: "key_abc123XYZ".to_string(),
        };
        let json = output.to_json();
        assert_eq!(json["key"], "key_abc123XYZ");
    }
}
