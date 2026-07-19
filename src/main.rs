pub mod cli;

use clap::Parser;
use cli::Cli;
use themedex::config::ThemedexConfig;

fn main() {
    let cli = Cli::parse();
    // TODO: implement Cli::get_config to generate a get_config
    // * Argument to set config location, else look in a default
    // * Arguments to override config options
    let config = ThemedexConfig{
        wallpaper_directory: "tests/tempwalls".into(),
        database_directory: "tests/temp.db".into(),
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
