use crate::output::Output;
#[cfg(feature = "json")]
use serde::Serialize;

/// The output from the uuid command.
#[cfg_attr(feature = "json", derive(Serialize))]
#[derive(Debug)]
pub struct UuidOutput {
    /// The generated UUID
    pub uuid: String,
    /// The UUID version
    pub version: String,
}

impl Output for UuidOutput {
    fn to_plain(&self) -> String {
        self.uuid.clone()
    }

    #[cfg(feature = "json")]
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test plain text output formatting for UUID generation.
    ///
    /// Verifies that `UuidOutput::to_plain()` returns only the UUID string
    /// without the version information, suitable for command-line piping.
    #[test]
    fn test_to_plain() {
        let output = UuidOutput {
            uuid: "01234567-89ab-7def-0123-456789abcdef".to_string(),
            version: "v7".to_string(),
        };
        assert_eq!(output.to_plain(), "01234567-89ab-7def-0123-456789abcdef");
    }

    /// Test JSON output formatting for UUID generation.
    ///
    /// Verifies that `UuidOutput::to_json()` produces valid JSON with both
    /// the UUID and version fields, providing complete information in
    /// structured format.
    #[test]
    #[cfg(feature = "json")]
    fn test_to_json() {
        let output = UuidOutput {
            uuid: "01234567-89ab-7def-0123-456789abcdef".to_string(),
            version: "v7".to_string(),
        };
        let json = output.to_json();
        assert_eq!(json["uuid"], "01234567-89ab-7def-0123-456789abcdef");
        assert_eq!(json["version"], "v7");
    }
}
