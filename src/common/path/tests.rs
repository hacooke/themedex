use super::*;

#[test]
fn test_path_components() -> Result<(), ThemedexError> {
    let paths = PathComponents::new("/path/to/my/file.tdx.config")?;
    assert_eq!(paths.directory, "/path/to/my");
    assert_eq!(paths.base_name, "file");
    assert_eq!(paths.extensions, ["tdx", "config"]);
    Ok(())
}

#[test]
fn test_path_components_hidden() -> Result<(), ThemedexError> {
    let paths = PathComponents::new("relative/path/.hidden.tdx.config")?;
    assert_eq!(paths.directory, "relative/path");
    assert_eq!(paths.base_name, ".hidden");
    assert_eq!(paths.extensions, ["tdx", "config"]);
    Ok(())
}
