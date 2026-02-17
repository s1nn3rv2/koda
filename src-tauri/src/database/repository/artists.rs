use rusqlite::Result as SqliteResult;

use super::{TRACK_SELECT, TrackRepository, row_to_track};
use crate::database::models::{ArtistWithCount, Track};

impl TrackRepository {
    pub fn get_all_artists(&self) -> SqliteResult<Vec<ArtistWithCount>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT ar.id, ar.name, COUNT(ta.track_id) AS track_count
             FROM artists ar
             LEFT JOIN track_artists ta ON ta.artist_id = ar.id
             GROUP BY ar.id
             ORDER BY ar.name COLLATE NOCASE",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(ArtistWithCount {
                id: row.get(0)?,
                name: row.get(1)?,
                track_count: row.get(2)?,
            })
        })?;

        rows.collect()
    }

    pub fn get_tracks_by_artist(&self, artist_id: &str) -> SqliteResult<Vec<Track>> {
        let conn = self.conn.lock().unwrap();
        let sql = format!(
            "{} WHERE t.id IN (
                SELECT ta2.track_id FROM track_artists ta2 WHERE ta2.artist_id = ?1
             )
             GROUP BY t.id
             ORDER BY al.name, t.track_number",
            TRACK_SELECT
        );

        let mut stmt = conn.prepare(&sql)?;
        let tracks = stmt.query_map([artist_id], row_to_track)?;
        tracks.collect()
    }
}
