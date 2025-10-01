use super::Commands;
use clap::Parser;

/// A command-line utility generating values.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Format the output as JSON.
    #[cfg(feature = "json")]
    #[arg(short, long, default_value_t = false)]
    pub json: bool,

    /// The available commands.
    #[command(subcommand)]
    pub command: Commands,
}
