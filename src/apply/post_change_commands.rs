use std::process::Command;

use crate::{common::ThemedexError, config::ThemedexConfig};

pub fn run_post_change_commands(config: &ThemedexConfig) -> Result<(), ThemedexError> {
    let outputs = config
        .scheme_change_commands
        .iter()
        .filter_map(|command_vec| {
            let mut iter = command_vec.iter();
            iter.next().map(|cmd| Command::new(cmd).args(iter).output())
        })
        .collect::<Result<Vec<_>, _>>()?;
    for output in outputs {
        println!(
            "Ran command, got output {} {} {}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}
