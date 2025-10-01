use crate::{
    app::{AppContext, output::Output},
    error::GivError,
};
use rand::RngCore;
use serde::Serialize;

/// A specification for random number generation.
#[derive(Debug, Clone, PartialEq)]
enum RngSpec {
    /// Dice notation: XdY means roll X dice with Y sides each
    Dice {
        /// Number of dice to roll
        count: u64,
        /// Number of sides per die
        sides: u64,
    },
    /// Integer range: X..Y (inclusive)
    RangeInt {
        /// Start of range (inclusive)
        start: u64,
        /// End of range (inclusive)
        end: u64,
    },
    /// Float range: X..Y (inclusive) with specified precision
    RangeFloat {
        /// Start of range (inclusive)
        start: f64,
        /// End of range (inclusive)
        end: f64,
        /// Number of decimal places to display
        precision: usize,
    },
}

/// The result of a single random number generation.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
enum RngResult {
    /// Dice roll result
    #[serde(rename = "dice")]
    Dice {
        /// The dice notation used
        notation: String,
        /// The individual roll results
        rolls: Vec<u64>,
        /// The sum of all rolls
        value: u64,
    },
    /// Integer range result
    #[serde(rename = "range_int")]
    RangeInt {
        /// The range notation used
        notation: String,
        /// The generated value
        value: u64,
    },
    /// Float range result
    #[serde(rename = "range_float")]
    RangeFloat {
        /// The range notation used
        notation: String,
        /// The generated value
        value: f64,
        /// Number of decimal places
        precision: usize,
    },
}

/// The output from the RNG command.
#[derive(Debug, Serialize)]
struct RngOutput {
    /// List of results from each specification
    rng: Vec<RngResult>,
}

