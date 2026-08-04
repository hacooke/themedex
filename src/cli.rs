pub mod add_color_scheme;
pub mod add_wallpaper;
pub mod add_wallpaper_version;
pub mod list_color_schemes;
pub mod list_wallpaper_versions;
pub mod list_wallpapers;
pub mod validate_templates;

use clap::{Parser, Subcommand};

use add_color_scheme::AddSchemeArgs;
use add_wallpaper::AddWallpaperArgs;
use add_wallpaper_version::AddWallpaperVersionArgs;
use themedex::config::ThemedexConfig;

use crate::cli::{
    list_color_schemes::ListColorSchemesArgs, list_wallpaper_versions::ListWallpaperVersionsArgs,
    list_wallpapers::ListWallpapersArgs, validate_templates::ValidateTemplatesArgs,
};

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: ThemedexCommands,
}

#[derive(Subcommand)]
pub enum ThemedexCommands {
    AddColorScheme(AddSchemeArgs),
    AddWallpaper(AddWallpaperArgs),
    AddWallpaperVersion(AddWallpaperVersionArgs),
    ListColorSchemes(ListColorSchemesArgs),
    ListWallpapers(ListWallpapersArgs),
    #[command(visible_alias = "ls")]
    ListWallpaperVersions(ListWallpaperVersionsArgs),
    ValidateTemplates(ValidateTemplatesArgs),
}

impl ThemedexCommands {
    pub fn execute(&self, config: &ThemedexConfig) {
        let result = match self {
            Self::AddColorScheme(args) => add_color_scheme::execute(args, config),
            Self::AddWallpaper(args) => add_wallpaper::execute(args, config),
            Self::AddWallpaperVersion(args) => add_wallpaper_version::execute(args, config),
            Self::ListColorSchemes(args) => list_color_schemes::execute(args, config),
            Self::ListWallpapers(args) => list_wallpapers::execute(args, config),
            Self::ListWallpaperVersions(args) => list_wallpaper_versions::execute(args, config),
            Self::ValidateTemplates(args) => validate_templates::execute(args, config),
        };
        if let Err(error) = result {
            println!("Action failed: {}", error.user_message())
        }
    }
}
