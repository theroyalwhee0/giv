//! Giv - A command-line utility providing various useful tools.
//!
//! This crate provides output for the following:
//! - Random byte generation.
//! - Character and emoji conversion.
//! - Formatted dates.
//! - Key generation.
//! - UUID v7 generation.
//! - PI.
//! - Random number generation.

/// Application module.
mod app;
/// Bytes generation module.
#[cfg(feature = "bytes")]
mod bytes;
/// Character and emoji conversion module.
#[cfg(feature = "chars")]
mod chars;
/// Date command module.
#[cfg(feature = "date")]
mod date;
/// Key generation module.
#[cfg(feature = "key")]
mod key;
/// Pi calculation module.
#[cfg(feature = "pi")]
mod pi;
/// Random number generation module.
#[cfg(feature = "rng")]
mod rng;
/// UUID generation module.
#[cfg(feature = "uuid")]
mod uuid;
/// Error handling module.
mod error;
/// Output formatting module.
mod output;

use std::process::ExitCode;

/// The application entry point.
///
/// # Returns
///
/// Returns an `ExitCode` indicating success or failure.
fn main() -> ExitCode {
    app::run()
}
