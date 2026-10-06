use clap::Args;
use themedex::{
    common::ThemedexError,
    config::ThemedexConfig,
    database::{
        query,
        utils::{HasConnection, ThemedexDb},
    },
};

#[derive(Args, Debug)]
pub struct ListWallpaperVersionsArgs {
    #[arg(long, help = "Wallpaper name")]
    wallpaper: Option<String>,
    #[arg(long, help = "Color scheme name")]
    color_scheme: Option<String>,
}

pub fn execute(
    args: &ListWallpaperVersionsArgs,
    config: &ThemedexConfig,
) -> Result<(), ThemedexError> {
    println!("{:?} {:?}", args, config);
    let db = ThemedexDb::new(&config.directories.database_path);
    let wallpaper_versions = query::list_wallpaper_version_names(
        args.wallpaper.as_deref(),
        args.color_scheme.as_deref(),
        db.conn(),
    )?;
    for version in wallpaper_versions {
        println!("{} > {}", version.wallpaper_name, version.color_scheme_name)
    }
    Ok(())
}
