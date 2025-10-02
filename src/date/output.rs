use crate::output::Output;
use serde::Serialize;

/// The output from the date command.
#[derive(Debug, Serialize)]
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
