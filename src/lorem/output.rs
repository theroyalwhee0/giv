use crate::output::Output;
#[cfg(feature = "json")]
use serde::Serialize;

/// The output from the lorem command.
#[cfg_attr(feature = "json", derive(Serialize))]
#[derive(Debug)]
pub struct LoremOutput {
    /// The generated lorem ipsum text.
    pub lorem: String,
}

impl Output for LoremOutput {
    fn to_plain(&self) -> String {
        self.lorem.clone()
    }

    #[cfg(feature = "json")]
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test plain text output formatting.
    ///
    /// Verifies that `LoremOutput::to_plain()` returns the lorem ipsum text
    /// as a plain string without any formatting or decoration.
    #[test]
    fn test_to_plain() {
        let output = LoremOutput {
            lorem: "Lorem ipsum dolor sit amet.".to_string(),
        };
        assert_eq!(output.to_plain(), "Lorem ipsum dolor sit amet.");
    }

    /// Test JSON output formatting.
    ///
    /// Verifies that `LoremOutput::to_json()` produces valid JSON with
    /// the lorem ipsum text in a "lorem" field, matching the project's
    /// output format conventions.
    #[test]
    #[cfg(feature = "json")]
    fn test_to_json() {
        let output = LoremOutput {
            lorem: "Lorem ipsum dolor sit amet.".to_string(),
        };
        let json = output.to_json();
        assert_eq!(json["lorem"], "Lorem ipsum dolor sit amet.");
    }
}
