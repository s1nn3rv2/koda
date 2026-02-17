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
           t.duration, t.track_number, t.added_at, t.cover_hash,
           al.name AS album_name,
           g.name AS genre_name,
           GROUP_CONCAT(ar.name, '; ') AS artists
    FROM tracks t
    LEFT JOIN albums al ON al.id = t.album_id
    LEFT JOIN genres g ON g.id = t.genre_id
    LEFT JOIN track_artists ta ON ta.track_id = t.id
    LEFT JOIN artists ar ON ar.id = ta.artist_id
";

pub(crate) fn row_to_track(row: &Row) -> SqliteResult<Track> {
    Ok(Track {
        id: row.get(0)?,
        path: row.get(1)?,
        title: row.get(2)?,
        album_id: row.get(3)?,
        genre_id: row.get(4)?,
        duration: row.get(5)?,
        track_number: row.get(6)?,
        added_at: row.get(7)?,
        cover_hash: row.get(8)?,
        album: row.get(9)?,
        genre: row.get(10)?,
        artists: row.get(11)?,
    })
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
