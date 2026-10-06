use clap::Args;
use themedex::{
    common::ThemedexError, config::ThemedexConfig, database::utils::ThemedexDb, import::ImportAgent,
};

#[derive(Args, Debug)]
pub struct AddWallpaperArgs {
    #[arg(help = "Path to image to be used as a wallpaper")]
    path: String,
    #[arg(help = "Name of color scheme to attach this wallpaper version to")]
    color_scheme: String,
    #[arg(
        long,
        help = "Name of wallpaper, defaults to name of image file if not provided"
    )]
    name: Option<String>,
    #[arg(
        long,
        action,
        help = "Replace the existing image for this wallpaper and color scheme if it exists"
    )]
    update: bool,
}

pub fn execute(args: &AddWallpaperArgs, config: &ThemedexConfig) -> Result<(), ThemedexError> {
    println!("{:?} {:?}", args, config);
    let agent = ImportAgent {
        wallpaper_directory: config.directories.wallpaper_directory.clone(),
        conn: ThemedexDb::new(&config.directories.database_path),
    };
    agent.import_wallpaper_and_version(
        &args.path,
        &args.color_scheme,
        args.name.as_deref(),
        args.update,
    )?;
    Ok(())
}
