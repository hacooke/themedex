use clap::Args;
use themedex::{
    apply::wallpapers::apply_wallpaper,
    common::ThemedexError,
    config::ThemedexConfig,
    database::{
        interfaces::{DatabaseTable, TableWithName},
        models::{ColorScheme, Wallpaper, WallpaperVersion},
        utils::{HasConnection, ThemedexDb},
    },
};

#[derive(Args, Debug)]
pub struct ApplyWallpaperArgs {
    #[arg(help = "Name of wallpaper to apply")]
    wallpaper: String,
    #[arg(
        help = "Color scheme corresponding to wallpaper version to apply (defaults to the default version for chosen wallpaper)"
    )]
    color_scheme: Option<String>,
}

pub fn execute(args: &ApplyWallpaperArgs, config: &ThemedexConfig) -> Result<(), ThemedexError> {
    println!("{:?}", args);
    let conn = ThemedexDb::new(&config.directories.database_path);
    let Ok(wallpaper) = Wallpaper::select_by_name(&args.wallpaper, conn.conn()) else {
        return Err(ThemedexError::Internal(format!(
            "Wallpaper {} does not exist.",
            args.wallpaper
        )));
    };
    let version = if let Some(color_scheme_name) = &args.color_scheme {
        let Ok(color_scheme) = ColorScheme::select_by_name(color_scheme_name, conn.conn()) else {
            return Err(ThemedexError::Internal(format!(
                "Color scheme {} does not exist.",
                color_scheme_name
            )));
        };
        WallpaperVersion::from_wallpaper_and_color_scheme(&wallpaper, &color_scheme, conn.conn())?
    } else {
        let Some(default_version_id) = wallpaper.default_version_id else {
            return Err(ThemedexError::Internal(format!(
                "Wallpaper {} has no default version.",
                args.wallpaper
            )));
        };
        WallpaperVersion::select_by_id(default_version_id, conn.conn())?
    };
    apply_wallpaper(&wallpaper, &version, config)?;
    Ok(())
}
