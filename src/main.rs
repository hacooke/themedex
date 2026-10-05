pub mod cli;

use clap::Parser;
use cli::Cli;
use themedex::{
    common::ThemedexError,
    config::{ThemedexConfig, get_config_file},
};

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    // TODO: implement Cli::get_config to generate a get_config
    // * Argument to set config location, else look in a default
    // * Arguments to override config options

    // let config = ThemedexConfig {
    //     directories: DirectoryConfig {
    //         wallpaper_directory: "tests/tempwalls".into(),
    //         database_directory: "tests/temp.db".into(),
    //         template_directory: "tests/templates_tmp".into(),
    //         rendered_config_directory: "tests/rendered".into(),
    //     },
    // };
    let config_file = match get_config_file(cli.config, Default::default()) {
        Ok(config_file) => {
            println!("Using config file at location {}", config_file.display());
            config_file
        }
        Err(err) => {
            eprintln!("Could not resolve config path: {}", err.user_message());
            return;
        }
    };

    let config = match ThemedexConfig::from_file(config_file) {
        Ok(config) => config,
        Err(ThemedexError::Io(err)) if err.kind() == std::io::ErrorKind::NotFound => {
            Default::default()
        }
        Err(err) => {
            eprintln!("Could not parse config file: {}", err,);
            return;
        }
    };
    cli.command.execute(&config);
}

// themedex
// Wallpaper and colour scheme manager
//
// themedex ls
// -> list wallpapers with their corresponding themes
// themedex add <path-to-wallpaper> <colour-scheme>
// -> add a new wallpaper with the specified theme
// themedex add-theme <name> <colour-spec>
//
//
