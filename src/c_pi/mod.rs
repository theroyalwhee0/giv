/// Module containing PI decimal constants.
mod decimals;

use crate::{cli::CommandOptions, error::GivError, output::outputln};
use decimals::PI_DECIMALS;
use std::borrow::Cow;

/// The default rounding behavior.
pub const DEFAULT_ROUND: bool = true;

/// The default number of places to use, if not specified.
/// This matches the f64 PI and JS Math.PI precision.
const PI_DEFAULT_PLACES: usize = 15;

/// The PI prefix.
/// This is separated from the decimals to make the indexes 0-based.
const PI_PREFIX: &str = "3.";

/// The maximum number of decimal places for PI.
const PI_MAX_DECIMALS: usize = PI_DECIMALS.len();

/// The digit used for rounding up.
const ROUND_UP_FROM: u8 = 5;

/// Base 10.
const BASE10: u32 = 10;

/// The rounding flags for the 'pi' command.
/// The first element indicates if rounding is enabled,
/// the second element indicates if rounding is disabled.
pub type Rounding = (Option<bool>, Option<bool>);

/// Get PI with the specified number of decimal places.
///
/// # Arguments
///
/// - `places` The number of decimal places to display.
///
/// # Returns
///
/// A string representation of PI with the specified number of decimal places.
fn get_pi(places: usize, round: bool) -> Result<String, GivError> {
    // Ensure the number of places is within the valid range...
    if places == 0 || places > PI_MAX_DECIMALS {
        Err(GivError::DecimalPlacesOutOfRange(places, PI_MAX_DECIMALS))
    } else {
        // Create a Cow with a slice of the PI decimals
        let mut decimals = Cow::Borrowed(&PI_DECIMALS[..places]);

        // If rounding is enabled and there's a next digit to check...
        if round && places < PI_MAX_DECIMALS {
            let next_digit = PI_DECIMALS[places];
            if next_digit >= ROUND_UP_FROM {
                // Convert to owned vec for modification.
                let decimals = decimals.to_mut();

                // Round up from the last digit.
                for idx in (0..=decimals.len() - 1).rev() {
                    // If the current digit is less than 9, increment it and stop.
                    if decimals[idx] < 9 {
                        decimals[idx] += 1;
                        break;
                    } else {
                        // Otherwise, set it to 0 and carry the 1 to the next digit.
                        decimals[idx] = 0;
                        debug_assert!(idx == 0, "Rounding PI can not carry over to the 0th index.");
                    }
                }
            }
        }

        // Format the decimals into a string.
        let mut result = String::with_capacity(PI_PREFIX.len() + decimals.len());
        result.push_str(PI_PREFIX);
        for &digit in decimals.iter() {
            result.push(char::from_digit(digit as u32, BASE10).unwrap());
        }

        // Success.
        Ok(result)
    }
}

