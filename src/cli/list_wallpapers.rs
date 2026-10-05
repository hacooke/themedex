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
pub struct ListWallpapersArgs {
    #[arg(help = "Part of wallpaper name to search for, lists all if omitted")]
    name: Option<String>,
}

pub fn execute(args: &ListWallpapersArgs, config: &ThemedexConfig) -> Result<(), ThemedexError> {
    println!("{:?} {:?}", args, config);
    let db = ThemedexDb::new(&config.directories.database_path);
    let wallpapers = query::list_wallpaper_names(args.name.as_deref(), db.conn())?;
    for name in wallpapers {
        println!("{}", name)
    }
    Ok(())
}
