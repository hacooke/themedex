use std::fs;

use super::*;
use crate::{
    common::{
        ThemedexError,
        test_fixtures::{TestDir, catppuccin_mocha_json, make_scheme, tokyonight_storm_json},
    },
    config::test_fixtures::make_config,
};

fn template_paths_from_string(
    dir: &TestDir,
    raw_template: impl AsRef<str>,
) -> Result<TemplatePaths, ThemedexError> {
    let template_file = template_file_from_string(dir, raw_template)?;
    TemplatePaths::new(template_file, &make_config())
}

fn template_file_from_string(
    dir: &TestDir,
    raw_template: impl AsRef<str>,
) -> Result<PathBuf, ThemedexError> {
    let template_file = dir.path.join("temp.tdx.something");
    std::fs::write(&template_file, raw_template.as_ref())?;
    Ok(template_file)
}

#[test]
fn test_template_paths_with_ext() -> Result<(), ThemedexError> {
    // Arrange
    let mut config = make_config();
    config.directories.rendered_config_directory = "output_dir_here".into();
    let template_file = "templates_dir/test-file.tdx.something";
    // Act
    let paths = TemplatePaths::new(template_file, &config)?;
    // Assert
    assert_eq!(paths.directory, "templates_dir");
    assert_eq!(paths.template_file_name, "test-file.tdx.something");
    assert_eq!(
        paths.rendered_file_path,
        "output_dir_here/test-file.something"
    );
    Ok(())
}

#[test]
fn test_template_paths_no_ext() -> Result<(), ThemedexError> {
    // Arrange
    let mut config = make_config();
    config.directories.rendered_config_directory = "output_dir_here".into();
    let template_file = "templates_dir/.vimrc.tdx";
    // Act
    let paths = TemplatePaths::new(template_file, &config)?;
    // Assert
    assert_eq!(paths.directory, "templates_dir");
    assert_eq!(paths.template_file_name, ".vimrc.tdx");
    assert_eq!(paths.rendered_file_path, "output_dir_here/.vimrc");
    Ok(())
}

#[test]
fn test_template_path_missing_tdx() -> Result<(), ThemedexError> {
    // Arrange
    let template_file = "templates_dir/test-file.something";
    // Act
    let paths = TemplatePaths::new(template_file, &make_config());
    // Assert
    match paths {
        Err(e) if e.to_string().contains(".tdx") => Ok(()),
        _ => Err(internal_error("Did not raise error for missing .tdx")),
    }
}

#[test]
fn test_template_path_wrong_ext() -> Result<(), ThemedexError> {
    // Arrange
    let template_file = "templates_dir/test-file.one.tdx";
    // Act
    let paths = TemplatePaths::new(template_file, &make_config());
    // Assert
    match paths {
        Err(e) if e.to_string().contains(".tdx") => Ok(()),
        _ => Err(internal_error("Did not raise error for missing .tdx")),
    }
}

#[test]
fn test_apply_template_end_to_end_catppuccin() -> Result<(), ThemedexError> {
    // Arrange
    let template_file = "tests/templates/simple.tdx.css";
    let expected_result = fs::read_to_string("tests/templates/simple.catppuccin-mocha.css")?;
    let color_scheme = catppuccin_mocha_json();
    // > set output directory (temp directory) via config
    let tmp_dir = TestDir::new("test_apply_template_end_to_end_catppuccin");
    let mut config = make_config();
    config.directories.rendered_config_directory = tmp_dir.path.clone();
    // Act
    apply_template(template_file, &color_scheme, &config)?;
    // Assert
    let result = fs::read_to_string(tmp_dir.path.join("simple.css")).unwrap();
    assert_eq!(result.trim(), expected_result.trim());
    Ok(())
}

#[test]
fn test_apply_template_end_to_end_tokyonight() -> Result<(), ThemedexError> {
    // Arrange
    let template_file = "tests/templates/simple.tdx.css";
    let expected_result = fs::read_to_string("tests/templates/simple.tokyo-night-storm.css")?;
    let color_scheme = tokyonight_storm_json();
    // > set output directory (temp directory) via config
    let tmp_dir = TestDir::new("test_apply_template_end_to_end_tokyonight");
    let mut config = make_config();
    config.directories.rendered_config_directory = tmp_dir.path.clone();
    // Act
    apply_template(template_file, &color_scheme, &config)?;
    // Assert
    let result = fs::read_to_string(tmp_dir.path.join("simple.css")).unwrap();
    assert_eq!(result.trim(), expected_result.trim());
    Ok(())
}

#[test]
fn test_template_aliases() -> Result<(), ThemedexError> {
    // Arrange
    let tmp_dir = TestDir::new("test_template_aliases");
    let paths = template_paths_from_string(
        &tmp_dir,
        "01:{{undefined or palette.color01}},02:{{foreground}},03:{{background}}",
    )?;
    let mut scheme = make_scheme();
    scheme.extra.insert("foreground".into(), "#ffffff".into());
    // Act
    let rendered = jinja_apply_template(&paths, &scheme)?;
    // Assert
    assert_eq!(rendered, "01:#000001,02:#ffffff,03:#000000");
    Ok(())
}

#[test]
fn test_template_defaults() -> Result<(), ThemedexError> {
    // Arrange
    let tmp_dir = TestDir::new("test_template_defaults");
    let paths = template_paths_from_string(
        &tmp_dir,
        "01:{{tools.testtool or palette.color00}},02:{{tools.nothing or palette.color00}}\
        ,03:{{extra.something_blue or palette.color00}},04:{{extra.nothing or palette.color00}}\
        ,05:{{palette.color17 or palette.color00}}",
    )?;
    let mut scheme = make_scheme();
    scheme
        .tools
        .insert("testtool".into(), "my_test_tool".into());
    scheme
        .extra
        .insert("something_blue".into(), "#0000ff".into());
    // Act
    let rendered = jinja_apply_template(&paths, &scheme)?;
    // Assert
    assert_eq!(
        rendered,
        "01:my_test_tool,02:#000000,03:#0000ff,04:#000000,05:#000017"
    );
    Ok(())
}

#[test]
fn test_validate_template_with_fallbacks() -> Result<(), ThemedexError> {
    // Arrange
    let tmp_dir = TestDir::new("test_validate_template_with_fallbacks");
    let paths = template_file_from_string(
        &tmp_dir,
        "01:{{foreground}},02:{{extra.anything or background}},03:{{palette.color17 or palette.color00}}",
    )?;
    // Act
    validate_template(&paths, &make_config())
}

#[test]
fn test_validate_template_undefined_variable() -> Result<(), ThemedexError> {
    // Arrange
    let tmp_dir = TestDir::new("test_validate_template_invalid");
    let paths = template_file_from_string(&tmp_dir, "{{undefined}}")?;
    // Act
    validate_template(&paths, &make_config()).expect_err("");
    Ok(())
}

#[test]
fn test_validate_template_non_base16_color() -> Result<(), ThemedexError> {
    // Arrange
    let tmp_dir = TestDir::new("test_validate_template_non_base16_color");
    let paths = template_file_from_string(&tmp_dir, "{{palette.color16}}")?;
    // Act
    validate_template(&paths, &make_config()).expect_err("");
    Ok(())
}

#[test]
fn test_validate_template_non_defaulted_extra() -> Result<(), ThemedexError> {
    // Arrange
    let tmp_dir = TestDir::new("test_validate_template_non_defaulted_extra");
    let paths = template_file_from_string(&tmp_dir, "{{extra.foreground}}")?;
    // Act
    validate_template(&paths, &make_config()).expect_err("");
    Ok(())
}
