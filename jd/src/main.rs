use jd::list_areas;
use clap::Parser;
use cli::{Cli, Commands};

mod cli;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add { name: _ }) => {}

        Some(Commands::Remove { name: _ }) => {}

        Some(Commands::Ls { name: _ }) => {
            list_areas();
        }

        Some(Commands::Serve {}) => {
            // server::init();
        }

        None => list_areas(),

        _ => todo!(),
    }
}
