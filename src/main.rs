use clap::Parser;
use cli::{Cli, Commands};

mod cli;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add { name }) => {
            println!("{:?}", name)
        },
        Some(Commands::Remove { name }) => {
            println!("{:?}", name)
        },
        Some(Commands::Ls { name }) => {
            println!("{:?}", name)
        },
        None => {
            println!("none")
        }
    }
}
