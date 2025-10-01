//! Giv - A command-line utility providing various useful tools.
//!
//! This crate provides output for the following:
//! - Random byte generation.
//! - Formatted dates.
//! - Key generation.
//! - UUID v7 generation.
//! - PI.
//! - Random number generation.

/// Application module.
mod app;
/// Bytes generation module.
#[cfg(feature = "bytes")]
mod c_bytes;
/// Date command module.
#[cfg(feature = "date")]
mod c_date;
/// Key generation module.
#[cfg(feature = "key")]
mod c_key;
/// Pi calculation module.
#[cfg(feature = "pi")]
mod c_pi;
/// Random number generation module.
#[cfg(feature = "rng")]
mod c_rng;
/// UUID generation module.
#[cfg(feature = "uuid")]
mod c_uuid;
/// Error handling module.
mod error;

use std::process::ExitCode;

/// The application entry point.
///
/// # Returns
///
/// Returns an `ExitCode` indicating success or failure.
fn main() -> ExitCode {
    app::run()
}
