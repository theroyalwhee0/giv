use crate::cli::CommandOptions;
use giv::output::Output;

/// Handles formatting and outputting values in either plain text or JSON format.
///
/// This struct carries the output configuration (whether to use JSON or plain text)
/// and provides a unified interface for formatting and outputting values.
pub struct Formatter {
    /// Whether to copy output to clipboard.
    #[cfg(feature = "clipboard")]
    clip: bool,
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
            #[cfg(feature = "clipboard")]
            clip: options.clip,
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
    /// depending on the configuration. If clipboard is enabled, the output
    /// will also be copied to the clipboard.
    ///
    /// # Arguments
    ///
    /// - `value` The value to output.
    #[allow(clippy::print_stdout)]
    #[allow(clippy::print_stderr)]
    pub fn output<T: Output>(&self, value: &T) {
        // Generate the output string
        let output_string = {
            #[cfg(feature = "json")]
            if self.json {
                let json_value = value.to_json();
                serde_json::to_string(&json_value).unwrap()
            } else {
                value.to_plain()
            }

            #[cfg(not(feature = "json"))]
            value.to_plain()
        };

        // Always print to stdout
        println!("{output_string}");

        // Copy to clipboard if enabled
        #[cfg(feature = "clipboard")]
        if self.clip {
            match arboard::Clipboard::new().and_then(|mut ctx| ctx.set_text(output_string)) {
                Ok(()) => {}
                Err(e) => eprintln!("Warning: Failed to copy to clipboard: {e}"),
            }
        }
    }
}
