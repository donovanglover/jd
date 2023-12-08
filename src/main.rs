use clap::Parser;
use cli::{Cli, Commands};

mod cli;
mod server;

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

        Some(Commands::Serve {}) => {
            server::init();
        },

        None => {
            jd::list_areas();
        },

        _ => todo!()
    }
}
