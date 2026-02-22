mod albums;
mod artists;
mod genres;
mod helpers;
mod stats;
mod tracks;

use rusqlite::{Connection, Result as SqliteResult, Row};
use std::sync::Mutex;

use super::models::Track;
use super::schema;

pub struct TrackRepository {
    pub(crate) conn: Mutex<Connection>,
}

pub(crate) const TRACK_SELECT: &str = "
    SELECT t.id, t.path, t.title, t.album_id, t.genre_id,
           t.duration, t.track_number, t.disc_number, t.added_at, 
           COALESCE(t.cover_hash, al.cover_hash, (
               SELECT t2.cover_hash FROM tracks t2 
               WHERE t2.album_id = t.album_id AND t2.cover_hash IS NOT NULL 
               ORDER BY t2.release_date DESC, t2.added_at DESC LIMIT 1
           )) AS cover_hash,
           al.name AS album_name,
           g.name AS genre_name,
           GROUP_CONCAT(ar.name, '; ') AS artists,
           (SELECT GROUP_CONCAT(aa.artist_id, ';') FROM album_artists aa WHERE aa.album_id = t.album_id) AS album_artist_ids,
           (SELECT GROUP_CONCAT(aa_ar.name, '; ') FROM album_artists aa JOIN artists aa_ar ON aa_ar.id = aa.artist_id WHERE aa.album_id = t.album_id) AS album_artist,
           t.release_date
    FROM tracks t
    LEFT JOIN albums al ON al.id = t.album_id
    LEFT JOIN genres g ON g.id = t.genre_id
    LEFT JOIN track_artists ta ON ta.track_id = t.id
    LEFT JOIN artists ar ON ar.id = ta.artist_id
";

pub(crate) fn row_to_track(row: &Row) -> SqliteResult<Track> {
    let album_artist_ids_raw: Option<String> = row.get(13)?;
    let album_artist_ids = album_artist_ids_raw
        .map(|s| s.split(';').map(|id| id.to_string()).collect())
        .unwrap_or_default();

    Ok(Track {
        id: row.get(0)?,
        path: row.get(1)?,
        title: row.get(2)?,
        album_id: row.get(3)?,
        genre_id: row.get(4)?,
        duration: row.get(5)?,
        track_number: row.get(6)?,
        disc_number: row.get(7)?,
        added_at: row.get(8)?,
        cover_hash: row.get(9)?,
        album: row.get(10)?,
        genre: row.get(11)?,
        artists: row.get(12)?,
        album_artist_ids,
        album_artist: row.get(14)?,
        release_date: row.get(15)?,
    })
}

pub(crate) fn get_track_sort_clause(sort_column: Option<&str>, sort_dir: Option<&str>) -> String {
    let col = match sort_column {
        Some("added_at") => "t.added_at",
        Some("release_date") => "t.release_date",
        Some("title") => "t.title COLLATE NOCASE",
        Some("duration") => "t.duration",
        _ => "artists, al.name, t.track_number",
    };

    let dir = match sort_dir {
        Some(d) if d.eq_ignore_ascii_case("desc") => "DESC",
        _ => "ASC",
    };

    if col == "artists, al.name, t.track_number" {
        format!("ORDER BY {}", col)
    } else {
        format!("ORDER BY {} {}", col, dir)
    }
}

pub(crate) fn get_album_sort_clause(sort_column: Option<&str>, sort_dir: Option<&str>) -> String {
    let col = match sort_column {
        Some("added_at") => "MAX(t.added_at)",
        Some("release_date") => "release_date",
        Some("name") => "al.name COLLATE NOCASE",
        Some("track_count") => "track_count",
        _ => "al.name COLLATE NOCASE",
    };

    let dir = match sort_dir {
        Some(d) if d.eq_ignore_ascii_case("desc") => "DESC",
        _ => "ASC",
    };

    if col == "al.name COLLATE NOCASE" && dir == "ASC" {
        format!("ORDER BY {}", col)
    } else {
        format!("ORDER BY {} {}", col, dir)
    }
}

impl TrackRepository {
    pub fn new(conn: Connection) -> SqliteResult<Self> {
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA foreign_keys = ON;",
        )?;
        schema::initialize_schema(&conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn prepare_for_bulk_insert(&self) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        schema::drop_indices(&conn)
    }

    pub fn finish_bulk_insert(&self) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        helpers::cleanup_orphaned_entities(&conn)?;
        schema::create_indices(&conn)
    }
}
