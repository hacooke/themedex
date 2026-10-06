use clap::Args;
use themedex::{
    apply::{
        post_change_commands::run_post_change_commands, templates::apply_all_templates,
        wallpapers::apply_wallpaper,
    },
    common::ThemedexError,
    config::ThemedexConfig,
    database::{
        interfaces::TableWithName,
        models::{ColorScheme, Wallpaper, WallpaperVersion},
        utils::{HasConnection, ThemedexDb},
    },
};

#[derive(Args, Debug)]
pub struct ApplyThemeArgs {
    #[arg(help = "Name of wallpaper to apply")]
    wallpaper: String,
    #[arg(help = "Name of color scheme to apply")]
    color_scheme: String,
}

pub fn execute(args: &ApplyThemeArgs, config: &ThemedexConfig) -> Result<(), ThemedexError> {
    println!("{:?}", args);
    let conn = ThemedexDb::new(&config.directories.database_path);
    let Ok(wallpaper) = Wallpaper::select_by_name(&args.wallpaper, conn.conn()) else {
        return Err(ThemedexError::Internal(format!(
            "Wallpaper {} does not exist.",
            args.wallpaper
        )));
    };
    let Ok(color_scheme) = ColorScheme::select_by_name(&args.color_scheme, conn.conn()) else {
        return Err(ThemedexError::Internal(format!(
            "Color scheme {} does not exist.",
            args.color_scheme
        )));
    };
    let version =
        WallpaperVersion::from_wallpaper_and_color_scheme(&wallpaper, &color_scheme, conn.conn())?;
    apply_wallpaper(&wallpaper, &version, config)?;
    apply_all_templates(&color_scheme.colors, config)?;
    println!("Running commands");
    run_post_change_commands(config)?;
    Ok(())
}
