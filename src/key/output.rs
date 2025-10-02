use crate::output::Output;
#[cfg(feature = "json")]
use serde::Serialize;

/// The output from the key command.
#[cfg_attr(feature = "json", derive(Serialize))]
#[derive(Debug)]
pub struct KeyOutput {
    /// The generated key
    pub key: String,
}

impl Output for KeyOutput {
    fn to_plain(&self) -> String {
        self.key.clone()
    }

    #[cfg(feature = "json")]
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!(self)
    }
}
