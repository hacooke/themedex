use std::{
    fs,
    path::{Path, PathBuf, absolute},
};

use rusqlite::{Error::SqliteFailure, ffi::SQLITE_CONSTRAINT_UNIQUE};

use crate::{
    common::{
        ThemedexError::{self, Database},
        ToInternal, internal_error,
    },
    database::{
        interfaces::{DatabaseTable, TableWithName},
        models::{ColorScheme, Wallpaper, WallpaperVersion},
        utils::HasConnection,
    },
};

pub struct ImportAgent<T: HasConnection> {
    pub wallpaper_directory: String,
    pub conn: T,
}

impl<T: HasConnection> ImportAgent<T> {
    pub fn import_color_scheme_json(
        &self,
        path: &str,
        name: Option<&str>,
        update: bool,
    ) -> Result<ColorScheme, ThemedexError> {
        // Get name
        let scheme_name = match name {
            Some(n) => n,
            None => get_name_from_path(path)?,
        };
        // Guard against pre-existing color scheme if update is false
        let exists_check =
            ColorScheme::select_by_name_if_exists(scheme_name, self.conn.conn())?;
        if !update && let Some(_) = exists_check {
            return Err(internal_error(
                "Imported color scheme already exists and update not specified.",
            ));
        }
        // Read json file to struct
        let file_contents = fs::read_to_string(path)?;
        let color_data = serde_json::from_str(&file_contents)?;
        match exists_check {
            Some(mut existing_scheme) => {
                existing_scheme.colors = color_data;
                Ok(existing_scheme)
            }
            None => {
                let mut new_scheme = ColorScheme::new(scheme_name.to_string(), color_data);
                new_scheme.insert_to_db(self.conn.conn())?;
                Ok(new_scheme)
            }
        }
    }

    pub fn import_wallpaper_and_version(
        &self,
        path: &str,
        color_scheme: &str,
        name: Option<&str>,
        update: bool,
    ) -> Result<(Wallpaper, WallpaperVersion), ThemedexError> {
        let mut wallpaper = self.import_wallpaper(path, name, update)?;
        let version = self.import_wallpaper_version(path, &wallpaper, color_scheme, update)?;
        wallpaper.default_version_id = version.id;
        wallpaper.sync_to_db(self.conn.conn())?;
        Ok((wallpaper, version))
    }

    pub fn import_wallpaper(
        &self,
        path: &str,
        name: Option<&str>,
        update: bool,
    ) -> Result<Wallpaper, ThemedexError> {
        // Get name
        let wall_name = match name {
            Some(n) => n,
            None => get_name_from_path(path)?,
        };
        // Guard against pre-existing color scheme if update is false
        let exists_check = Wallpaper::select_by_name_if_exists(wall_name, self.conn.conn())?;
        if !update && let Some(_) = exists_check {
            return Err(internal_error(
                "Imported wallpaper already exists and update not specified.",
            ));
        }
        // Create wallpaper directory in themedex directories
        let buf = Path::new(&self.wallpaper_directory).join(wall_name);
        let wall_dir_path = buf
            .to_str()
            .internal_err("Could not create wallpaper directory.")?;
        std::fs::create_dir_all(wall_dir_path)?;
        // Add wallpaper to DB with path to created directory
        match exists_check {
            Some(mut existing_wallpaper) => {
                existing_wallpaper.path = wall_dir_path.to_string();
                Ok(existing_wallpaper)
            }
            None => {
                let mut new_wallpaper =
                    Wallpaper::new(wall_name.to_string(), wall_dir_path.to_string());
                new_wallpaper.insert_to_db(self.conn.conn())?;
                Ok(new_wallpaper)
            }
        }
    }

    pub fn import_wallpaper_version_by_wall_name(
        &self,
        path: &str,
        wallpaper_name: &str,
        color_scheme_name: &str,
        update: bool,
    ) -> Result<WallpaperVersion, ThemedexError> {
        self.import_wallpaper_version(
            path,
            &Wallpaper::select_by_name(wallpaper_name, self.conn.conn())?,
            color_scheme_name,
            update,
        )
    }

    pub fn import_wallpaper_version(
        &self,
        path: &str,
        wallpaper: &Wallpaper,
        color_scheme_name: &str,
        update: bool,
    ) -> Result<WallpaperVersion, ThemedexError> {
        // Update behaviour defines what we do if a file already exists where we want to put the
        // image. If the database entry already exists we continue regardless as if this is true
        // but there is no image there then the entry would be broken.
        // Get color scheme
        let Ok(color_scheme) =
            ColorScheme::select_by_name(color_scheme_name, self.conn.conn())
        else {
            return Err(ThemedexError::Internal(
                format!("Color scheme {color_scheme_name} does not exist."),
            ));
        };
        // Get new file location
        let destination_path = get_variant_path(&wallpaper.path, &color_scheme.name, path)?;
        copy_image(path.into(), destination_path, update)?;
        // Create variant in DB (ignore if fails due to unique constraint violation)
        let mut wallpaper_version = WallpaperVersion::new(wallpaper, &color_scheme)?;
        match wallpaper_version.insert_to_db(self.conn.conn()) {
            Ok(_) => Ok(()),
            Err(Database(SqliteFailure(e, _))) if e.extended_code == SQLITE_CONSTRAINT_UNIQUE => {
                Ok(())
            }
            Err(e) => Err(e),
        }?;
        Ok(wallpaper_version)
    }
}

fn copy_image(
    original_path: PathBuf,
    destination_path: PathBuf,
    replace: bool,
) -> Result<(), ThemedexError> {
    let original_path_absolute = absolute(original_path)?;
    let destination_path_absolute = absolute(destination_path)?;
    // Check if source and destination path are the same (likely image was created by themedex)
    if destination_path_absolute.with_extension("") == original_path_absolute.with_extension("") {
        return Ok(());
    }
    // Check if destination path already exists otherwise
    let directory = destination_path_absolute
        .parent()
        .internal_err("Error finding wallpaper directory when copying image.")?;
    let destination_stem = destination_path_absolute.file_stem();
    let mut existing: Option<PathBuf> = None;
    for entry in fs::read_dir(directory)? {
        let candidate = entry?.path();
        if candidate.file_stem() == destination_stem {
            existing = Some(candidate);
            break;
        }
    }
    match existing {
        Some(_) if !replace => {
            return Err(internal_error(
                "Wallpaper variant already exists and update not specified.",
            ));
        }
        Some(f) => std::fs::remove_file(f)?,
        None => (),
    }
    // Copy file
    std::fs::copy(&original_path_absolute, &destination_path_absolute)?;
    Ok(())
}

fn get_name_from_path(path: &str) -> Result<&str, ThemedexError> {
    Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .internal_err("Failed to extract name from file path.")
}

fn get_variant_path(
    wallpaper_path: &str,
    color_scheme_name: &str,
    original_path: &str,
) -> Result<PathBuf, ThemedexError> {
    let extension = Path::new(original_path)
        .extension()
        .internal_err("Failed to get extension from wallpaper path, expected an image file?")?;
    Ok(Path::new(&wallpaper_path)
        .join(color_scheme_name)
        .with_extension(extension))
}

#[cfg(test)]
mod tests;
