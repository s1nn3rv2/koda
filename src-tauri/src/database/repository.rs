use rusqlite::{Connection, Result as SqliteResult, Row};
use std::sync::Mutex;

use super::models::{LibraryStats, Track};
use super::schema;

pub struct TrackRepository {
    conn: Mutex<Connection>,
}

impl TrackRepository {
    pub fn new(conn: Connection) -> SqliteResult<Self> {
        schema::initialize_schema(&conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn insert(&self, track: &Track) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO tracks (id, path, title, artists, album, duration, track_number, added_at, cover_hash)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![
                &track.id,
                &track.path,
                &track.title,
                &track.artists,
                &track.album,
                &track.duration,
                &track.track_number,
                &track.added_at,
                &track.cover_hash,
            ],
        )?;
        Ok(())
    }

    pub fn insert_batch(&self, tracks: &[Track]) -> SqliteResult<usize> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction()?;

        let mut inserted = 0;
        for track in tracks {
            tx.execute(
                "INSERT OR REPLACE INTO tracks (id, path, title, artists, album, duration, track_number, added_at, cover_hash)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                rusqlite::params![
                    &track.id,
                    &track.path,
                    &track.title,
                    &track.artists,
                    &track.album,
                    &track.duration,
                    &track.track_number,
                    &track.added_at,
                    &track.cover_hash,
                ],
            )?;
            inserted += 1;
        }

        tx.commit()?;
        Ok(inserted)
    }

    pub fn get_by_id(&self, id: &str) -> SqliteResult<Option<Track>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, path, title, artists, album, duration, track_number, added_at, cover_hash
             FROM tracks WHERE id = ?1",
        )?;

        let mut rows = stmt.query([id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(row_to_track(row)?))
        } else {
            Ok(None)
        }
    }

    pub fn get_by_path(&self, path: &str) -> SqliteResult<Option<Track>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, path, title, artists, album, duration, track_number, added_at, cover_hash
             FROM tracks WHERE path = ?1",
        )?;

        let mut rows = stmt.query([path])?;

        if let Some(row) = rows.next()? {
            Ok(Some(row_to_track(row)?))
        } else {
            Ok(None)
        }
    }

    pub fn get_all(&self) -> SqliteResult<Vec<Track>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, path, title, artists, album, duration, track_number, added_at, cover_hash
             FROM tracks ORDER BY artists, album, track_number",
        )?;

        let tracks = stmt.query_map([], row_to_track)?;
        tracks.collect()
    }

    pub fn search(&self, query: &str) -> SqliteResult<Vec<Track>> {
        let conn = self.conn.lock().unwrap();
        let search_pattern = format!("%{}%", query);

        let mut stmt = conn.prepare(
            "SELECT id, path, title, artists, album, duration, track_number, added_at, cover_hash
             FROM tracks
             WHERE title LIKE ?1 OR artists LIKE ?1 OR album LIKE ?1
             ORDER BY artists, album, track_number",
        )?;

        let tracks = stmt.query_map([&search_pattern], row_to_track)?;
        tracks.collect()
    }

    pub fn delete(&self, id: &str) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM tracks WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn clear(&self) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM tracks", [])?;
        Ok(())
    }

    pub fn count(&self) -> SqliteResult<i64> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM tracks", [], |row| row.get(0))?;
        Ok(count)
    }

    pub fn get_stats(&self) -> SqliteResult<LibraryStats> {
        let conn = self.conn.lock().unwrap();

        let total_tracks: i64 =
            conn.query_row("SELECT COUNT(*) FROM tracks", [], |row| row.get(0))?;

        let total_duration: i64 =
            conn.query_row("SELECT COALESCE(SUM(duration), 0) FROM tracks", [], |row| {
                row.get(0)
            })?;

        let unique_artists: i64 = conn.query_row(
            "SELECT COUNT(DISTINCT artists) FROM tracks WHERE artists IS NOT NULL",
            [],
            |row| row.get(0),
        )?;

        let unique_albums: i64 = conn.query_row(
            "SELECT COUNT(DISTINCT album) FROM tracks WHERE album IS NOT NULL",
            [],
            |row| row.get(0),
        )?;

        Ok(LibraryStats {
            total_tracks,
            total_duration,
            unique_artists,
            unique_albums,
        })
    }

    // drop indices before bulk insert for better performance
    pub fn prepare_for_bulk_insert(&self) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        schema::drop_indices(&conn)
    }

    pub fn finish_bulk_insert(&self) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        schema::create_indices(&conn)
    }
}

fn row_to_track(row: &Row) -> SqliteResult<Track> {
    Ok(Track {
        id: row.get(0)?,
        path: row.get(1)?,
        title: row.get(2)?,
        artists: row.get(3)?,
        album: row.get(4)?,
        duration: row.get(5)?,
        track_number: row.get(6)?,
        added_at: row.get(7)?,
        cover_hash: row.get(8)?,
    })
}
