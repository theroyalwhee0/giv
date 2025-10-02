use crate::output::Output;
#[cfg(feature = "json")]
use serde::Serialize;

/// The output from the uuid command.
#[cfg_attr(feature = "json", derive(Serialize))]
#[derive(Debug)]
pub struct UuidOutput {
    /// The generated UUID
    pub uuid: String,
    /// The UUID version
    pub version: String,
}

impl Output for UuidOutput {
    fn to_plain(&self) -> String {
        self.uuid.clone()
    }

    #[cfg(feature = "json")]
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!(self)
    }
}
