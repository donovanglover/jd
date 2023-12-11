use clap::Parser;
use cli::{Cli, Commands};

mod cli;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add { name }) => {
            commands::add::with_name(name);
        },

        Some(Commands::Remove { name }) => {
            // commands::remove::exec();
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
