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
