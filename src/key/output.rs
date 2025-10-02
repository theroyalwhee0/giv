use crate::output::Output;
use serde::Serialize;

/// The output from the key command.
#[derive(Debug, Serialize)]
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