/// The 'pi' command handler.
///
/// # Arguments
///
/// - `places` The number of decimal places to display.
/// - `rounding` A tuple indicating with the CLI rounding flags.
/// - `options` The command options.
///
/// # Returns
///
/// A result indicating success or failure.
pub fn pi_command(
    places: Option<usize>,
    rounding: Rounding,
    options: CommandOptions,
) -> Result<(), GivError> {
    // Default the number of places if not specified.
    let places = places.unwrap_or(PI_DEFAULT_PLACES);

    // Determines if rounding is enabled from CLI flags.
    let round = match rounding {
        // User explicitly enabled rounding.
        (Some(true), _) => true,
        // User explicitly disabled rounding.
        (_, Some(true)) => false,
        // Otherwise, use the default.
        (_, _) => DEFAULT_ROUND,
    };

    // Get the PI value with the specified number of decimal places.
    let pi_value = get_pi(places, round)?;

    // Output the PI value.
    outputln(options, pi_value);

    // Success.
    Ok(())
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// Test the `get_pi` function with various decimal places.
    #[test]
    fn test_get_pi() {
        // Test with default size.
        let result = get_pi(PI_DEFAULT_PLACES, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "3.141592653589793");
        // Test with 1 decimal place.
        assert_eq!(get_pi(1, true).unwrap(), "3.1");
        assert_eq!(get_pi(1, false).unwrap(), "3.1");
        // Test with 2 decimal places.
        assert_eq!(get_pi(2, true).unwrap(), "3.14");
        assert_eq!(get_pi(2, false).unwrap(), "3.14");
        // Test with 3 decimal places.
        assert_eq!(get_pi(3, true).unwrap(), "3.142");
        assert_eq!(get_pi(3, false).unwrap(), "3.141");
        // Test with 6 decimal places.
        assert_eq!(get_pi(6, true).unwrap(), "3.141593");
        assert_eq!(get_pi(6, false).unwrap(), "3.141592");
        // Test with 7 decimal places.
        assert_eq!(get_pi(7, true).unwrap(), "3.1415927");
        assert_eq!(get_pi(7, false).unwrap(), "3.1415926");
        // Test with 10 decimal places.
        assert_eq!(get_pi(10, true).unwrap(), "3.1415926536");
        assert_eq!(get_pi(10, false).unwrap(), "3.1415926535");
        // Test with 15 decimal places.
        assert_eq!(get_pi(15, true).unwrap(), "3.141592653589793");
        assert_eq!(get_pi(15, false).unwrap(), "3.141592653589793");
        // Test comparison with built-in f64::consts::PI.
        let f64_pi = std::f64::consts::PI.to_string();
        assert_eq!(&get_pi(15, true).unwrap(), &f64_pi);
        // Test comparison with built-in f32::consts::PI.
        let f32_pi = std::f32::consts::PI.to_string();
        assert_eq!(&get_pi(7, true).unwrap(), &f32_pi);

        // Test with 25 decimal places
        assert_eq!(get_pi(25, true).unwrap(), "3.1415926535897932384626434");
        assert_eq!(get_pi(25, false).unwrap(), "3.1415926535897932384626433");
        // Test with 50 decimal places.
        assert_eq!(
            get_pi(50, true).unwrap(),
            "3.14159265358979323846264338327950288419716939937511"
        );
        assert_eq!(
            get_pi(50, false).unwrap(),
            "3.14159265358979323846264338327950288419716939937510"
        );
        // Test with the full size.
        let full_decimals = format!(
            "{}{}",
            PI_PREFIX,
            PI_DECIMALS
                .iter()
                .map(|&digit| char::from_digit(digit as u32, BASE10).unwrap())
                .collect::<String>()
        );
        assert_eq!(get_pi(PI_DECIMALS.len(), true).unwrap(), full_decimals);
        assert_eq!(get_pi(PI_DECIMALS.len(), false).unwrap(), full_decimals);
    }

    /// Test the error behavior of `get_pi` with 0 decimal places.
    #[test]
    fn test_get_pi_zero_places() {
        let result = get_pi(0, true);
        assert!(result.is_err());
        let err = result.unwrap_err();
        match err {
            GivError::DecimalPlacesOutOfRange(places, max) => {
                assert_eq!(places, 0);
                assert_eq!(max, PI_MAX_DECIMALS);
            }
        }
        assert_eq!(
            err.to_string(),
            format!(
                "Requested number of PI decimal places '{}' is not supported please select a value between '1' and '{}'",
                0, PI_MAX_DECIMALS
            )
        );
    }

    /// Test the error behavior of `get_pi` with too many decimal places.
    #[test]
    fn test_get_pi_too_many_places() {
        let result = get_pi(PI_MAX_DECIMALS + 1, true);
        assert!(result.is_err());
        let err = result.unwrap_err();
        match err {
            GivError::DecimalPlacesOutOfRange(places, max) => {
                assert_eq!(places, PI_MAX_DECIMALS + 1);
                assert_eq!(max, PI_MAX_DECIMALS);
            }
        }
        assert_eq!(
            err.to_string(),
            format!(
                "Requested number of PI decimal places '{}' is not supported please select a value between '1' and '{}'",
                PI_MAX_DECIMALS + 1,
                PI_MAX_DECIMALS
            )
        );
    }

    /// Test the length of the `PI_DECIMALS` constant.
    #[test]
    fn test_pi_decimals_length() {
        // Ensure the length of PI_DECIMALS is 10000.
        assert_eq!(PI_DECIMALS.len(), 10_000);
    }
}
