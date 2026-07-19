use rusqlite::Connection;

use crate::{common::ThemedexError, database::models::WallpaperVersionNames};

pub fn list_color_scheme_names(
    like: Option<&str>,
    conn: &Connection,
) -> Result<Vec<String>, ThemedexError> {
    let (sql, params) = match like {
        Some(name) => (
            "SELECT name FROM color_scheme WHERE name LIKE ?",
            vec![format!("%{}%", name)],
        ),
        None => ("SELECT name FROM color_scheme", vec![]),
    };
    conn.prepare(sql)?
        .query_map(rusqlite::params_from_iter(params), |row| row.get(0))?
        .collect::<Result<Vec<String>, _>>()
        .map_err(|e| e.into())
}

pub fn list_wallpaper_names(
    like: Option<&str>,
    conn: &Connection,
) -> Result<Vec<String>, ThemedexError> {
    let (sql, params) = match like {
        Some(name) => (
            "SELECT name FROM wallpaper WHERE name LIKE ?",
            vec![format!("%{}%", name)],
        ),
        None => ("SELECT name FROM wallpaper", vec![]),
    };
    conn.prepare(sql)?
        .query_map(rusqlite::params_from_iter(params), |row| row.get(0))?
        .collect::<Result<Vec<String>, _>>()
        .map_err(|e| e.into())
}

pub fn list_wallpaper_version_names(
    wallpaper: Option<&str>,
    color_scheme: Option<&str>,
    conn: &Connection,
) -> Result<Vec<WallpaperVersionNames>, ThemedexError> {
    let mut sql = "
        SELECT wallpaper.name, color_scheme.name
        FROM wallpaper_version
        JOIN wallpaper ON wallpaper.id = wallpaper_version.wallpaper_id
        JOIN color_scheme ON color_scheme.id = wallpaper_version.color_scheme_id
        WHERE 1=1"
        .to_string();
    let mut params = vec![];
    if let Some(wallpaper_name) = wallpaper {
        sql.push_str(" AND wallpaper.name LIKE ?");
        params.push(format!("%{}%", wallpaper_name));
    }
    if let Some(color_scheme_name) = color_scheme {
        sql.push_str(" AND color_scheme.name LIKE ?");
        params.push(format!("%{}%", color_scheme_name));
    }
    conn.prepare(&sql)?
        .query_map(rusqlite::params_from_iter(params), |row| {
            Ok(WallpaperVersionNames {
                wallpaper_name: row.get(0)?,
                color_scheme_name: row.get(1)?,
            })
        })?
        .collect::<Result<Vec<WallpaperVersionNames>, _>>()
        .map_err(|e| e.into())
}
