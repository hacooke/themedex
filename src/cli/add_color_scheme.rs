use clap::Args;
use themedex::{common::ThemedexError, config::ThemedexConfig, database::utils::ThemedexDb, import::ImportAgent};

#[derive(Args, Debug)]
pub struct AddSchemeArgs {
    #[arg(help = "Path to json file containing color scheme specification")]
    path: String,
    #[arg(
        long,
        help = "Name of color scheme, defaults to name of JSON file if not provided"
    )]
    name: Option<String>,
    #[arg(
        long,
        action,
        help = "Replace an existing color scheme of the same name if it exists"
    )]
    update: bool,
}

pub fn execute(args: &AddSchemeArgs, config: &ThemedexConfig) -> Result<(), ThemedexError> {
    println!("{:?}", args);
    let agent = ImportAgent {
        wallpaper_directory: config.wallpaper_directory.clone(),
        conn: ThemedexDb::new(&config.database_directory),
    };
    agent.import_color_scheme_json(&args.path, args.name.as_deref(), args.update)?;
    Ok(())
}
