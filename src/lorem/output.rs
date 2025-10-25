use crate::output::Output;
#[cfg(feature = "json")]
use serde::Serialize;

/// The output from the lorem command.
#[cfg_attr(feature = "json", derive(Serialize))]
#[derive(Debug)]
pub struct LoremOutput {
    /// The generated lorem ipsum text.
    pub lorem: String,
}

impl Output for LoremOutput {
    fn to_plain(&self) -> String {
        self.lorem.clone()
    }

    #[cfg(feature = "json")]
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!(self)
    }
}
