use std::{iter, path::Path, process::Command};

use crate::{
    common::ThemedexError,
    config::ThemedexConfig,
    database::models::{Wallpaper, WallpaperVersion},
};

// TODO do I need domain models? Weird that I have to pass wallpaper and wallpaper version just to
// get full path to image
// Or: just pass path into apply_wallpaper. Perhaps a DB helper to get full path from DB?

pub fn apply_wallpaper(
    wallpaper: &Wallpaper,
    version: &WallpaperVersion,
    config: &ThemedexConfig,
) -> Result<(), ThemedexError> {
    let wallpaper_path = Path::new(&wallpaper.path)
        .join(&version.filename)
        .to_str()
        .ok_or(ThemedexError::Internal(
            "Could not get wallpaper path in apply_wallpaper".into(),
        ))?
        .to_string();
    let cmd = &config.set_wallpaper_command;
    Command::new(&cmd.executable)
        .args(cmd.arguments.iter().chain(iter::once(&wallpaper_path)))
        .output()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        config::test_fixtures::make_config,
        database::{
            interfaces::DatabaseTable,
            test_structures::{TestDb, create_test_color_scheme},
            utils::HasConnection,
        },
    };

    use super::*;

    #[test]
    fn test_apply_wallpaper() -> Result<(), ThemedexError> {
        // Arrange
        let db = TestDb::new("test_apply_wallpaper");
        let mut color_scheme = create_test_color_scheme("test-color-scheme");
        color_scheme.insert_to_db(db.conn())?;
        let mut wallpaper = Wallpaper::new(
            String::from("test"),
            String::from("/home/harry/documents/photos/wallpapers"),
        );
        wallpaper.insert_to_db(db.conn())?;
        assert!(color_scheme.id.unwrap() > 0);
        assert!(wallpaper.id.unwrap() > 0);
        // let name = "purple_pyramid.jpg".to_string();
        let name = "current.png".to_string();
        let mut wallpaper_version = WallpaperVersion::new(&wallpaper, &color_scheme, name.clone())?;
        wallpaper_version.insert_to_db(db.conn())?;
        let mut config = make_config();
        config.set_wallpaper_command.arguments = vec![
            "img".into(),
            "-t".into(),
            "wave".into(),
            "--transition-duration".into(),
            "2".into(),
        ];
        // Act
        apply_wallpaper(&wallpaper, &wallpaper_version, &config)?;
        Ok(())
    }
}
