use crate::common::{ThemedexError, internal_error};
use crate::database::models::{ColorScheme, Wallpaper, WallpaperVersion};
use rusqlite::{Connection, Error::QueryReturnedNoRows, params};

pub trait DatabaseTable: Sized {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self>;
    fn insert_to_db(&mut self, conn: &Connection) -> Result<(), ThemedexError>;
    fn select_by_id(id: u32, conn: &Connection) -> Result<Self, ThemedexError>;
    fn id_exists(id: u32, conn: &Connection) -> Result<bool, ThemedexError> {
        match Self::select_by_id(id, conn) {
            Err(ThemedexError::Database(QueryReturnedNoRows)) => Ok(true),
            Err(err) => Err(err),
            Ok(_) => Ok(false),
        }
    }
    fn sync_to_db(&self, conn: &Connection) -> Result<(), ThemedexError>;
}

pub trait TableWithName: Sized {
    fn select_by_name(name: &str, conn: &Connection) -> Result<Self, ThemedexError>;

    fn select_by_name_if_exists(
        name: &str,
        conn: &Connection,
    ) -> Result<Option<Self>, ThemedexError> {
        match Self::select_by_name(name, conn) {
            Err(ThemedexError::Database(QueryReturnedNoRows)) => Ok(None),
            Err(err) => Err(err),
            Ok(s) => Ok(Some(s)),
        }
    }
}

impl DatabaseTable for ColorScheme {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        let json_colors: String = row.get(2)?;
        let colors_map = serde_json::from_str(&json_colors).map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(2, rusqlite::types::Type::Text, Box::new(e))
        })?;
        Ok(Self {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            colors: colors_map,
        })
    }

    fn insert_to_db(&mut self, conn: &Connection) -> Result<(), ThemedexError> {
        if let Some(current_id) = self.id {
            return Err(ThemedexError::Internal(format!(
                "insert_to_db called for wallpaper but object already exists with id {current_id}"
            )));
        }
        let json_colors = serde_json::to_string(&self.colors)?;
        self.id = conn.query_row(
            "INSERT INTO color_scheme (name, colors_json) VALUES (?1, ?2) RETURNING id",
            params![self.name, json_colors],
            |row| row.get(0),
        )?;
        Ok(())
    }

    fn select_by_id(id: u32, conn: &Connection) -> Result<Self, ThemedexError> {
        let res = conn.query_row(
            "SELECT id, name, colors_json FROM color_scheme WHERE id = ?1",
            params![id],
            Self::from_row,
        )?;
        Ok(res)
    }

    fn sync_to_db(&self, conn: &Connection) -> Result<(), ThemedexError> {
        let id = self
            .id
            .ok_or_else(|| internal_error("sync_to_db called for struct with no ID"))?;
        let json_colors = serde_json::to_string(&self.colors)?;
        conn.execute(
            "UPDATE color_scheme SET name = ?1, colors_json = ?2 WHERE id = ?3",
            params![self.name, json_colors, id],
        )?;
        Ok(())
    }
}

impl TableWithName for ColorScheme {
    fn select_by_name(name: &str, conn: &Connection) -> Result<Self, ThemedexError> {
        let res = conn.query_row(
            "SELECT id, name, colors_json FROM color_scheme WHERE name = ?1",
            params![name],
            Self::from_row,
        )?;
        Ok(res)
    }
}

impl DatabaseTable for Wallpaper {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            path: row.get(2)?,
            default_version_id: row.get(3)?,
        })
    }

    fn insert_to_db(&mut self, conn: &Connection) -> Result<(), ThemedexError> {
        if let Some(current_id) = self.id {
            return Err(ThemedexError::Internal(format!(
                "insert_to_db called for wallpaper but object already exists with id {current_id}"
            )));
        }
        self.id = conn.query_row(
            "INSERT INTO wallpaper (name, path, default_version_id) VALUES (?1, ?2, ?3) RETURNING id",
            params![self.name, self.path, self.default_version_id],
            |row| row.get(0),
        )?;
        Ok(())
    }

    fn select_by_id(id: u32, conn: &Connection) -> Result<Self, ThemedexError> {
        let res = conn.query_row(
            "SELECT id, name, path, default_version_id FROM wallpaper WHERE id = ?1",
            params![id],
            Self::from_row,
        )?;
        Ok(res)
    }

    fn sync_to_db(&self, conn: &Connection) -> Result<(), ThemedexError> {
        let id = self
            .id
            .ok_or_else(|| internal_error("sync_to_db called for struct with no ID"))?;
        conn.execute(
            "UPDATE wallpaper SET name = ?1, path = ?2, default_version_id = ?3 WHERE id = ?4",
            params![self.name, self.path, self.default_version_id, id],
        )?;
        Ok(())
    }
}

impl TableWithName for Wallpaper {
    fn select_by_name(name: &str, conn: &Connection) -> Result<Self, ThemedexError> {
        let res = conn.query_row(
            "SELECT id, name, path, default_version_id FROM wallpaper WHERE name = ?1",
            params![name],
            Self::from_row,
        )?;
        Ok(res)
    }
}

impl DatabaseTable for WallpaperVersion {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: Some(row.get(0)?),
            wallpaper_id: row.get(1)?,
            color_scheme_id: row.get(2)?,
            filename: row.get(3)?,
        })
    }

    fn insert_to_db(&mut self, conn: &Connection) -> Result<(), ThemedexError> {
        if let Some(current_id) = self.id {
            return Err(ThemedexError::Internal(format!(
                "insert_to_db called for wallpaper_version but object already exists with id {current_id}"
            )));
        }
        self.id = conn.query_row(
            "INSERT INTO wallpaper_version (wallpaper_id, color_scheme_id, filename) VALUES (?1, ?2, ?3) RETURNING id",
            params![self.wallpaper_id, self.color_scheme_id, self.filename],
            |row| row.get(0),
        )?;
        Ok(())
    }

    fn select_by_id(id: u32, conn: &Connection) -> Result<Self, ThemedexError> {
        let res = conn.query_row(
            "SELECT id, wallpaper_id, color_scheme_id, filename FROM wallpaper_version WHERE id = ?1",
            params![id],
            Self::from_row,
        )?;
        Ok(res)
    }

    fn sync_to_db(&self, conn: &Connection) -> Result<(), ThemedexError> {
        let id = self
            .id
            .ok_or_else(|| internal_error("sync_to_db called for struct with no ID"))?;
        conn.execute(
            "UPDATE wallpaper_version SET wallpaper_id = ?1, color_scheme_id = ?2, filename = ?3 WHERE id = ?4",
            params![self.wallpaper_id, self.color_scheme_id, self.filename, id],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests;
