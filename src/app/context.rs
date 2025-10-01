use super::{cli::CommandOptions, output::Outputer};

/// Context for command execution.
///
/// This struct holds shared resources that commands need to execute,
/// including the output handler and, when enabled, a random number generator.
pub struct AppContext {
    /// The output handler for this command.
    output: Outputer,
    /// Random number generator (only when rng feature is enabled).
    #[cfg(feature = "rng")]
    rng: rand::rngs::ThreadRng,
}

impl AppContext {
    /// Create a new command context from command options.
    ///
    /// # Arguments
    ///
    /// - `options` The command options to configure the context.
    ///
    /// # Returns
    ///
    /// A new `CommandContext` instance.
    #[must_use]
    pub fn new(options: CommandOptions) -> Self {
        Self {
            output: Outputer::new(options),
            #[cfg(feature = "rng")]
            rng: rand::rng(),
        }
    }

    /// Get a reference to the output handler.
    ///
    /// # Returns
    ///
    /// A reference to the `Outputer`.
    #[must_use]
    pub fn output(&self) -> &Outputer {
        &self.output
    }

    /// Get a mutable reference to the random number generator.
    ///
    /// This is only available when the `rng` feature is enabled.
    ///
    /// # Returns
    ///
    /// A mutable reference to the RNG.
    #[cfg(feature = "rng")]
    #[must_use]
    pub fn rng(&mut self) -> &mut rand::rngs::ThreadRng {
        &mut self.rng
    }
}
