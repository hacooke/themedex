pub mod cli;

use clap::Parser;
use cli::Cli;
use themedex::{
    common::ThemedexError,
    config::{ThemedexConfig, get_config_file},
};

// TODO add highlight feature -- highlight configurable outside of colour scheme (just number 0-23?)
// Where stored? Not in DB, just transient for current theme application?
// Could add to config, allow override at CLI, easily passed to all methods?
// Add new kwarg to jinja templates -- highlight is var from config, else extra.highlight (default
// set at color scheme level) else a fixed palette color

// TODO add cli override of individual config options?

fn main() {
    env_logger::init();
    let cli = Cli::parse();
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
