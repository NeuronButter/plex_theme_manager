use clap::{Parser, Subcommand};
use std::panic;

mod commands;
mod lib;

#[derive(Parser)]
#[command(name = "Plex Theme Manager",author = "Neeron Bhatta", version, about = "Manage your Plex theme songs", long_about = None)]
struct Cli {
    /// Config file path
    #[arg(long)]
    config: Option<String>,

    /// Overwrite existing files or links
    #[arg(short, long)]
    force: bool,

    /// A command to perform.
    #[command(subcommand)]
    command: CliCommands,
}

#[derive(Subcommand)]
enum CliCommands {
    /// Update theme songs into media folders. (Theme songs will be derived first)
    Update {},

    /// Derive theme songs into main folder
    Derive {},

    /// Delete theme songs in media folders
    Delete {},
}

fn main() {
    panic::set_hook(Box::new(|_| {
        println!();
    }));

    let cli = Cli::parse();
    let config: lib::config::Config;
    match cli.config {
        Some(path) => {
            let parsed = lib::config::get_config(path);
            match parsed {
                Ok(object) => config = object,

                Err(_) => {
                    panic!("Unable to parse JSON. Is your config.json file correctly formatted?")
                }
            }
        }
        None => {
            panic!("No config file was passed in.");
        }
    }

    match &cli.command {
        CliCommands::Update {} => commands::update::updater(config),
        CliCommands::Derive {} => commands::derive::deriver(config),
        CliCommands::Delete {} => commands::delete::deleter(config),
    }
}
