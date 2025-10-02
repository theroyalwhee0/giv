use crate::output::Output;
use serde::Serialize;

/// The output from the uuid command.
#[derive(Debug, Serialize)]
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
