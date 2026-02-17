use rusqlite::Result as SqliteResult;

use super::{TRACK_SELECT, TrackRepository, row_to_track};
use crate::database::models::{AlbumWithCount, Track};

impl TrackRepository {
    pub fn get_all_albums(&self) -> SqliteResult<Vec<AlbumWithCount>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT al.id, al.name, al.cover_hash, COUNT(t.id) AS track_count,
                    (SELECT GROUP_CONCAT(DISTINCT ar.name, '; ')
                     FROM track_artists ta
                     JOIN artists ar ON ar.id = ta.artist_id
                     WHERE ta.track_id IN (SELECT t2.id FROM tracks t2 WHERE t2.album_id = al.id)
                    ) AS artist_name
             FROM albums al
             LEFT JOIN tracks t ON t.album_id = al.id
             GROUP BY al.id
             ORDER BY al.name COLLATE NOCASE",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(AlbumWithCount {
                id: row.get(0)?,
                name: row.get(1)?,
                cover_hash: row.get(2)?,
                track_count: row.get(3)?,
                artist_name: row.get(4)?,
            })
        })?;

        rows.collect()
    }

    pub fn get_tracks_by_album(&self, album_id: &str) -> SqliteResult<Vec<Track>> {
        let conn = self.conn.lock().unwrap();
        let sql = format!(
            "{} WHERE t.album_id = ?1
             GROUP BY t.id
             ORDER BY t.track_number, t.title",
            TRACK_SELECT
        );

        let mut stmt = conn.prepare(&sql)?;
        let tracks = stmt.query_map([album_id], row_to_track)?;
        tracks.collect()
    }
}
