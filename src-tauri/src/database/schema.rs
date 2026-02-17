use rusqlite::{Connection, Result as SqliteResult};

const SCHEMA_VERSION: i32 = 1;

pub fn initialize_schema(conn: &Connection) -> SqliteResult<()> {
    // create metadata table for versioning
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_metadata (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    )?;

    let current_version = get_schema_version(conn)?;

    if current_version == 0 {
        create_initial_schema(conn)?;
        set_schema_version(conn, SCHEMA_VERSION)?;
    } else if current_version < SCHEMA_VERSION {
        migrate(conn, current_version, SCHEMA_VERSION)?;
    }

    Ok(())
}

fn create_initial_schema(conn: &Connection) -> SqliteResult<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tracks (
            id TEXT PRIMARY KEY,
            path TEXT NOT NULL UNIQUE,
            title TEXT,
            artists TEXT,
            album TEXT,
            duration INTEGER,
            track_number INTEGER,
            added_at INTEGER NOT NULL,
            cover_hash TEXT
        )",
        [],
    )?;

    create_indices(conn)?;
    Ok(())
}

pub fn create_indices(conn: &Connection) -> SqliteResult<()> {
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_artists ON tracks(artists)",
        [],
    )?;

    conn.execute("CREATE INDEX IF NOT EXISTS idx_album ON tracks(album)", [])?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_added_at ON tracks(added_at)",
        [],
    )?;

    Ok(())
}

pub fn drop_indices(conn: &Connection) -> SqliteResult<()> {
    conn.execute("DROP INDEX IF EXISTS idx_artists", [])?;
    conn.execute("DROP INDEX IF EXISTS idx_album", [])?;
    conn.execute("DROP INDEX IF EXISTS idx_added_at", [])?;
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

fn migrate(conn: &Connection, from: i32, to: i32) -> SqliteResult<()> {
    for version in (from + 1)..=to {
        // future migrations would go here
        set_schema_version(conn, version)?;
    }
    Ok(())
}
