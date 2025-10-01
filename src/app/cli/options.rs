use super::Cli;

/// The shared options for all commands.
#[derive(Debug)]
pub struct CommandOptions {
    /// Format the output as JSON.
    #[cfg(feature = "json")]
    pub json: bool,
}

/// Implement `Default` for `CommandOptions`
impl Default for CommandOptions {
    /// Create a default instance of `CommandOptions`
    ///
    /// # Returns
    ///
    /// A `CommandOptions` instance with default values.
    fn default() -> Self {
        Self {
            #[cfg(feature = "json")]
            json: false,
        }
    }
}

/// Implement the From trait for `CommandOptions`
impl From<&Cli> for CommandOptions {
    /// Convert a Cli reference into `CommandOptions`
    ///
    /// # Arguments
    ///
    /// - `cli` The CLI info.
    ///
    /// # Returns
    ///
    /// A `CommandOptions` instance.
    fn from(cli: &Cli) -> Self {
        Self {
            #[cfg(feature = "json")]
            json: cli.json,
        }
    }
}
