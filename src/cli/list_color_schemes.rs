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
pub struct ListColorSchemesArgs {
    #[arg(help = "Part of color scheme name to search for, lists all if omitted")]
    name: Option<String>,
}

pub fn execute(args: &ListColorSchemesArgs, config: &ThemedexConfig) -> Result<(), ThemedexError> {
    println!("{:?} {:?}", args, config);
    let db = ThemedexDb::new(&config.directories.database_path);
    let wallpapers = query::list_color_scheme_names(args.name.as_deref(), db.conn())?;
    for name in wallpapers {
        println!("{}", name)
    }
    Ok(())
}
