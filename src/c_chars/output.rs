use crate::app::output::Output;
use serde::Serialize;

/// A single character conversion result.
#[derive(Debug, Serialize)]
pub struct CharResult {
    /// The input pattern or shortcode.
    pub input: String,
    /// The output character or emoji.
    pub output: String,
    /// The type of conversion (emoji, pattern, or passthrough).
    #[serde(rename = "type")]
    pub result_type: String,
    /// Optional name/description of the character.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// The output from the chars command.
#[derive(Debug, Serialize)]
pub struct CharsOutput {
    /// The list of conversion results.
    pub results: Vec<CharResult>,
}

impl CharsOutput {
    /// Create a new chars output.
    ///
    /// # Arguments
    ///
    /// - `results` The list of conversion results.
    ///
    /// # Returns
    ///
    /// A new `CharsOutput` instance.
    pub fn new(results: Vec<CharResult>) -> Self {
        Self { results }
    }
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
