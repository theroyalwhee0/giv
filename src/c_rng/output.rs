use crate::app::output::Output;
use serde::Serialize;

use super::result::RngResult;

/// The output from the RNG command.
#[derive(Debug, Serialize)]
pub struct RngOutput {
    /// List of results from each specification
    pub rng: Vec<RngResult>,
}

impl Output for RngOutput {
    fn to_plain(&self) -> String {
        self.rng
            .iter()
            .map(|result| match result {
                RngResult::Dice { rolls, .. } => rolls
                    .iter()
                    .map(|r| r.to_string())
                    .collect::<Vec<_>>()
                    .join(" "),
                RngResult::RangeInt { value, .. } => value.to_string(),
                RngResult::RangeFloat {
                    value, precision, ..
                } => format!("{value:.precision$}"),
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[cfg(feature = "json")]
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!(self)
    }
}
