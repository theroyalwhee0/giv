//! Giv - A command-line utility providing various useful tools.
//!
//! This crate provides output for the following:
//! - Random byte generation.
//! - Character and emoji conversion.
//! - Formatted dates.
//! - Key generation.
//! - Lorem ipsum text.
//! - UUID generation (v4 and v7 with multiple formats).
//! - PI.
//! - Random number generation.

/// CLI parsing and argument handling.
mod cli;
/// Command handlers bridging CLI to library functions.
mod commands;
/// Command execution context.
mod context;
/// Output formatting and display.
mod output;

use commands::*;
use context::AppContext;

use clap::Parser as _;
use cli::{Cli, Commands};
#[cfg(feature = "date")]
use giv::date::DateKind;
use std::process::ExitCode;

/// The application entry point.
///
/// Parses command-line arguments, creates a command context, and executes
/// the requested command.
///
/// # Returns
///
/// Returns an `ExitCode` indicating success or failure.
#[allow(clippy::print_stderr)]
fn main() -> ExitCode {
    // Parse command line arguments.
    let cli = Cli::parse();
    let cmd_options = (&cli).into();
    let mut ctx = AppContext::new(cmd_options);

    // Handle the command line arguments.
    let result = match cli.command {
        // The 'bytes' command.
        #[cfg(feature = "bytes")]
        Commands::Bytes {
            length,
            encoding,
            pad,
        } => bytes_command(length, encoding, pad, &mut ctx),
        // The 'chars' command.
        #[cfg(feature = "chars")]
        Commands::Chars { inputs } => chars_command(inputs, &mut ctx),
        // The 'now' command. An alias for 'date now'.
        #[cfg(feature = "date")]
        Commands::Now { format } => date_command(DateKind::Now, format, &mut ctx),
        // The 'key' command.
        #[cfg(feature = "key")]
        Commands::Key { size } => key_command(size, &mut ctx),
        // The 'uuid' command.
        #[cfg(feature = "uuid")]
        Commands::Uuid {
            version,
            format,
            uppercase,
        } => uuid_command(version, format, uppercase, &mut ctx),
        // The 'date' command.
        #[cfg(feature = "date")]
        Commands::Date { kind, format } => date_command(kind, format, &mut ctx),
        // The 'pi' command.
        #[cfg(feature = "pi")]
        Commands::Pi {
            places,
            round,
            no_round,
        } => pi_command(places, (round, no_round), &mut ctx),
        // The 'rng' command.
        #[cfg(feature = "rng")]
        Commands::Rng { specs } => rng_command(specs, &mut ctx),
        // The 'lorem' command.
        #[cfg(feature = "lorem")]
        Commands::Lorem {
            count,
            words: _,
            sentences,
            paragraphs,
        } => {
            use giv::lorem::LoremUnit;
            let unit = if sentences {
                LoremUnit::Sentences
            } else if paragraphs {
                LoremUnit::Paragraphs
            } else {
                // Default to words if no flag is specified
                LoremUnit::Words
            };
            lorem_command(count, unit, &mut ctx)
        }
    };

    // If the result is an error, print the error message and return failure.
    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err}");
            ExitCode::FAILURE
        }
    }
}
