use crate::output::Output;
#[cfg(feature = "json")]
use serde::Serialize;

use super::result::RngResult;

/// The output from the RNG command.
#[cfg_attr(feature = "json", derive(Serialize))]
#[derive(Debug)]
pub struct RngOutput {
    /// List of results from each specification
    pub rng: Vec<RngResult>,
}

impl Output for RngOutput {
    fn to_plain(&self) -> String {
        self.rng
            .iter()
            .map(|result| match result {
                RngResult::Dice { value, .. } => value.to_string(),
                RngResult::RangeInt { value, .. } => value.to_string(),
                RngResult::RangeFloat { value, .. } => value.clone(),
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[cfg(feature = "json")]
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test plain text output formatting for RNG dice results.
    ///
    /// Verifies that `RngOutput::to_plain()` returns only the numeric
    /// values separated by newlines, one per specification, without
    /// metadata about the specifications themselves.
    #[test]
    fn test_to_plain_dice() {
        let output = RngOutput {
            rng: vec![
                RngResult::Dice {
                    notation: "2d6".to_string(),
                    value: 8,
                    modifier: 0,
                    source: vec![3, 5],
                },
                RngResult::Dice {
                    notation: "d20".to_string(),
                    value: 15,
                    modifier: 0,
                    source: vec![15],
                },
            ],
        };
        assert_eq!(output.to_plain(), "8\n15");
    }

    /// Test plain text output formatting for RNG range results.
    ///
    /// Verifies that `RngOutput::to_plain()` handles mixed result types
    /// (dice, integer ranges, float ranges) correctly, formatting each
    /// value appropriately.
    #[test]
    fn test_to_plain_mixed() {
        let output = RngOutput {
            rng: vec![
                RngResult::RangeInt {
                    notation: "1..100".to_string(),
                    value: 42,
                },
                RngResult::RangeFloat {
                    notation: "0.0..1.0".to_string(),
                    value: "0.756".to_string(),
                    precision: 3,
                    source: vec![0.756123],
                },
            ],
        };
        assert_eq!(output.to_plain(), "42\n0.756");
    }

    /// Test JSON output formatting for RNG results.
    ///
    /// Verifies that `RngOutput::to_json()` produces complete JSON
    /// with all specification details and generated values, allowing
    /// programmatic inspection of what was requested and generated.
    #[test]
    #[cfg(feature = "json")]
    fn test_to_json() {
        let output = RngOutput {
            rng: vec![
                RngResult::Dice {
                    notation: "2d6".to_string(),
                    value: 8,
                    modifier: 0,
                    source: vec![3, 5],
                },
                RngResult::RangeInt {
                    notation: "1..100".to_string(),
                    value: 42,
                },
            ],
        };
        let json = output.to_json();
        assert_eq!(json["rng"][0]["notation"], "2d6");
        assert_eq!(json["rng"][0]["value"], 8);
        assert_eq!(json["rng"][1]["notation"], "1..100");
        assert_eq!(json["rng"][1]["value"], 42);
    }
}
