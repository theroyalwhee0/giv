use super::Commands;
use clap::Parser;

/// Build the long version string with repository and package links.
///
/// # Panics
///
/// Panics if CARGO_PKG_VERSION, CARGO_PKG_REPOSITORY, CARGO_PKG_NAME, BUILD_DATETIME_ISO, or EXPECT_PROFILE environment variables are not set at compile time.
const fn long_version() -> &'static str {
    concat!(
        // The version.
        env!("CARGO_PKG_VERSION"),
        "\n\n",
        // Build information.
        "Build Date:    ",
        env!("BUILD_DATETIME_ISO"),
        "\n",
        "Build Profile: ",
        env!("EXPECT_PROFILE"),
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
    /// Copy output to clipboard (still prints to stdout).
    #[cfg(feature = "clipboard")]
    #[arg(short = 'c', long, default_value_t = false)]
    pub clip: bool,

    /// Format the output as JSON.
    #[cfg(feature = "json")]
    #[arg(short, long, default_value_t = false)]
    pub json: bool,

    /// The available commands.
    #[command(subcommand)]
    pub command: Commands,
}
