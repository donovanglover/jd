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
    #[arg(short, long, default_value = "$HOME")]
    pub dir: PathBuf,

    /// Print debugging information
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}


#[derive(Subcommand)]
pub enum Commands {
    /// Add a new area, category, or id.
    Add {
        name: PathBuf,
    },

    /// Remove an existing area, category, or id.
    Remove {
        name: PathBuf,
    },

    /// List the contents of an area, category, or id.
    Ls {
        name: PathBuf,
    }
}
