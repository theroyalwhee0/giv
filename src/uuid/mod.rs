/// Output formatting for uuid generation.
pub mod output;

use crate::error::GivError;
pub use output::UuidOutput;
use uuid::Uuid;

/// Generate a UUID v7.
///
/// # Returns
///
/// Returns a [`UuidOutput`] containing the generated UUID and version information.
///
/// # Errors
///
/// This function does not currently return errors, but returns a Result for consistency.
///
/// # Examples
///
/// ```
/// use giv::uuid::generate_uuid;
/// use giv::GivError;
///
/// # fn main() -> Result<(), GivError> {
/// let uuid = generate_uuid()?;
/// println!("Generated UUID: {}", uuid.uuid);
/// assert_eq!(uuid.version, "v7");
/// # Ok(())
/// # }
/// ```
pub fn generate_uuid() -> Result<UuidOutput, GivError> {
    // Generate a UUID version 7.
    let uuid = Uuid::now_v7();
    // Create output with the UUID.
    Ok(UuidOutput {
        uuid: uuid.to_string(),
        version: "v7".to_string(),
    })
}
