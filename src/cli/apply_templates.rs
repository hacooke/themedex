use clap::Args;
use themedex::{
    apply::{post_change_commands::run_post_change_commands, templates::apply_all_templates},
    common::ThemedexError,
    config::ThemedexConfig,
    database::{
        interfaces::TableWithName,
        models::ColorScheme,
        utils::{HasConnection, ThemedexDb},
    },
};

#[derive(Args, Debug)]
pub struct ApplyTemplatesArgs {
    #[arg(help = "Name of color scheme to apply to templates")]
    color_scheme: String,
}

pub fn execute(args: &ApplyTemplatesArgs, config: &ThemedexConfig) -> Result<(), ThemedexError> {
    println!("{:?}", args);
    let conn = ThemedexDb::new(&config.directories.database_path);
    let Ok(color_scheme) = ColorScheme::select_by_name(&args.color_scheme, conn.conn()) else {
        return Err(ThemedexError::Internal(format!(
            "Color scheme {} does not exist.",
            args.color_scheme
        )));
    };
    apply_all_templates(&color_scheme.colors, config)?;
    run_post_change_commands(config)?;
    Ok(())
}
