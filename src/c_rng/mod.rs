/// Execution logic for RNG specifications.
pub mod execute;
/// Random number generation functions.
pub mod generator;
/// Output formatting for RNG results.
pub mod output;
/// Result types for RNG operations.
pub mod result;
/// Specification parsing for RNG commands.
pub mod spec;

use crate::{app::AppContext, error::GivError};

use execute::execute_spec;
pub use output::RngOutput;
use spec::parse_spec;

/// The 'rng' command handler.
///
/// # Arguments
///
/// - `specs` The list of RNG specifications to execute.
/// - `ctx` The command context.
///
/// # Returns
///
/// A result indicating success or failure.
///
/// # Errors
///
/// Returns [`GivError::RequiredArgumentsNotProvided`] if specs is empty.
/// Propagates parsing errors from spec parsing and execution.
pub fn rng_command(specs: Vec<String>, ctx: &mut AppContext) -> Result<(), GivError> {
    if specs.is_empty() {
        return Err(GivError::RequiredArgumentsNotProvided(
            "giv rng --help".to_string(),
        ));
    }

    // Parse all specifications
    let parsed_specs: Result<Vec<_>, GivError> =
        specs.iter().map(|s| parse_spec(s.as_str())).collect();
    let parsed_specs = parsed_specs?;

    // Execute all specifications using the context's RNG
    let mut results = Vec::with_capacity(parsed_specs.len());
    for spec in &parsed_specs {
        results.push(execute_spec(ctx.rng(), spec)?);
    }

    // Create output
    let output = RngOutput { rng: results };

    // Output the results
    ctx.output().output(&output);

    Ok(())
}
