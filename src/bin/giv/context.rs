use super::{cli::CommandOptions, output::Formatter};

/// Context for command execution.
///
/// This struct holds shared resources that commands need to execute,
/// including the output formatter and, when enabled, a random number generator.
pub struct AppContext {
    /// The output formatter for this command.
    output: Formatter,
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
            output: Formatter::new(options),
            #[cfg(feature = "rng")]
            rng: rand::rng(),
        }
    }

    /// Get a reference to the output formatter.
    ///
    /// # Returns
    ///
    /// A reference to the `Formatter`.
    #[must_use]
    pub fn format(&self) -> &Formatter {
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
