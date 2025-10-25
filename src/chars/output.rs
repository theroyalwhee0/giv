use crate::output::Output;
#[cfg(feature = "json")]
use serde::Serialize;

/// A single character conversion result.
#[cfg_attr(feature = "json", derive(Serialize))]
#[derive(Debug)]
pub struct CharResult {
    /// The input pattern or shortcode.
    pub input: String,
    /// The output character or emoji.
    pub output: String,
    /// The type of conversion (emoji, pattern, or passthrough).
    #[cfg_attr(feature = "json", serde(rename = "type"))]
    pub result_type: String,
    /// Optional name/description of the character.
    #[cfg_attr(feature = "json", serde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<String>,
}

/// The output from the chars command.
#[cfg_attr(feature = "json", derive(Serialize))]
#[derive(Debug)]
pub struct CharsOutput {
    /// The list of conversion results.
    pub results: Vec<CharResult>,
}

impl Output for CharsOutput {
    fn to_plain(&self) -> String {
        self.results
            .iter()
            .map(|r| r.output.as_str())
            .collect::<Vec<_>>()
            .join(" ")
    }

    #[cfg(feature = "json")]
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!(self.results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test plain text output formatting for character conversions.
    ///
    /// Verifies that `CharsOutput::to_plain()` returns only the output
    /// characters separated by spaces, without metadata, suitable for
    /// command-line piping.
    #[test]
    fn test_to_plain() {
        let output = CharsOutput {
            results: vec![
                CharResult {
                    input: ":smile:".to_string(),
                    output: "😀".to_string(),
                    result_type: "emoji".to_string(),
                    name: Some("grinning face".to_string()),
                },
                CharResult {
                    input: "->".to_string(),
                    output: "→".to_string(),
                    result_type: "pattern".to_string(),
                    name: Some("rightwards arrow".to_string()),
                },
            ],
        };
        assert_eq!(output.to_plain(), "😀 →");
    }

    /// Test JSON output formatting for character conversions.
    ///
    /// Verifies that `CharsOutput::to_json()` produces a JSON array with
    /// complete conversion information including input, output, type, and
    /// optional name fields.
    #[test]
    #[cfg(feature = "json")]
    fn test_to_json() {
        let output = CharsOutput {
            results: vec![
                CharResult {
                    input: ":smile:".to_string(),
                    output: "😀".to_string(),
                    result_type: "emoji".to_string(),
                    name: Some("grinning face".to_string()),
                },
                CharResult {
                    input: "->".to_string(),
                    output: "→".to_string(),
                    result_type: "pattern".to_string(),
                    name: None,
                },
            ],
        };
        let json = output.to_json();
        assert!(json.is_array());
        assert_eq!(json[0]["input"], ":smile:");
        assert_eq!(json[0]["output"], "😀");
        assert_eq!(json[0]["type"], "emoji");
        assert_eq!(json[0]["name"], "grinning face");
        assert_eq!(json[1]["input"], "->");
        assert_eq!(json[1]["output"], "→");
        assert!(json[1]["name"].is_null());
    }
}
