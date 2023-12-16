use clap::Parser;
use cli::{Cli, Commands};
use jd::list_areas;

mod cli;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add { name: _ }) => {}

        Some(Commands::Remove { name: _ }) => {}

        Some(Commands::Index { name: _ }) => {
            list_areas();
        }

        Some(Commands::Serve {}) => {
            // server::init();
        }

        None => list_areas(),

        _ => todo!(),
    }
}
