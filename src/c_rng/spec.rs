use crate::error::GivError;

/// A specification for random number generation.
#[derive(Debug, Clone, PartialEq)]
pub enum RngSpec {
    /// Dice notation: XdY means roll X dice with Y sides each
    Dice {
        /// Number of dice to roll
        count: u64,
        /// Number of sides per die
        sides: u64,
        /// Modifier to add to the result (can be negative)
        modifier: i64,
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
pub fn parse_spec(spec: &str) -> Result<RngSpec, GivError> {
    // Try to parse as dice notation: XdY or dY with optional +N or -N modifier
    if let Some(d_pos) = spec.find('d') {
        let count_str = &spec[..d_pos];
        let rest = &spec[d_pos + 1..];

        // Handle "dY" as "1dY"
        let count = if count_str.is_empty() {
            1
        } else {
            count_str
                .parse::<u64>()
                .map_err(|_| GivError::InvalidRngSpec(spec.to_string()))?
        };

        // Parse sides and optional modifier
        let (sides_str, modifier) = if let Some(plus_pos) = rest.find('+') {
            let sides = &rest[..plus_pos];
            let mod_str = &rest[plus_pos + 1..];
            let modifier = mod_str
                .parse::<i64>()
                .map_err(|_| GivError::InvalidRngSpec(spec.to_string()))?;
            (sides, modifier)
        } else if let Some(minus_pos) = rest.find('-') {
            let sides = &rest[..minus_pos];
            let mod_str = &rest[minus_pos + 1..];
            let modifier = mod_str
                .parse::<i64>()
                .map_err(|_| GivError::InvalidRngSpec(spec.to_string()))?;
            (sides, -modifier)
        } else {
            (rest, 0)
        };

        let sides = sides_str
            .parse::<u64>()
            .map_err(|_| GivError::InvalidRngSpec(spec.to_string()))?;

        if count == 0 || sides == 0 {
            return Err(GivError::InvalidRngSpec(spec.to_string()));
        }

        return Ok(RngSpec::Dice {
            count,
            sides,
            modifier,
        });
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

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// Test parsing dice notation
    #[test]
    fn test_parse_dice() {
        // Standard dice notation
        let spec = parse_spec("2d6").unwrap();
        assert_eq!(
            spec,
            RngSpec::Dice {
                count: 2,
                sides: 6,
                modifier: 0
            }
        );

        // Shorthand notation (d6 means 1d6)
        let spec = parse_spec("d8").unwrap();
        assert_eq!(
            spec,
            RngSpec::Dice {
                count: 1,
                sides: 8,
                modifier: 0
            }
        );

        // Large numbers
        let spec = parse_spec("10d100").unwrap();
        assert_eq!(
            spec,
            RngSpec::Dice {
                count: 10,
                sides: 100,
                modifier: 0
            }
        );

        // Dice with positive modifier
        let spec = parse_spec("3d6+2").unwrap();
        assert_eq!(
            spec,
            RngSpec::Dice {
                count: 3,
                sides: 6,
                modifier: 2
            }
        );

        // Dice with negative modifier
        let spec = parse_spec("1d20-1").unwrap();
        assert_eq!(
            spec,
            RngSpec::Dice {
                count: 1,
                sides: 20,
                modifier: -1
            }
        );

        // Shorthand with modifier
        let spec = parse_spec("d8+5").unwrap();
        assert_eq!(
            spec,
            RngSpec::Dice {
                count: 1,
                sides: 8,
                modifier: 5
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
}
