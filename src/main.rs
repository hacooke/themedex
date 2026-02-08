pub mod database;

pub mod common;

pub mod import;

fn main() {
    database::init_db("test.db").unwrap();
    println!("Hello, world!");
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
