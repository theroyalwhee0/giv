use crate::error::GivError;
use rand::RngCore;

use super::generator::{gen_range_float, gen_range_int};
use super::result::RngResult;
use super::spec::RngSpec;

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
pub fn execute_spec<R: RngCore>(rng: &mut R, spec: &RngSpec) -> Result<RngResult, GivError> {
    match spec {
        RngSpec::Dice {
            count,
            sides,
            modifier,
        } => {
            let source: Vec<u64> = (0..*count).map(|_| gen_range_int(rng, 1, *sides)).collect();
            let sum: u64 = source.iter().sum();

            // Convert sum to i64 safely
            let sum_i64 = i64::try_from(sum).map_err(|_| {
                GivError::NumericOverflow(format!("dice sum {sum} too large for i64"))
            })?;

            // Apply modifier with overflow checking
            let value = sum_i64.checked_add(*modifier).ok_or_else(|| {
                GivError::NumericOverflow(format!(
                    "overflow applying modifier {modifier} to sum {sum_i64}"
                ))
            })?;

            // Format notation with modifier if non-zero
            let notation = if *count == 1 {
                if *modifier == 0 {
                    format!("d{sides}")
                } else if *modifier > 0 {
                    format!("d{sides}+{modifier}")
                } else {
                    format!("d{sides}{modifier}")
                }
            } else if *modifier == 0 {
                format!("{count}d{sides}")
            } else if *modifier > 0 {
                format!("{count}d{sides}+{modifier}")
            } else {
                format!("{count}d{sides}{modifier}")
            };

            Ok(RngResult::Dice {
                notation,
                value,
                modifier: *modifier,
                source,
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
            let raw_value = gen_range_float(rng, *start, *end);
            let value = format!("{raw_value:.precision$}");
            // Format notation with the precision to preserve decimal places
            let notation = format!("{start:.precision$}..{end:.precision$}");
            Ok(RngResult::RangeFloat {
                notation,
                value,
                precision: *precision,
                source: vec![raw_value],
            })
        }
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// Test spec execution
    #[test]
    fn test_execute_spec() {
        let mut rng = rand::rng();

        // Test dice execution without modifier
        let spec = RngSpec::Dice {
            count: 3,
            sides: 6,
            modifier: 0,
        };
        let result = execute_spec(&mut rng, &spec).unwrap();
        match result {
            RngResult::Dice {
                notation,
                value,
                modifier,
                source,
            } => {
                assert_eq!(notation, "3d6");
                assert_eq!(modifier, 0);
                assert_eq!(source.len(), 3);
                let expected_sum: i64 = source.iter().map(|&x| i64::try_from(x).unwrap()).sum();
                assert_eq!(value, expected_sum);
                for roll in source {
                    assert!(roll >= 1 && roll <= 6);
                }
                assert!(value >= 3 && value <= 18);
            }
            _ => panic!("Expected Dice result"),
        }

        // Test dice execution with positive modifier
        let spec = RngSpec::Dice {
            count: 2,
            sides: 6,
            modifier: 5,
        };
        let result = execute_spec(&mut rng, &spec).unwrap();
        match result {
            RngResult::Dice {
                notation,
                value,
                modifier,
                source,
            } => {
                assert_eq!(notation, "2d6+5");
                assert_eq!(modifier, 5);
                assert_eq!(source.len(), 2);
                let roll_sum: i64 = source.iter().map(|&x| i64::try_from(x).unwrap()).sum();
                assert_eq!(value, roll_sum + 5);
                assert!(value >= 7 && value <= 17);
            }
            _ => panic!("Expected Dice result"),
        }

        // Test dice execution with negative modifier
        let spec = RngSpec::Dice {
            count: 1,
            sides: 20,
            modifier: -1,
        };
        let result = execute_spec(&mut rng, &spec).unwrap();
        match result {
            RngResult::Dice {
                notation,
                value,
                modifier,
                source,
            } => {
                assert_eq!(notation, "d20-1");
                assert_eq!(modifier, -1);
                assert_eq!(source.len(), 1);
                let roll_sum: i64 = source.iter().map(|&x| i64::try_from(x).unwrap()).sum();
                assert_eq!(value, roll_sum - 1);
                assert!(value >= 0 && value <= 19);
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
                source,
            } => {
                assert_eq!(notation, "0.000..1.000");
                assert_eq!(precision, 3);
                assert_eq!(source.len(), 1);
                assert!(source[0] >= 0.0 && source[0] <= 1.0);
                // Value should be formatted with 3 decimal places
                assert_eq!(value.matches('.').count(), 1);
                let parts: Vec<&str> = value.split('.').collect();
                assert_eq!(parts[1].len(), 3);
            }
            _ => panic!("Expected RangeFloat result"),
        }
    }
}
