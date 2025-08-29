#![doc = include_str!("../README.md")]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![warn(clippy::doc_markdown)]

#[cfg(feature = "date")]
mod c_date;
#[cfg(feature = "key")]
mod c_key;
#[cfg(feature = "pi")]
mod c_pi;
#[cfg(feature = "uuid")]
mod c_uuid;
mod cli;
mod error;
mod output;

use c_date::DateKind;
#[cfg(feature = "date")]
use c_date::date_command;
#[cfg(feature = "key")]
use c_key::key_command;
#[cfg(feature = "pi")]
use c_pi::pi_command;
#[cfg(feature = "uuid")]
use c_uuid::uuid_command;
use clap::Parser as _;
use cli::{Cli, Commands};
use std::process::ExitCode;

/// The application entry point.
///
/// # Returns
///
/// Returns an `ExitCode` indicating success or failure.
fn main() -> ExitCode {
    // Parse command line arguments.
    let cli = Cli::parse();
    let cmd_options = (&cli).into();
    // Handle the command line arguments.
    let result = match cli.command {
        // The 'now' command. An alias for 'date now'.
        #[cfg(feature = "date")]
        Commands::Now { format } => date_command(DateKind::Now, format, cmd_options),
        // The 'key' command.
        #[cfg(feature = "key")]
        Commands::Key { size } => key_command(size, cmd_options),
        // The 'uuid' command.
        #[cfg(feature = "uuid")]
        Commands::Uuid => uuid_command(cmd_options),
        // The 'date' command.
        #[cfg(feature = "date")]
        Commands::Date { kind, format } => date_command(kind, format, cmd_options),
        // The 'pi' command.
        #[cfg(feature = "pi")]
        #[allow(unused_variables)]
        Commands::Pi {
            places,
            round,
            no_round,
        } => pi_command(places, (round, no_round), cmd_options),
    };
    // If the result is an error, print the error message and return failure.
    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {}", err);
            ExitCode::FAILURE
        }
    }
}
