use rusqlite::Result as SqliteResult;

use super::{TRACK_SELECT, TrackRepository, row_to_track};
use crate::database::models::{ArtistWithCount, PaginatedTracks, Track};

impl TrackRepository {
    pub fn get_all_artists(&self, query: Option<&str>) -> SqliteResult<Vec<ArtistWithCount>> {
        let conn = self.conn.lock().unwrap();

        if let Some(q) = query {
            let pattern = format!("%{}%", q);
            let mut stmt = conn.prepare(
                "SELECT ar.id, ar.name, COUNT(DISTINCT t.id) AS track_count, COUNT(DISTINCT t.album_id) AS album_count, ar.image_hash, ar.mbid
                 FROM artists ar
                 JOIN track_artists ta ON ta.artist_id = ar.id
                 JOIN tracks t ON ta.track_id = t.id
                 LEFT JOIN albums al ON al.id = t.album_id
                 WHERE t.title LIKE ?1
                    OR al.name LIKE ?1
                    OR t.id IN (
                        SELECT ta2.track_id FROM track_artists ta2
                        JOIN artists ar2 ON ar2.id = ta2.artist_id
                        WHERE ar2.name LIKE ?1
                    )
                 GROUP BY ar.id
                 ORDER BY ar.name COLLATE NOCASE",
            )?;

            let rows = stmt.query_map([&pattern], |row| {
                Ok(ArtistWithCount {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    track_count: row.get(2)?,
                    album_count: row.get(3)?,
                    image_hash: row.get(4)?,
                    mbid: row.get(5)?,
                })
            })?;

            return rows.collect();
        }

        let mut stmt = conn.prepare(
            "SELECT ar.id, ar.name, COUNT(ta.track_id) AS track_count, COUNT(DISTINCT t.album_id) AS album_count, ar.image_hash, ar.mbid
             FROM artists ar
             LEFT JOIN track_artists ta ON ta.artist_id = ar.id
             LEFT JOIN tracks t ON ta.track_id = t.id
             GROUP BY ar.id
             ORDER BY ar.name COLLATE NOCASE",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(ArtistWithCount {
                id: row.get(0)?,
                name: row.get(1)?,
                track_count: row.get(2)?,
                album_count: row.get(3)?,
                image_hash: row.get(4)?,
                mbid: row.get(5)?,
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

    pub fn get_tracks_by_artist_paginated(
        &self,
        artist_id: &str,
        limit: i64,
        offset: i64,
        sort_column: Option<&str>,
        sort_dir: Option<&str>,
    ) -> SqliteResult<PaginatedTracks> {
        let conn = self.conn.lock().unwrap();

        let total: i64 = conn.query_row(
            "SELECT COUNT(DISTINCT ta.track_id)
             FROM track_artists ta
             WHERE ta.artist_id = ?1",
            [artist_id],
            |row| row.get(0),
        )?;

        let sort_clause = super::get_track_sort_clause(sort_column, sort_dir);
        let sql = format!(
            "{} WHERE t.id IN (
                SELECT ta2.track_id FROM track_artists ta2 WHERE ta2.artist_id = ?1
             )
             GROUP BY t.id
             {}
             LIMIT ?2 OFFSET ?3",
            TRACK_SELECT, sort_clause
        );

        let mut stmt = conn.prepare(&sql)?;
        let tracks: Vec<Track> = stmt
            .query_map(rusqlite::params![artist_id, limit, offset], row_to_track)?
            .collect::<SqliteResult<Vec<_>>>()?;

        Ok(PaginatedTracks { tracks, total })
    }

    pub fn get_artist_by_name(&self, name: &str) -> SqliteResult<Option<ArtistWithCount>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT ar.id, ar.name, COUNT(ta.track_id) AS track_count, COUNT(DISTINCT t.album_id) AS album_count, ar.image_hash, ar.mbid
             FROM artists ar
             LEFT JOIN track_artists ta ON ta.artist_id = ar.id
             LEFT JOIN tracks t ON ta.track_id = t.id
             WHERE ar.name = ?1
             GROUP BY ar.id",
        )?;

        let mut rows = stmt.query_map([name], |row| {
            Ok(ArtistWithCount {
                id: row.get(0)?,
                name: row.get(1)?,
                track_count: row.get(2)?,
                album_count: row.get(3)?,
                image_hash: row.get(4)?,
                mbid: row.get(5)?,
            })
        })?;

        match rows.next() {
            Some(row) => Ok(Some(row?)),
            None => Ok(None),
        }
    }
}
