use rusqlite::Result as SqliteResult;

use super::{TRACK_SELECT, TrackRepository, row_to_track};
use crate::database::models::{GenreWithCount, PaginatedTracks, Track};

impl TrackRepository {
    pub fn get_all_genres(&self, query: Option<&str>) -> SqliteResult<Vec<GenreWithCount>> {
        let conn = self.conn.lock().unwrap();

        if let Some(q) = query {
            let pattern = format!("%{}%", q);
            let mut stmt = conn.prepare(
                "SELECT g.id, g.name, COUNT(DISTINCT t.id) AS track_count, COUNT(DISTINCT t.album_id) AS album_count
                 FROM genres g
                 JOIN tracks t ON t.genre_id = g.id
                 LEFT JOIN albums al ON al.id = t.album_id
                 WHERE t.title LIKE ?1
                    OR al.name LIKE ?1
                    OR t.id IN (
                        SELECT ta2.track_id FROM track_artists ta2
                        JOIN artists ar2 ON ar2.id = ta2.artist_id
                        WHERE ar2.name LIKE ?1
                    )
                 GROUP BY g.id
                 ORDER BY g.name COLLATE NOCASE",
            )?;

            let rows = stmt.query_map([&pattern], |row| {
                Ok(GenreWithCount {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    track_count: row.get(2)?,
                    album_count: row.get(3)?,
                })
            })?;

            return rows.collect();
        }

        let mut stmt = conn.prepare(
            "SELECT g.id, g.name, COUNT(t.id) AS track_count, COUNT(DISTINCT t.album_id) AS album_count
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
                album_count: row.get(3)?,
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

    pub fn get_tracks_by_genre_paginated(
        &self,
        genre_id: &str,
        limit: i64,
        offset: i64,
        sort_column: Option<&str>,
        sort_dir: Option<&str>,
    ) -> SqliteResult<PaginatedTracks> {
        let conn = self.conn.lock().unwrap();

        let total: i64 = conn.query_row(
            "SELECT COUNT(*) FROM tracks WHERE genre_id = ?1",
            [genre_id],
            |row| row.get(0),
        )?;

        let sort_clause = super::get_track_sort_clause(sort_column, sort_dir);
        let sql = format!(
            "{} WHERE t.genre_id = ?1
             GROUP BY t.id
             {}
             LIMIT ?2 OFFSET ?3",
            TRACK_SELECT, sort_clause
        );

        let mut stmt = conn.prepare(&sql)?;
        let tracks: Vec<Track> = stmt
            .query_map(rusqlite::params![genre_id, limit, offset], row_to_track)?
            .collect::<SqliteResult<Vec<_>>>()?;

        Ok(PaginatedTracks { tracks, total })
    }
}
