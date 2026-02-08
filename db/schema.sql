BEGIN;

CREATE TABLE IF NOT EXISTS color_scheme (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    colors_json TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS wallpaper (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    path TEXT NOT NULL UNIQUE,
    default_version_id INTEGER,

    FOREIGN KEY (default_version_id) REFERENCES wallpaper_version(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS wallpaper_version (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    wallpaper_id INTEGER NOT NULL,
    color_scheme_id INTEGER NOT NULL,

    FOREIGN KEY (wallpaper_id) REFERENCES wallpaper(id) ON DELETE CASCADE,
    FOREIGN KEY (color_scheme_id) REFERENCES color_scheme(id) ON DELETE CASCADE,
    UNIQUE (wallpaper_id, color_scheme_id)
);

COMMIT;
