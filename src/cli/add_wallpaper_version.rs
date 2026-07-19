use clap::Args;
use themedex::{common::ThemedexError, config::ThemedexConfig, database::utils::ThemedexDb, import::ImportAgent};

#[derive(Args, Debug)]
pub struct AddWallpaperVersionArgs {
    #[arg(help = "Path to image to be used as a wallpaper")]
    path: String,
    #[arg(help = "Name of existing wallpaper of which this is a variant")]
    wallpaper: String,
    #[arg(help = "Name of color scheme to attach this wallpaper version to")]
    color_scheme: String,
    #[arg(
        long,
        action,
        help = "Replace the existing image for this wallpaper and color scheme if it exists"
    )]
    update: bool,
}

pub fn execute(args: &AddWallpaperVersionArgs, config: &ThemedexConfig) -> Result<(), ThemedexError> {
    println!("{:?}", args);
    let agent = ImportAgent {
        wallpaper_directory: config.wallpaper_directory.clone(),
        conn: ThemedexDb::new(&config.database_directory),
    };
    agent.import_wallpaper_version_by_wall_name(&args.path, &args.wallpaper, &args.color_scheme, args.update)?;
    Ok(())
}
