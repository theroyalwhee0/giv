use crate::{app::AppContext, error::GivError};
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
    // Print the generated UUID.
    ctx.output().output(&uuid.to_string());
    // Success.
    Ok(())
}
