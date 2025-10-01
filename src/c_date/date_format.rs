/// The date formats.
#[derive(clap::ValueEnum, Clone, PartialEq, Eq, Default, Debug)]
pub enum DateFormat {
    /// RFC 3339 format with milliseconds '2025-04-17T15:14:12.748Z'
    #[default]
    Rfc3339,
    /// The RFC 3339 format with just the date '2025-04-17'
    Rfc3339Date,
    /// The RFC 3339 format with just the time '15:14:12.748Z'
    Rfc3339Time,
    /// Unix Timestamp '1744902865'
    Timestamp,
    /// Unix Timestamp in milliseconds '1744902874772'
    TimestampMs,
    /// RFC 2822 format 'Fri, 17 Apr 2025 15:14:12 +0000'
    Rfc2882,
}
