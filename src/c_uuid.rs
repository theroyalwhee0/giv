#![cfg(feature = "uuid")]

use crate::{cli::CommandOptions, error::GivError, output::outputln};
use uuid::Uuid;

/// The 'uuid' command handler.
///
/// # Returns
///
/// A result indicating success or failure.
pub fn uuid_command(options: CommandOptions) -> Result<(), GivError> {
    // Generate a UUID version 7.
    let uuid = Uuid::now_v7();
    // Print the generated UUID.
    outputln(options, uuid);
    // Success.
    Ok(())
}
