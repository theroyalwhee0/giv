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
