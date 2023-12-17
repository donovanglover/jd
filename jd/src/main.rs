use clap::Parser;
use cli::{Cli, Commands};

mod cli;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add { name: _ }) => {}

        Some(Commands::Remove { name: _ }) => {}

        Some(Commands::Index { name: _ }) => {}

        Some(Commands::Serve {}) => {
            // server::init();
        }

        None => {},

        _ => todo!(),
    }
}
