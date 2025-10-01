use crate::app::output::Output;
use serde::Serialize;

/// The output from the pi command.
#[derive(Debug, Serialize)]
pub struct PiOutput {
    /// The pi value
    pub pi: String,
    /// Whether rounding was applied
    pub rounded: bool,
}

impl Output for PiOutput {
    fn to_plain(&self) -> String {
        self.pi.clone()
    }

    #[cfg(feature = "json")]
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!(self)
    }
}
