use crate::output::Output;
#[cfg(feature = "json")]
use serde::Serialize;

/// The output from the pi command.
#[cfg_attr(feature = "json", derive(Serialize))]
#[derive(Debug)]
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
