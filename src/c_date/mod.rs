/// Date format enumeration.
mod date_format;
/// Date kind enumeration.
mod date_kind;
/// Output formatting for date command.
mod output;

pub use date_format::DateFormat;
pub use date_kind::DateKind;

use crate::{app::AppContext, error::GivError};
use chrono::{DateTime, SecondsFormat, Utc};
use output::DateOutput;

/// Get the date format based on the kind and format options.
///
/// # Arguments
/// - `kind` - The kind of date.
/// - `format` - The format option.
///
/// # Returns
///
/// The date format.
fn get_date_format(kind: &DateKind, format: Option<DateFormat>) -> DateFormat {
    match format {
        Some(format) => format,
        None => match kind {
            // Now defaults to a Rfc3339 format.
            DateKind::Now => DateFormat::Rfc3339,
            // Timestamp defaults to a Timestamp format.
            DateKind::Timestamp => DateFormat::Timestamp,
            // Days default to a Rfc3339Date format.
            DateKind::Today => DateFormat::Rfc3339Date,
            DateKind::Tomorrow => DateFormat::Rfc3339Date,
            DateKind::Yesterday => DateFormat::Rfc3339Date,
        },
    }
}

/// Get the date time based on the kind.
///
/// # Arguments
///
/// - `date` The the current date.
/// - `kind` The kind of date.
///
/// # Returns
///
/// The date time.
fn get_date_time(date: DateTime<Utc>, kind: &DateKind) -> chrono::DateTime<Utc> {
    match kind {
        DateKind::Now => date,
        DateKind::Today => date,
        DateKind::Timestamp => date,
        DateKind::Tomorrow => date + chrono::Duration::days(1),
        DateKind::Yesterday => date + chrono::Duration::days(-1),
    }
}

/// Format the date time based on the specified format.
///
/// # Arguments
///
/// - `date` The date to format.
/// - `format` The format to use.
///
/// # Returns
///
/// The formatted date as a string.
fn format_date_time(date: &DateTime<Utc>, format: &DateFormat) -> String {
    match format {
        // RFC 3339 format with milliseconds '2025-04-17T15:14:12.748Z'
        DateFormat::Rfc3339 => date.to_rfc3339_opts(SecondsFormat::Millis, true),
        // The RFC 3339 format with just the date '2025-04-17'
        DateFormat::Rfc3339Date => {
            let formatted = date.to_rfc3339_opts(SecondsFormat::Millis, true);
            // Get everything before the 'T'.
            formatted.split('T').next().unwrap().to_string()
        }
        // The RFC 3339 format with just the time '15:14:12.748Z'
        DateFormat::Rfc3339Time => {
            let formatted = date.to_rfc3339_opts(SecondsFormat::Millis, true);
            // Get everything after the 'T' including the 'Z'.
            formatted.split('T').nth(1).unwrap().to_string()
        }
        // Unix Timestamp '1744902865'
        DateFormat::Timestamp => date.timestamp().to_string(),
        // Unix Timestamp in milliseconds '1744902874772'
        DateFormat::TimestampMs => date.timestamp_millis().to_string(),
        // RFC 2822 format 'Fri, 17 Apr 2025 15:14:12 +0000'
        DateFormat::Rfc2882 => date.to_rfc2822(),
    }
}

/// Handle the date command
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
pub fn date_command(
    kind: DateKind,
    format: Option<DateFormat>,
    ctx: &mut AppContext,
) -> Result<(), GivError> {
    // Get the current time.
    let now = Utc::now();

    // Get the specified date.
    let date = get_date_time(now, &kind);

    // Get the date format, defaulting if not specified.
    let format = get_date_format(&kind, format);

    // Format the current time based on the specified format.
    let formatted = format_date_time(&date, &format);

    // Create output with the formatted date.
    let output = DateOutput { date: formatted };

    // Output the formatted date.
    ctx.output().output(&output);

    // Success.
    Ok(())
}

// Tests.
#[cfg(test)]
mod tests {
    use super::*;

    /// Test `get_date_time`.
    #[test]
    fn test_get_date_time() {
        // Use a fixed date for consistent testing.
        let date = DateTime::parse_from_rfc3339("2023-05-15T10:30:15.123Z")
            .unwrap()
            .with_timezone(&Utc);

        // Test Now, Timestamp and Today returns current time.
        let now_result = get_date_time(date, &DateKind::Now);
        assert_eq!(date, now_result);
        let timestamp_result = get_date_time(date, &DateKind::Timestamp);
        assert_eq!(date, timestamp_result);
        let today_result = get_date_time(date, &DateKind::Today);
        assert_eq!(date, today_result);

        // Test Yesterday is 24 hours before.
        let yesterday_result = get_date_time(date, &DateKind::Yesterday);
        assert_eq!(date + chrono::Duration::days(-1), yesterday_result);

        // Test Tomorrow is 24 hours after.
        let tomorrow_result = get_date_time(date, &DateKind::Tomorrow);
        assert_eq!(date + chrono::Duration::days(1), tomorrow_result);
    }

    /// Test `get_date_format`.
    #[test]
    fn test_get_date_format() {
        // Test explicit format overrides defaults.
        assert_eq!(
            get_date_format(&DateKind::Now, Some(DateFormat::Timestamp)),
            DateFormat::Timestamp
        );

        // Test defaulted formats.
        assert_eq!(get_date_format(&DateKind::Now, None), DateFormat::Rfc3339);
        assert_eq!(
            get_date_format(&DateKind::Timestamp, None),
            DateFormat::Timestamp
        );
        assert_eq!(
            get_date_format(&DateKind::Today, None),
            DateFormat::Rfc3339Date
        );
        assert_eq!(
            get_date_format(&DateKind::Tomorrow, None),
            DateFormat::Rfc3339Date
        );
        assert_eq!(
            get_date_format(&DateKind::Yesterday, None),
            DateFormat::Rfc3339Date
        );
    }

    /// Test `format_date_time`.
    #[test]
    fn test_format_date_time() {
        // Use a fixed date for consistent testing.
        let date = DateTime::parse_from_rfc3339("2023-05-15T10:30:15.123Z")
            .unwrap()
            .with_timezone(&Utc);

        // Test RFC 3339 format.
        assert_eq!(
            format_date_time(&date, &DateFormat::Rfc3339),
            "2023-05-15T10:30:15.123Z"
        );

        // Test RFC 3339 Date format.
        assert_eq!(
            format_date_time(&date, &DateFormat::Rfc3339Date),
            "2023-05-15"
        );

        // Test RFC 3339 Time format.
        assert_eq!(
            format_date_time(&date, &DateFormat::Rfc3339Time),
            "10:30:15.123Z"
        );

        // Test Timestamp format.
        assert_eq!(
            format_date_time(&date, &DateFormat::Timestamp),
            "1684146615"
        );

        // Test Timestamp in milliseconds format.
        assert_eq!(
            format_date_time(&date, &DateFormat::TimestampMs),
            "1684146615123"
        );

        // Test RFC 2822 format.
        assert_eq!(
            format_date_time(&date, &DateFormat::Rfc2882),
            "Mon, 15 May 2023 10:30:15 +0000"
        );
    }
}
