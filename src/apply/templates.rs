use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use minijinja::{Environment, Value, context, path_loader};

use crate::{
    common::{ThemedexError, internal_error, models::SchemeJson, path::PathComponents},
    config::ThemedexConfig,
};

const TEMPLATE_EXTENSION: &str = "tdx";

fn get_all_templates(
    config: &ThemedexConfig,
) -> Result<impl Iterator<Item = PathBuf>, ThemedexError> {
    Ok(
        fs::read_dir(&config.directories.template_directory)?.filter_map(|file_result| {
            let path = file_result.ok()?.path();
            path.file_name()?.to_str()?.contains(".tdx").then_some(path)
        }),
    )
}

pub fn apply_all_templates(
    color_scheme: &SchemeJson,
    config: &ThemedexConfig,
) -> Result<(), ThemedexError> {
    for template_path in get_all_templates(config)? {
        apply_template(template_path, color_scheme, config)?;
    }
    Ok(())
}

pub fn validate_all_templates(config: &ThemedexConfig) -> Result<(), ThemedexError> {
    for template_path in get_all_templates(config)? {
        validate_template(template_path, config)?;
    }
    Ok(())
}

pub fn apply_template(
    template_file_path: impl AsRef<Path>,
    color_scheme: &SchemeJson,
    config: &ThemedexConfig,
) -> Result<(), ThemedexError> {
    let paths = TemplatePaths::new(template_file_path, config)?;
    let rendered = jinja_apply_template(&paths, color_scheme)?;
    let mut rendered_file = File::create(paths.rendered_file_path)?;
    rendered_file.write_all(rendered.as_bytes())?;
    Ok(())
}

/// Wrapper for the minijinja semantics of applying template
///
/// Applies variables from `color_scheme` to the template specified by `paths`.
/// Returns a [`String`] with the content of the rendered template.
fn jinja_apply_template(
    paths: &TemplatePaths,
    color_scheme: &SchemeJson,
) -> Result<String, ThemedexError> {
    let mut env = Environment::new();
    env.set_undefined_behavior(minijinja::UndefinedBehavior::SemiStrict);
    env.set_loader(path_loader(&paths.directory));
    let template = env.get_template(&paths.template_file_name)?;
    let available_variables = get_variables(color_scheme);
    Ok(template.render(available_variables)?)
}

/// Validate template
///
/// Ensures that a template will work with any config, i.e. works with a
/// minimal config with only non-optional values provided.
pub fn validate_template(
    template_file_path: impl AsRef<Path>,
    config: &ThemedexConfig,
) -> Result<(), ThemedexError> {
    let paths = TemplatePaths::new(template_file_path, config)?;
    let minimal_scheme = SchemeJson::default();
    jinja_apply_template(&paths, &minimal_scheme).map(|_| ())
}

/// Defines mapping of `color_scheme` to available template variables.
///
/// Sections `palette`, `extra`, and `tools` of `color_scheme` are available as
/// namespaces in templates. `name` is maps to the name field of the scheme.
/// Additional aliases are defined as shortcuts to colors defined in the scheme
/// with sensible defaults.
fn get_variables(color_scheme: &SchemeJson) -> Value {
    context! {
        palette => &color_scheme.palette,
        extra => &color_scheme.extra,
        tools => &color_scheme.tools,
        name => &color_scheme.name,
        // Aliases:
        background => color_scheme.background(),
        lighter_background => color_scheme.lighter_background(),
        selection_background => color_scheme.selection_background(),
        dark_foreground => color_scheme.dark_foreground(),
        foreground => color_scheme.foreground(),
        light_foreground => color_scheme.light_foreground(),
        light_background => color_scheme.light_background(),
    }
}

struct TemplatePaths {
    directory: String,
    template_file_name: String,
    rendered_file_path: String,
}

impl TemplatePaths {
    fn new(
        template_file: impl AsRef<Path>,
        config: &ThemedexConfig,
    ) -> Result<Self, ThemedexError> {
        let components = PathComponents::new(template_file)?;
        if components
            .extensions
            .first()
            .is_none_or(|ext| ext != TEMPLATE_EXTENSION)
        {
            return Err(internal_error(
                "Expect .tdx as first extension for Themedex templates.",
            ));
        }
        let rendered_file_name = std::iter::once(&components.base_name)
            .chain(components.extensions.iter().skip(1))
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
            .join(".");
        let rendered_file_path = config
            .directories
            .rendered_config_directory
            .join(rendered_file_name)
            .to_str()
            .map(|s| s.to_owned())
            .ok_or(internal_error("Could not calculate rendered file path"))?;
        Ok(Self {
            directory: components.directory,
            template_file_name: std::iter::once(&components.base_name)
                .chain(components.extensions.iter())
                .map(|s| s.as_str())
                .collect::<Vec<&str>>()
                .join("."),
            rendered_file_path,
        })
    }
}

#[cfg(test)]
mod tests;
