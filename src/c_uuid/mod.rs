/// Output formatting for uuid generation.
mod output;

use crate::{app::AppContext, error::GivError};
use output::UuidOutput;
use uuid::Uuid;

/// The 'uuid' command handler.
///
/// # Arguments
///
/// - `ctx` The command context.
///
/// # Returns
///
/// A result indicating success or failure.
pub fn uuid_command(ctx: &mut AppContext) -> Result<(), GivError> {
    // Generate a UUID version 7.
    let uuid = Uuid::now_v7();
    // Create output with the UUID.
    let output = UuidOutput {
        uuid: uuid.to_string(),
        version: "v7".to_string(),
    };
    // Output the UUID.
    ctx.output().output(&output);
    // Success.
    Ok(())
}
