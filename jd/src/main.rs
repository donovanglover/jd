use clap::Parser;
use cli::{Cli, Commands};
use jd::System;
use johnnydecimal::{Area, Category, Id};

mod cli;

fn main() {
    let cli = Cli::parse();

    let Ok(mut system) = System::new("./test_systems/empty") else {
        println!("Invalid system");

        return;
    };

    match &cli.command {
        Some(Commands::Add { name }) => {
            let maybe_area_category_or_id = name.join(" ");

            if let Ok(id) = Id::new(&maybe_area_category_or_id) {
                match system.add_id(&id) {
                    Ok(ids) => {
                        dbg!(ids);
                    }

                    Err(message) => {
                        println!("{}", message)
                    }
                }

                return;
            }

            if let Ok(category) = Category::new(&maybe_area_category_or_id) {
                match system.add_category(&category) {
                    Ok(categories) => {
                        dbg!(categories);
                    }

                    Err(message) => {
                        println!("{}", message)
                    }
                }

                return;
            }

            if let Ok(area) = Area::new(&maybe_area_category_or_id) {
                match system.add_area(&area) {
                    Ok(areas) => {
                        dbg!(areas);
                    }

                    Err(message) => {
                        println!("{}", message)
                    }
                }

                return;
            }
        }

        Some(Commands::Rm { name }) => {
            let maybe_area_category_or_id = name.join(" ");

            if let Ok(id) = Id::new(&maybe_area_category_or_id) {
                match system.remove_id(&id) {
                    Ok(ids) => {
                        dbg!(ids);
                    }

                    Err(message) => {
                        println!("{}", message)
                    }
                }

                return;
            }

            if let Ok(category) = Category::new(&maybe_area_category_or_id) {
                match system.remove_category(&category) {
                    Ok(categories) => {
                        dbg!(categories);
                    }

                    Err(message) => {
                        println!("{}", message)
                    }
                }

                return;
            }

            if let Ok(area) = Area::new(&maybe_area_category_or_id) {
                match system.remove_area(&area) {
                    Ok(areas) => {
                        dbg!(areas);
                    }

                    Err(message) => {
                        println!("{}", message)
                    }
                }

                return;
            }
        }

        Some(Commands::Index {}) => {}
        Some(Commands::Clean {}) => {}
        Some(Commands::Serve {}) => {}
        None => {}
    }
}
