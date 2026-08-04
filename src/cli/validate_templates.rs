use clap::Args;
use themedex::{apply::validate_all_templates, common::ThemedexError, config::ThemedexConfig};

#[derive(Args, Debug)]
pub struct ValidateTemplatesArgs {}

pub fn execute(args: &ValidateTemplatesArgs, config: &ThemedexConfig) -> Result<(), ThemedexError> {
    println!("{:?} {:?}", args, config);
    validate_all_templates(config)?;
    Ok(())
}