impl Output for RngOutput {
    fn to_plain(&self) -> String {
        self.rng
            .iter()
            .map(|result| match result {
                RngResult::Dice { rolls, .. } => rolls
                    .iter()
                    .map(|r| r.to_string())
                    .collect::<Vec<_>>()
                    .join(" "),
                RngResult::RangeInt { value, .. } => value.to_string(),
                RngResult::RangeFloat {
                    value, precision, ..
                } => format!("{value:.precision$}"),
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[cfg(feature = "json")]
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!(self)
    }
}

/// Parse a specification string into an `RngSpec`.
///
/// # Arguments
///
/// - `spec` The specification string to parse.
///
/// # Returns
///
/// A result containing the parsed `RngSpec` or an error.
///
/// # Errors
///
/// Returns an error if the specification string is invalid.
fn parse_spec(spec: &str) -> Result<RngSpec, GivError> {
    // Try to parse as dice notation: XdY or dY
    if let Some(d_pos) = spec.find('d') {
        let count_str = &spec[..d_pos];
        let sides_str = &spec[d_pos + 1..];

        // Handle "dY" as "1dY"
        let count = if count_str.is_empty() {
            1
        } else {
            count_str
                .parse::<u64>()
                .map_err(|_| GivError::InvalidRngSpec(spec.to_string()))?
        };

        let sides = sides_str
            .parse::<u64>()
            .map_err(|_| GivError::InvalidRngSpec(spec.to_string()))?;

        if count == 0 || sides == 0 {
            return Err(GivError::InvalidRngSpec(spec.to_string()));
        }

        return Ok(RngSpec::Dice { count, sides });
    }

    // Try to parse as range notation: X..Y
    if let Some(range_pos) = spec.find("..") {
        let start_str = &spec[..range_pos];
        let end_str = &spec[range_pos + 2..];

        // Check if either part contains a decimal point
        let is_float = start_str.contains('.') || end_str.contains('.');

        if is_float {
            let start = start_str
                .parse::<f64>()
                .map_err(|_| GivError::InvalidRngSpec(spec.to_string()))?;
            let end = end_str
                .parse::<f64>()
                .map_err(|_| GivError::InvalidRngSpec(spec.to_string()))?;

            if start >= end {
                return Err(GivError::InvalidRngSpec(spec.to_string()));
            }

            // Calculate precision: max of the decimal places in start and end
            let start_precision = count_decimal_places(start_str);
            let end_precision = count_decimal_places(end_str);
            let precision = start_precision.max(end_precision);

            return Ok(RngSpec::RangeFloat {
                start,
                end,
                precision,
            });
        } else {
            let start = start_str
                .parse::<u64>()
                .map_err(|_| GivError::InvalidRngSpec(spec.to_string()))?;
            let end = end_str
                .parse::<u64>()
                .map_err(|_| GivError::InvalidRngSpec(spec.to_string()))?;

            if start >= end {
                return Err(GivError::InvalidRngSpec(spec.to_string()));
            }

            return Ok(RngSpec::RangeInt { start, end });
        }
    }

    // If we get here, the spec is invalid
    Err(GivError::InvalidRngSpec(spec.to_string()))
}

/// Count the number of decimal places in a string representation of a number.
///
/// # Arguments
///
/// - `s` The string to analyze.
///
/// # Returns
///
/// The number of decimal places.
fn count_decimal_places(s: &str) -> usize {
    if let Some(dot_pos) = s.find('.') {
        s.len() - dot_pos - 1
    } else {
        0
    }
}

/// Generate a random integer in the range [min, max] (inclusive).
///
/// Uses rejection sampling to avoid modulo bias.
///
/// # Arguments
///
/// - `rng` The random number generator to use.
/// - `min` The minimum value (inclusive).
/// - `max` The maximum value (inclusive).
///
/// # Returns
///
/// A random integer in the specified range.
fn gen_range_int<R: RngCore>(rng: &mut R, min: u64, max: u64) -> u64 {
    debug_assert!(min <= max, "min must be less than or equal to max");
    let range = max - min + 1;

    // Calculate the maximum value we can accept to avoid modulo bias
    let max_acceptable = u64::MAX - (u64::MAX % range);

    // Rejection sampling: keep generating until we get a value in range
    loop {
        let random_u64 = rng.next_u64();
        if random_u64 < max_acceptable {
            return min + (random_u64 % range);
        }
        // If random_u64 >= max_acceptable, reject and try again
    }
}

/// Generate a random float in the range [min, max].
///
/// # Arguments
///
/// - `rng` The random number generator to use.
/// - `min` The minimum value (inclusive).
/// - `max` The maximum value (inclusive).
///
/// # Returns
///
/// A random float in the specified range.
fn gen_range_float<R: RngCore>(rng: &mut R, min: f64, max: f64) -> f64 {
    debug_assert!(min <= max, "min must be less than or equal to max");
    let random_u64 = rng.next_u64();
    // Convert u64 to f64 in range [0.0, 1.0)
    let normalized = (random_u64 as f64) / (u64::MAX as f64);
    // Scale to desired range
    min + normalized * (max - min)
}

/// Execute a single RNG specification.
///
/// # Arguments
///
/// - `rng` The random number generator to use.
/// - `spec` The specification to execute.
///
/// # Returns
///
/// A result containing the `RngResult` or an error.
fn execute_spec<R: RngCore>(rng: &mut R, spec: &RngSpec) -> Result<RngResult, GivError> {
    match spec {
        RngSpec::Dice { count, sides } => {
            let rolls: Vec<u64> = (0..*count).map(|_| gen_range_int(rng, 1, *sides)).collect();
            let value: u64 = rolls.iter().sum();
            let notation = if *count == 1 {
                format!("d{sides}")
            } else {
                format!("{count}d{sides}")
            };
            Ok(RngResult::Dice {
                notation,
                rolls,
                value,
            })
        }
        RngSpec::RangeInt { start, end } => {
            let value = gen_range_int(rng, *start, *end);
            let notation = format!("{start}..{end}");
            Ok(RngResult::RangeInt { notation, value })
        }
        RngSpec::RangeFloat {
            start,
            end,
            precision,
        } => {
            let value = gen_range_float(rng, *start, *end);
            // Format notation with the precision to preserve decimal places
            let notation = format!("{start:.precision$}..{end:.precision$}");
            Ok(RngResult::RangeFloat {
                notation,
                value,
                precision: *precision,
            })
        }
    }
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
pub fn rng_command(specs: Vec<String>, ctx: &mut AppContext) -> Result<(), GivError> {
    if specs.is_empty() {
        return Err(GivError::RequiredArgumentsNotProvided("giv rng --help".to_string()));
    }

    // Parse all specifications
    let parsed_specs: Result<Vec<RngSpec>, GivError> =
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

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// Test parsing dice notation
    #[test]
    fn test_parse_dice() {
        // Standard dice notation
        let spec = parse_spec("2d6").unwrap();
        assert_eq!(spec, RngSpec::Dice { count: 2, sides: 6 });

        // Shorthand notation (d6 means 1d6)
        let spec = parse_spec("d8").unwrap();
        assert_eq!(spec, RngSpec::Dice { count: 1, sides: 8 });

        // Large numbers
        let spec = parse_spec("10d100").unwrap();
        assert_eq!(
            spec,
            RngSpec::Dice {
                count: 10,
                sides: 100
            }
        );
    }

    /// Test parsing integer ranges
    #[test]
    fn test_parse_range_int() {
        let spec = parse_spec("1..100").unwrap();
        assert_eq!(spec, RngSpec::RangeInt { start: 1, end: 100 });

        let spec = parse_spec("0..10").unwrap();
        assert_eq!(spec, RngSpec::RangeInt { start: 0, end: 10 });
    }

    /// Test parsing float ranges
    #[test]
    fn test_parse_range_float() {
        let spec = parse_spec("0.0..1.0").unwrap();
        assert_eq!(
            spec,
            RngSpec::RangeFloat {
                start: 0.0,
                end: 1.0,
                precision: 1
            }
        );

        let spec = parse_spec("0.000..1").unwrap();
        assert_eq!(
            spec,
            RngSpec::RangeFloat {
                start: 0.0,
                end: 1.0,
                precision: 3
            }
        );

        let spec = parse_spec("1.5..10.75").unwrap();
        assert_eq!(
            spec,
            RngSpec::RangeFloat {
                start: 1.5,
                end: 10.75,
                precision: 2
            }
        );
    }

    /// Test invalid specifications
    #[test]
    fn test_parse_invalid() {
        // Missing parts
        assert!(parse_spec("d").is_err());
        assert!(parse_spec("..").is_err());
        assert!(parse_spec("1..").is_err());
        assert!(parse_spec("..10").is_err());

        // Invalid ranges
        assert!(parse_spec("10..5").is_err());
        assert!(parse_spec("10..10").is_err());

        // Zero dice
        assert!(parse_spec("0d6").is_err());
        assert!(parse_spec("2d0").is_err());

        // Not a valid spec
        assert!(parse_spec("hello").is_err());
        assert!(parse_spec("123").is_err());
    }

    /// Test decimal place counting
    #[test]
    fn test_count_decimal_places() {
        assert_eq!(count_decimal_places("1.0"), 1);
        assert_eq!(count_decimal_places("1.00"), 2);
        assert_eq!(count_decimal_places("1.000"), 3);
        assert_eq!(count_decimal_places("0.123456"), 6);
        assert_eq!(count_decimal_places("123"), 0);
        assert_eq!(count_decimal_places("123.0"), 1);
    }

    /// Test integer range generation
    #[test]
    fn test_gen_range_int() {
        let mut rng = rand::rng();
        // Test that generated values are in range
        for _ in 0..100 {
            let value = gen_range_int(&mut rng, 1, 10);
            assert!(value >= 1 && value <= 10);
        }

        // Test single value range
        let value = gen_range_int(&mut rng, 5, 5);
        assert_eq!(value, 5);
    }

    /// Test float range generation
    #[test]
    fn test_gen_range_float() {
        let mut rng = rand::rng();
        // Test that generated values are in range
        for _ in 0..100 {
            let value = gen_range_float(&mut rng, 0.0, 1.0);
            assert!(value >= 0.0 && value <= 1.0);
        }

        for _ in 0..100 {
            let value = gen_range_float(&mut rng, 10.5, 20.5);
            assert!(value >= 10.5 && value <= 20.5);
        }
    }

    /// Test spec execution
    #[test]
    fn test_execute_spec() {
        let mut rng = rand::rng();

        // Test dice execution
        let spec = RngSpec::Dice { count: 3, sides: 6 };
        let result = execute_spec(&mut rng, &spec).unwrap();
        match result {
            RngResult::Dice {
                notation,
                rolls,
                value,
            } => {
                assert_eq!(notation, "3d6");
                assert_eq!(rolls.len(), 3);
                let expected_sum: u64 = rolls.iter().sum();
                assert_eq!(value, expected_sum);
                for roll in rolls {
                    assert!(roll >= 1 && roll <= 6);
                }
                assert!(value >= 3 && value <= 18);
            }
            _ => panic!("Expected Dice result"),
        }

        // Test integer range execution
        let spec = RngSpec::RangeInt { start: 1, end: 100 };
        let result = execute_spec(&mut rng, &spec).unwrap();
        match result {
            RngResult::RangeInt { notation, value } => {
                assert_eq!(notation, "1..100");
                assert!(value >= 1 && value <= 100);
            }
            _ => panic!("Expected RangeInt result"),
        }

        // Test float range execution
        let spec = RngSpec::RangeFloat {
            start: 0.0,
            end: 1.0,
            precision: 3,
        };
        let result = execute_spec(&mut rng, &spec).unwrap();
        match result {
            RngResult::RangeFloat {
                notation,
                value,
                precision,
            } => {
                assert_eq!(notation, "0.000..1.000");
                assert_eq!(precision, 3);
                assert!(value >= 0.0 && value <= 1.0);
            }
            _ => panic!("Expected RangeFloat result"),
        }
    }
}
