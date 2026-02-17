use rusqlite::Result as SqliteResult;

use super::TrackRepository;
use crate::database::models::LibraryStats;

impl TrackRepository {
    pub fn get_stats(&self) -> SqliteResult<LibraryStats> {
        let conn = self.conn.lock().unwrap();

        let total_tracks: i64 =
            conn.query_row("SELECT COUNT(*) FROM tracks", [], |row| row.get(0))?;

        let total_duration: i64 =
            conn.query_row("SELECT COALESCE(SUM(duration), 0) FROM tracks", [], |row| {
                row.get(0)
            })?;

        let unique_artists: i64 =
            conn.query_row("SELECT COUNT(*) FROM artists", [], |row| row.get(0))?;

        let unique_albums: i64 =
            conn.query_row("SELECT COUNT(*) FROM albums", [], |row| row.get(0))?;

        let unique_genres: i64 =
            conn.query_row("SELECT COUNT(*) FROM genres", [], |row| row.get(0))?;

        Ok(LibraryStats {
            total_tracks,
            total_duration,
            unique_artists,
            unique_albums,
            unique_genres,
        })
    }
}
