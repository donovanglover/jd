use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

const LONG_ABOUT: &str = "jd is a command line interface for interacting with Johnny Decimal systems.";

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Red.on_default() | Effects::BOLD)
        .usage(AnsiColor::Red.on_default() | Effects::BOLD)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Green.on_default())
}

#[derive(Parser)]
#[command(author, version, about, long_about = LONG_ABOUT, styles = styles())]
pub struct Cli {
    /// Directory where areas are stored.
    #[arg(short, long)]
    pub dir: Option<PathBuf>,

    /// Print debugging information
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new area, category, or id to the filesystem.
    Add { name: Vec<String> },

    /// Remove an existing area, category, or id from the filesystem.
    Rm { name: Vec<String> },

    /// List the contents of the index
    Index {},

    /// Insert a new area, category, or id, moving existing ones.
    ///
    /// NOTE: This is non-trivial to implement since it cascades.
    // Insert { name: String },

    /// Checks for empty folders and missing areas/categories/ids (Prune/Clean)
    ///
    /// NOTE: Could simplify this to a "check" function that validates the index with
    /// the filesystem (performance)
    Clean {},

    /// Start a new web server.
    Serve {},
}
