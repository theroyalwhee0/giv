//! Command handlers for the CLI binary.
//!
//! This module contains all `*_command` functions that bridge between
//! the CLI argument parsing and the library API functions.

use super::AppContext;
use giv::error::GivError;

#[cfg(feature = "bytes")]
use giv::bytes::BytesEncoding;
#[cfg(feature = "chars")]
use giv::chars::{convert_input, CharResult, CharsOutput};
#[cfg(feature = "date")]
use giv::date::{DateFormat, DateKind};
#[cfg(feature = "key")]
use giv::key;
#[cfg(feature = "pi")]
use giv::pi::{self, RoundingFlags};
#[cfg(feature = "rng")]
use giv::rng::{execute::execute_spec, output::RngOutput, spec::parse_spec};
#[cfg(feature = "uuid")]
use giv::uuid;
#[cfg(feature = "date")]
use chrono::Utc;

/// The 'bytes' command handler.
///
/// # Arguments
///
/// - `length` An optional length for the bytes to generate.
/// - `encoding` An optional encoding format for the output.
/// - `padding` Whether to use padding for base64 encoding.
/// - `ctx` The command context.
///
/// # Returns
///
/// A result indicating success or failure.
///
/// # Errors
///
/// Returns an error if:
/// - Raw encoding is requested with JSON output mode.
#[cfg(feature = "bytes")]
pub fn bytes_command(
    length: Option<usize>,
    encoding: Option<BytesEncoding>,
    padding: bool,
    ctx: &mut AppContext,
) -> Result<(), GivError> {
    use giv::bytes;

    // Check if raw encoding is requested with JSON output mode.
    if matches!(encoding, Some(BytesEncoding::Raw)) && ctx.output().is_json() {
        return Err(GivError::RawBytesNotSupportedInJson);
    }

    // Generate the random bytes with encoding.
    let output = bytes::generate_bytes(length, encoding, padding)?;

    // Output the bytes.
    ctx.output().output(&output);

    Ok(())
}

/// The 'chars' command handler.
///
/// # Arguments
///
/// - `inputs` The list of patterns or shortcodes to convert.
/// - `ctx` The command context.
///
/// # Returns
///
/// A result indicating success or failure.
///
/// # Errors
///
/// Returns an error if any pattern or shortcode is not recognized.
#[cfg(feature = "chars")]
pub fn chars_command(inputs: Vec<String>, ctx: &mut AppContext) -> Result<(), GivError> {
    // Convert all inputs, collecting into a Result.
    let results: Result<Vec<CharResult>, GivError> =
        inputs.iter().map(|input| convert_input(input)).collect();

    let results = results?;

    // Create output with results.
    let output = CharsOutput { results };

    // Output the results.
    ctx.output().output(&output);

    Ok(())
}

/// The 'date' command handler.
///
/// # Arguments
///
/// * `kind` - The kind of date to generate
/// * `format` - The optional format to use
/// * `ctx` - The command context
///
/// # Returns
///
/// Returns a Result indicating success or failure.
///
/// # Errors
///
/// This function does not currently return errors, but returns a Result for consistency.
#[cfg(feature = "date")]
pub fn date_command(
    kind: DateKind,
    format: Option<DateFormat>,
    ctx: &mut AppContext,
) -> Result<(), GivError> {
    use giv::date::{format_date_time, get_date_format, get_date_time, output::DateOutput};

    // Get the current time.
    let now = Utc::now();

    // Get the specified date.
    let date = get_date_time(now, &kind);

    // Get the date format, defaulting if not specified.
    let format = get_date_format(&kind, format);

    // Format the date.
    let formatted = format_date_time(&date, &format);

    // Create output with the formatted date.
    let output = DateOutput { date: formatted };

    // Output the date.
    ctx.output().output(&output);

    Ok(())
}

/// The 'key' command handler.
///
/// # Arguments
///
/// - `size` An optional size for the key.
/// - `ctx` The command context.
///
/// # Returns
///
/// A result indicating success or failure.
///
/// # Errors
///
/// Propagates errors from [`key::generate_key`].
#[cfg(feature = "key")]
pub fn key_command(size: Option<usize>, ctx: &mut AppContext) -> Result<(), GivError> {
    let output = key::generate_key(size)?;
    ctx.output().output(&output);
    Ok(())
}

/// The 'pi' command handler.
///
/// # Arguments
///
/// - `places` The number of decimal places to display.
/// - `rounding_flags` A tuple indicating with the CLI rounding flags.
/// - `ctx` The command context.
///
/// # Returns
///
/// A result indicating success or failure.
///
/// # Errors
///
/// Returns an error if the number of decimal places is out of range or if conflicting flags are provided.
#[cfg(feature = "pi")]
pub fn pi_command(
    places: Option<usize>,
    rounding_flags: RoundingFlags,
    ctx: &mut AppContext,
) -> Result<(), GivError> {
    use giv::pi::get_rounding;

    // Determine if rounding is enabled from CLI flags.
    let round = get_rounding(rounding_flags)?;

    // Generate PI with the specified options.
    let output = pi::generate_pi(places, Some(round))?;

    // Output the PI value.
    ctx.output().output(&output);

    Ok(())
}

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
#[cfg(feature = "rng")]
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

/// The 'uuid' command handler.
///
/// # Arguments
///
/// - `ctx` The command context.
///
/// # Returns
///
/// A result indicating success or failure.
///
/// # Errors
///
/// Propagates errors from [`uuid::generate_uuid`].
#[cfg(feature = "uuid")]
pub fn uuid_command(ctx: &mut AppContext) -> Result<(), GivError> {
    let output = uuid::generate_uuid()?;
    ctx.output().output(&output);
    Ok(())
}
