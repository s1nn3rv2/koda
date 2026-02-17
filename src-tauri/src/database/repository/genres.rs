use rusqlite::Result as SqliteResult;

use super::{TRACK_SELECT, TrackRepository, row_to_track};
use crate::database::models::{GenreWithCount, Track};

impl TrackRepository {
    pub fn get_all_genres(&self) -> SqliteResult<Vec<GenreWithCount>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT g.id, g.name, COUNT(t.id) AS track_count
             FROM genres g
             LEFT JOIN tracks t ON t.genre_id = g.id
             GROUP BY g.id
             ORDER BY g.name COLLATE NOCASE",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(GenreWithCount {
                id: row.get(0)?,
                name: row.get(1)?,
                track_count: row.get(2)?,
            })
        })?;

        rows.collect()
    }

    pub fn get_tracks_by_genre(&self, genre_id: &str) -> SqliteResult<Vec<Track>> {
        let conn = self.conn.lock().unwrap();
        let sql = format!(
            "{} WHERE t.genre_id = ?1
             GROUP BY t.id
             ORDER BY artists, al.name, t.track_number",
            TRACK_SELECT
        );

        let mut stmt = conn.prepare(&sql)?;
        let tracks = stmt.query_map([genre_id], row_to_track)?;
        tracks.collect()
    }
}
