use rusqlite::{Connection, Result as SqliteResult};

pub fn resolve_or_create_artist(conn: &Connection, name: &str) -> SqliteResult<String> {
    let existing: Option<String> = conn
        .query_row(
            "SELECT id FROM artists WHERE name = ?1 COLLATE NOCASE",
            [name],
            |row| row.get(0),
        )
        .ok();

    if let Some(id) = existing {
        return Ok(id);
    }

    let id = uuid::Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO artists (id, name) VALUES (?1, ?2)",
        rusqlite::params![&id, name],
    )?;
    Ok(id)
}

pub fn resolve_or_create_album(conn: &Connection, name: &str) -> SqliteResult<String> {
    let existing: Option<String> = conn
        .query_row(
            "SELECT id FROM albums WHERE name = ?1 COLLATE NOCASE",
            [name],
            |row| row.get(0),
        )
        .ok();

    if let Some(id) = existing {
        return Ok(id);
    }

    let id = uuid::Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO albums (id, name) VALUES (?1, ?2)",
        rusqlite::params![&id, name],
    )?;
    Ok(id)
}

pub fn resolve_or_create_genre(conn: &Connection, name: &str) -> SqliteResult<String> {
    let existing: Option<String> = conn
        .query_row(
            "SELECT id FROM genres WHERE name = ?1 COLLATE NOCASE",
            [name],
            |row| row.get(0),
        )
        .ok();

    if let Some(id) = existing {
        return Ok(id);
    }

    let id = uuid::Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO genres (id, name) VALUES (?1, ?2)",
        rusqlite::params![&id, name],
    )?;
    Ok(id)
}

pub fn cleanup_orphaned_entities(conn: &Connection) -> SqliteResult<()> {
    conn.execute_batch(
        "DELETE FROM artists WHERE id NOT IN (SELECT DISTINCT artist_id FROM track_artists);
         DELETE FROM albums  WHERE id NOT IN (SELECT DISTINCT album_id FROM tracks WHERE album_id IS NOT NULL);
         DELETE FROM genres  WHERE id NOT IN (SELECT DISTINCT genre_id FROM tracks WHERE genre_id IS NOT NULL);",
    )?;
    Ok(())
}
