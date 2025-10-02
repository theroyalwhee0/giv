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
