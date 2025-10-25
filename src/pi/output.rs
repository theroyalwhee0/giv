use crate::output::Output;
#[cfg(feature = "json")]
use serde::Serialize;

/// The output from the pi command.
#[cfg_attr(feature = "json", derive(Serialize))]
#[derive(Debug)]
pub struct PiOutput {
    /// The pi value
    pub pi: String,
    /// Whether rounding was applied
    pub rounded: bool,
}

impl Output for PiOutput {
    fn to_plain(&self) -> String {
        self.pi.clone()
    }

    #[cfg(feature = "json")]
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test plain text output formatting for pi value.
    ///
    /// Verifies that `PiOutput::to_plain()` returns only the pi value
    /// without metadata about rounding, suitable for command-line piping
    /// and numerical processing.
    #[test]
    fn test_to_plain() {
        let output = PiOutput {
            pi: "3.14159".to_string(),
            rounded: true,
        };
        assert_eq!(output.to_plain(), "3.14159");
    }

    /// Test JSON output formatting for pi value.
    ///
    /// Verifies that `PiOutput::to_json()` produces valid JSON with both
    /// the pi value and rounding status, providing complete information
    /// about how the value was computed.
    #[test]
    #[cfg(feature = "json")]
    fn test_to_json() {
        let output = PiOutput {
            pi: "3.14159".to_string(),
            rounded: true,
        };
        let json = output.to_json();
        assert_eq!(json["pi"], "3.14159");
        assert_eq!(json["rounded"], true);
    }

    /// Test JSON output with non-rounded pi value.
    ///
    /// Verifies that the rounded field correctly reflects when no
    /// rounding was applied to the pi calculation.
    #[test]
    #[cfg(feature = "json")]
    fn test_to_json_not_rounded() {
        let output = PiOutput {
            pi: "3.141592653589793".to_string(),
            rounded: false,
        };
        let json = output.to_json();
        assert_eq!(json["pi"], "3.141592653589793");
        assert_eq!(json["rounded"], false);
    }
}
