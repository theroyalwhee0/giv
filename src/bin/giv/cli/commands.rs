use clap::Subcommand;
#[cfg(feature = "bytes")]
use giv::bytes::BytesEncoding;
use giv::date::{DateFormat, DateKind};

/// The available commands for the CLI.
#[derive(Subcommand)]
pub enum Commands {
    /// Generate random bytes with specified encoding (hex, base64, raw)
    #[cfg(feature = "bytes")]
    Bytes {
        /// Number of bytes to generate (default: 32)
        #[arg(default_value = None)]
        length: Option<usize>,
        /// Encoding format for output (default: base64)
        #[arg(short, long, value_enum, default_value = None)]
        encoding: Option<BytesEncoding>,
        /// Enable padding for base64 encoding (default: false)
        #[arg(long, default_value_t = false)]
        pad: bool,
    },
    /// Convert emoji shortcodes and character patterns to Unicode characters
    #[cfg(feature = "chars")]
    Chars {
        /// Character patterns or emoji shortcodes to convert
        #[arg(required = true)]
        inputs: Vec<String>,
    },
    /// Generate a random key in the format of `key_<alphanumeric[size]>`
    #[cfg(feature = "key")]
    Key {
        /// Size of the output key in characters (default: 36)
        #[arg(default_value = None)]
        size: Option<usize>,
    },
    /// Generate and display a UUID
    #[cfg(feature = "uuid")]
    Uuid {
        /// UUID version to generate (default: v7)
        #[arg(short, long, value_enum, default_value = None)]
        version: Option<giv::uuid::UuidVersion>,
        /// Formatting style for output (default: standard)
        #[arg(short, long, value_enum, default_value = None)]
        format: Option<giv::uuid::UuidFormat>,
        /// Use uppercase hex digits (default: false)
        #[arg(short, long, default_value_t = false)]
        uppercase: bool,
    },
    /// Pi with the specified number of places.
    #[cfg(feature = "pi")]
    Pi {
        /// Number of decimal places to display. (default: 15)
        #[arg(default_value = None)]
        places: Option<usize>,
        /// Round flag. Use --no-round to negate this. (default: true)
        #[arg(long = "round", conflicts_with = "no_round", action = clap::ArgAction::SetTrue)]
        round: Option<bool>,
        /// No-Round flag. This is the negation of the round flag. (default: false)
        #[arg(long = "no-round", conflicts_with = "round", action = clap::ArgAction::SetTrue, hide= true)]
        no_round: Option<bool>,
    },
    /// Generate a date
    #[cfg(feature = "date")]
    Date {
        /// Date kind.
        #[arg(value_enum)]
        kind: DateKind,
        /// Format.
        #[arg(short, long, value_enum, default_value = None)]
        format: Option<DateFormat>,
    },
    /// Display the current UTC time. This is an alias for `date now`.
    #[cfg(feature = "date")]
    Now {
        /// Format.
        #[arg(short, long, value_enum, default_value = None)]
        format: Option<DateFormat>,
    },
    /// Generate random numbers using dice notation or ranges
    #[cfg(feature = "rng")]
    Rng {
        /// RNG specifications (e.g., '2d6', 'd20', '1..100', '0.0..1.0')
        #[arg(required = true)]
        specs: Vec<String>,
    },
    /// Generate lorem ipsum placeholder text
    #[cfg(feature = "lorem")]
    #[command(alias = "ipsum")]
    Lorem {
        /// Number of units to generate (default: 50)
        #[arg(default_value = None)]
        count: Option<usize>,
        /// Generate words (default if no unit flag is specified)
        #[arg(long, group = "unit")]
        words: bool,
        /// Generate sentences
        #[arg(short, long, group = "unit")]
        sentences: bool,
        /// Generate paragraphs
        #[arg(short, long, group = "unit")]
        paragraphs: bool,
    },
}
