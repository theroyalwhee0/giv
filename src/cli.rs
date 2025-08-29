use crate::c_date::{DateFormat, DateKind};
use clap::{Parser, Subcommand};

/// A command-line utility generating values.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Format the output as JSON.
    #[cfg(feature = "json")]
    #[arg(short, long, default_value_t = false)]
    pub json: bool,

    /// The available commands.
    #[command(subcommand)]
    pub command: Commands,
}

/// The available commands for the CLI.
#[derive(Subcommand)]
pub enum Commands {
    /// Generate a random key in the format of `key_<alphanumeric[size]>`
    #[cfg(feature = "key")]
    Key {
        /// Size of the output key in characters (default: 36)
        #[arg(default_value = None)]
        size: Option<usize>,
    },
    /// Generate and display a UUID version 7
    #[cfg(feature = "uuid")]
    Uuid,
    /// Pi with the specified number of places.
    #[cfg(feature = "pi")]
    Pi {
        /// Number of decimal places to display. (default: 15)
        #[arg(default_value = None)]
        places: Option<usize>,
        /// Round flag. Use --no-round to negate this. (default: true)
        #[arg(long = "round", conflicts_with = "no_round", action = clap::ArgAction::SetTrue)]
        round: Option<bool>,
        /// No-Round flag. This is the negation of the round flag. (default: false)
        #[arg(long = "no-round", conflicts_with = "round", action = clap::ArgAction::SetTrue, hide= true)]
        no_round: Option<bool>,
    },
    /// Generate a date
    #[cfg(feature = "date")]
    Date {
        /// Date kind.
        #[arg(value_enum)]
        kind: DateKind,
        /// Format.
        #[arg(short, long, value_enum, default_value = None)]
        format: Option<DateFormat>,
    },
    /// Display the current UTC time. This is an alias for `date now`.
    #[cfg(feature = "date")]
    Now {
        /// Format.
        #[arg(short, long, value_enum, default_value = None)]
        format: Option<DateFormat>,
    },
}

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
