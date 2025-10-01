/// CLI parsing and argument handling.
pub mod cli;
/// Command execution context.
mod context;
/// Output formatting and display.
pub mod output;

pub use context::AppContext;

#[cfg(feature = "bytes")]
use crate::c_bytes::bytes_command;
#[cfg(feature = "date")]
use crate::c_date::{DateKind, date_command};
#[cfg(feature = "key")]
use crate::c_key::key_command;
#[cfg(feature = "pi")]
use crate::c_pi::pi_command;
#[cfg(feature = "rng")]
use crate::c_rng::rng_command;
#[cfg(feature = "uuid")]
use crate::c_uuid::uuid_command;
use clap::Parser as _;
use cli::{Cli, Commands};
use std::process::ExitCode;

/// Run the application.
///
/// Parses command-line arguments, creates a command context, and executes
/// the requested command.
///
/// # Returns
///
/// Returns an `ExitCode` indicating success or failure.
#[allow(clippy::print_stderr)]
pub fn run() -> ExitCode {
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
        // The 'now' command. An alias for 'date now'.
        #[cfg(feature = "date")]
        Commands::Now { format } => date_command(DateKind::Now, format, &mut ctx),
        // The 'key' command.
        #[cfg(feature = "key")]
        Commands::Key { size } => key_command(size, &mut ctx),
        // The 'uuid' command.
        #[cfg(feature = "uuid")]
        Commands::Uuid => uuid_command(&mut ctx),
        // The 'date' command.
        #[cfg(feature = "date")]
        Commands::Date { kind, format } => date_command(kind, format, &mut ctx),
        // The 'pi' command.
        #[cfg(feature = "pi")]
        #[allow(unused_variables)]
        Commands::Pi {
            places,
            round,
            no_round,
        } => pi_command(places, (round, no_round), &mut ctx),
        // The 'rng' command.
        #[cfg(feature = "rng")]
        Commands::Rng { specs } => rng_command(specs, &mut ctx),
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
