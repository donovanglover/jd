use clap::Parser;
use cli::{Cli, Commands};

mod cli;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add { name }) => {
            println!("{:?}", name);
        },

        Some(Commands::Remove { name }) => {
            println!("{:?}", name)
        },

        Some(Commands::Ls { name }) => {
            jd::list_areas();
        },

        None => {
            jd::list_areas();
        },

        _ => todo!()
    }
}
