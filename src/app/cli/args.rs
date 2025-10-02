use super::Commands;
use clap::Parser;

/// Build the long version string with repository and package links.
///
/// # Panics
///
/// Panics if CARGO_PKG_VERSION, CARGO_PKG_REPOSITORY, or CARGO_PKG_NAME environment variables are not set at compile time.
const fn long_version() -> &'static str {
    concat!(
        // The version.
        env!("CARGO_PKG_VERSION"),
        "\n\n",
        // The Git Repository.
        "Repository:    ",
        env!("CARGO_PKG_REPOSITORY"),
        "/tree/v",
        env!("CARGO_PKG_VERSION"),
        "\n",
        // The crates.io link.
        "Crate:         ",
        "https://crates.io/crates/",
        env!("CARGO_PKG_NAME"),
        "/",
        env!("CARGO_PKG_VERSION"),
        "\n",
        // The docs.rs link.
        "Documentation: ",
        "https://docs.rs/",
        env!("CARGO_PKG_NAME"),
        "/",
        env!("CARGO_PKG_VERSION")
    )
}

/// A command-line utility generating values.
#[derive(Parser)]
#[command(author, version, about, long_about = None, long_version = long_version())]
pub struct Cli {
    /// Format the output as JSON.
    #[cfg(feature = "json")]
    #[arg(short, long, default_value_t = false)]
    pub json: bool,

    /// The available commands.
    #[command(subcommand)]
    pub command: Commands,
}
