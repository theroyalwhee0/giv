use crate::cli::CommandOptions;
use giv::output::Output;

/// Handles formatting and outputting values in either plain text or JSON format.
///
/// This struct carries the output configuration (whether to use JSON or plain text)
/// and provides a unified interface for formatting and outputting values.
pub struct Formatter {
    /// Whether to output in JSON format.
    #[cfg(feature = "json")]
    json: bool,
}

impl Formatter {
    /// Create a new `Formatter` from command options.
    ///
    /// # Arguments
    ///
    /// - `options` The command options containing output format preferences.
    ///
    /// # Returns
    ///
    /// A `Formatter` configured according to the command options.
    #[must_use]
    pub fn new(options: CommandOptions) -> Self {
        Self {
            #[cfg(feature = "json")]
            json: options.json,
        }
    }

    /// Check if the formatter is configured for JSON output.
    ///
    /// # Returns
    ///
    /// `true` if JSON output is enabled, `false` otherwise.
    #[cfg(feature = "json")]
    #[must_use]
    pub fn is_json(&self) -> bool {
        self.json
    }

    /// Check if the formatter is configured for JSON output.
    ///
    /// # Returns
    ///
    /// Always returns `false` when the JSON feature is disabled.
    #[cfg(not(feature = "json"))]
    #[must_use]
    pub fn is_json(&self) -> bool {
        false
    }

    /// Output a value that implements the `Output` trait.
    ///
    /// This will output the value in either plain text or JSON format,
    /// depending on the configuration.
    ///
    /// # Arguments
    ///
    /// - `value` The value to output.
    #[allow(clippy::print_stdout)]
    pub fn output<T: Output>(&self, value: &T) {
        #[cfg(feature = "json")]
        if self.json {
            let json_value = value.to_json();
            let json_output = serde_json::to_string(&json_value).unwrap();
            println!("{json_output}");
            return;
        }

        println!("{}", value.to_plain());
    }
}
