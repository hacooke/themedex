use std::{fs::File, io::Write, path::Path};

use minijinja::{Environment, Value, context, path_loader};

use crate::{common::ThemedexError, database::models::SchemeJson};

pub fn apply_template(template_file_path: &str, color_scheme: &SchemeJson) -> Result<(), ThemedexError> {
    let mut env = Environment::new();
    let paths = TemplatePaths::new(template_file_path).ok_or_else(|| {
        ThemedexError::Internal(format!(
            "Could not digest template path: {}",
            template_file_path
        ))
    })?;
    env.set_loader(path_loader(paths.directory));
    let template = env.get_template(&paths.full_file_name)?;
    let available_variables = get_variables(color_scheme);
    let rendered = template.render(available_variables)?;
    let mut rendered_file = File::create(paths.new_file_path)?;
    rendered_file.write_all(rendered.as_bytes())?;
    Ok(())
}

fn get_variables(color_scheme: &SchemeJson) -> Value {
    return context! {
        palette => &color_scheme.palette,
        extra => &color_scheme.extra,
        tools => &color_scheme.tools,
        name => &color_scheme.name,
        background => "red",
        foreground => "blue",
    };
}

struct TemplatePaths {
    directory: String,
    full_file_name: String,
    new_file_path: String,
}

impl TemplatePaths {
    fn new(template_file: &str) -> Option<Self> {
        let full_path = Path::new(template_file);
        let extension = full_path.extension()?;
        Some(Self {
            directory: full_path.parent()?.to_str()?.into(),
            full_file_name: full_path.file_name()?.to_str()?.into(),
            new_file_path: Path::new(full_path.file_stem()?).with_extension(extension).to_str()?.into(),
        })
    }
}

#[cfg(test)]
mod tests;
