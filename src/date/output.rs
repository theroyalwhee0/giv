use crate::output::Output;
#[cfg(feature = "json")]
use serde::Serialize;

/// The output from the date command.
#[cfg_attr(feature = "json", derive(Serialize))]
#[derive(Debug)]
pub struct DateOutput {
    /// The formatted date
    pub date: String,
}

impl Output for DateOutput {
    fn to_plain(&self) -> String {
        self.date.clone()
    }

    #[cfg(feature = "json")]
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test plain text output formatting for date values.
    ///
    /// Verifies that `DateOutput::to_plain()` returns the formatted date
    /// as a plain string, suitable for command-line piping and scripting.
    #[test]
    fn test_to_plain() {
        let output = DateOutput {
            date: "2025-01-15T10:30:00Z".to_string(),
        };
        assert_eq!(output.to_plain(), "2025-01-15T10:30:00Z");
    }

    /// Test JSON output formatting for date values.
    ///
    /// Verifies that `DateOutput::to_json()` produces valid JSON with
    /// the formatted date in a "date" field, matching the project's
    /// output format conventions.
    #[test]
    #[cfg(feature = "json")]
    fn test_to_json() {
        let output = DateOutput {
            date: "2025-01-15T10:30:00Z".to_string(),
        };
        let json = output.to_json();
        assert_eq!(json["date"], "2025-01-15T10:30:00Z");
    }
}
