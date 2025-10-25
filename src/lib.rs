//! Giv - A library for generating useful values.
//!
//! This library provides functionality for generating:
//! - Random bytes with various encodings
//! - Character and emoji conversion
//! - Formatted dates and timestamps
//! - Cryptographically secure random keys
//! - Lorem ipsum placeholder text
//! - UUID v7 identifiers
//! - PI digits with configurable precision
//! - Random numbers using dice notation or ranges
//!
//! Each module corresponds to a feature flag, allowing you to include only
//! the functionality you need. All features are enabled by default.
//!
//! # Examples
//!
//! ```
//! use giv::uuid::generate_uuid;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let uuid = generate_uuid()?;
//! println!("Generated UUID: {}", uuid.uuid);
//! # Ok(())
//! # }
//! ```
//!
//! # Features
//!
//! - `bytes` - Random byte generation
//! - `chars` - Character and emoji conversion
//! - `date` - Date and time formatting
//! - `key` - Random key generation
//! - `lorem` - Lorem ipsum text generation
//! - `pi` - PI digit calculation
//! - `rng` - Random number generation
//! - `uuid` - UUID v7 generation
//! - `json` - JSON output support
//! - `full` (default) - All features enabled

/// Random byte generation module.
#[cfg(feature = "bytes")]
pub mod bytes;

/// Character and emoji conversion module.
#[cfg(feature = "chars")]
pub mod chars;

/// Date and time formatting module.
#[cfg(feature = "date")]
pub mod date;

/// Random key generation module.
#[cfg(feature = "key")]
pub mod key;

/// Lorem ipsum text generation module.
#[cfg(feature = "lorem")]
pub mod lorem;

/// PI digit calculation module.
#[cfg(feature = "pi")]
pub mod pi;

/// Random number generation module.
#[cfg(feature = "rng")]
pub mod rng;

/// UUID v7 generation module.
#[cfg(feature = "uuid")]
pub mod uuid;

/// Build information module.
pub mod build_info;

/// Error types for the library.
pub mod error;

/// Output formatting types and traits.
pub mod output;

// Re-export commonly used types at crate root
pub use error::GivError;
