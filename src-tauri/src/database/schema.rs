use rusqlite::{Connection, Result as SqliteResult};

const SCHEMA_VERSION: i32 = 1;

pub fn initialize_schema(conn: &Connection) -> SqliteResult<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_metadata (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    )?;

    let current_version = get_schema_version(conn)?;

    if current_version < SCHEMA_VERSION {
        if current_version == 0 {
            create_schema(conn)?;
        } else {
            migrate(conn, current_version)?;
        }
        set_schema_version(conn, SCHEMA_VERSION)?;
    }

    Ok(())
}

fn migrate(_conn: &Connection, _from_version: i32) -> SqliteResult<()> {
    Ok(())
}

fn create_schema(conn: &Connection) -> SqliteResult<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS artists (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL UNIQUE COLLATE NOCASE,
            image_hash TEXT,
            mbid TEXT
        );

        CREATE TABLE IF NOT EXISTS albums (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL COLLATE NOCASE,
            artist_id TEXT REFERENCES artists(id) ON DELETE SET NULL,
            cover_hash TEXT,
            release_date TEXT,
            mbid TEXT,
            UNIQUE(name)
        );

        CREATE TABLE IF NOT EXISTS genres (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL UNIQUE COLLATE NOCASE
        );

        CREATE TABLE IF NOT EXISTS tracks (
            id TEXT PRIMARY KEY,
            path TEXT NOT NULL UNIQUE,
            title TEXT,
            album_id TEXT REFERENCES albums(id) ON DELETE SET NULL,
            genre_id TEXT REFERENCES genres(id) ON DELETE SET NULL,
            duration INTEGER,
            track_number INTEGER,
            disc_number INTEGER,
            release_date TEXT,
            added_at INTEGER NOT NULL,
            cover_hash TEXT
        );

        CREATE TABLE IF NOT EXISTS track_artists (
            track_id TEXT NOT NULL REFERENCES tracks(id) ON DELETE CASCADE,
            artist_id TEXT NOT NULL REFERENCES artists(id) ON DELETE CASCADE,
            PRIMARY KEY (track_id, artist_id)
        );
        
        CREATE TABLE IF NOT EXISTS album_artists (
            album_id TEXT NOT NULL REFERENCES albums(id) ON DELETE CASCADE,
            artist_id TEXT NOT NULL REFERENCES artists(id) ON DELETE CASCADE,
            PRIMARY KEY (album_id, artist_id)
        );",
    )?;

    create_indices(conn)?;
    Ok(())
}

pub fn create_indices(conn: &Connection) -> SqliteResult<()> {
    conn.execute_batch(
        "CREATE INDEX IF NOT EXISTS idx_tracks_album_id ON tracks(album_id);
         CREATE INDEX IF NOT EXISTS idx_tracks_genre_id ON tracks(genre_id);
         CREATE INDEX IF NOT EXISTS idx_tracks_added_at ON tracks(added_at);
         CREATE INDEX IF NOT EXISTS idx_track_artists_track ON track_artists(track_id);
         CREATE INDEX IF NOT EXISTS idx_track_artists_artist ON track_artists(artist_id);
         CREATE INDEX IF NOT EXISTS idx_album_artists_album ON album_artists(album_id);
         CREATE INDEX IF NOT EXISTS idx_album_artists_artist ON album_artists(artist_id);
         CREATE INDEX IF NOT EXISTS idx_artists_name ON artists(name);
         CREATE INDEX IF NOT EXISTS idx_albums_name ON albums(name);
         CREATE INDEX IF NOT EXISTS idx_genres_name ON genres(name);",
    )?;

    Ok(())
}

pub fn drop_indices(conn: &Connection) -> SqliteResult<()> {
    conn.execute_batch(
        "DROP INDEX IF EXISTS idx_tracks_album_id;
         DROP INDEX IF EXISTS idx_tracks_genre_id;
         DROP INDEX IF EXISTS idx_tracks_added_at;
         DROP INDEX IF EXISTS idx_track_artists_track;
         DROP INDEX IF EXISTS idx_track_artists_artist;
         DROP INDEX IF EXISTS idx_album_artists_album;
         DROP INDEX IF EXISTS idx_album_artists_artist;
         DROP INDEX IF EXISTS idx_artists_name;
         DROP INDEX IF EXISTS idx_albums_name;
         DROP INDEX IF EXISTS idx_genres_name;",
    )?;

    Ok(())
}

fn get_schema_version(conn: &Connection) -> SqliteResult<i32> {
    let version: Result<String, _> = conn.query_row(
        "SELECT value FROM schema_metadata WHERE key = 'version'",
        [],
        |row| row.get(0),
    );

    match version {
        Ok(v) => Ok(v.parse().unwrap_or(0)),
        Err(_) => Ok(0),
    }
}

fn set_schema_version(conn: &Connection, version: i32) -> SqliteResult<()> {
    conn.execute(
        "INSERT OR REPLACE INTO schema_metadata (key, value) VALUES ('version', ?1)",
        [version.to_string()],
    )?;
    Ok(())
}

pub fn split_artist_names(raw: &str) -> Vec<String> {
    let mut results = Vec::new();

    for sub in raw.split(';') {
        let lowered = sub.to_lowercase();
        if let Some(pos) = lowered.find("feat.") {
            let before = &sub[..pos];
            let after = &sub[pos + 5..];
            let b = before.trim().to_string();
            let a = after.trim().to_string();
            if !b.is_empty() {
                results.push(b);
            }
            if !a.is_empty() {
                results.push(a);
            }
        } else if let Some(pos) = lowered.find(" ft.") {
            let before = &sub[..pos];
            let after = &sub[pos + 4..];
            let b = before.trim().to_string();
            let a = after.trim().to_string();
            if !b.is_empty() {
                results.push(b);
            }
            if !a.is_empty() {
                results.push(a);
            }
        } else {
            let trimmed = sub.trim().to_string();
            if !trimmed.is_empty() {
                results.push(trimmed);
            }
        }
    }

    results
}
