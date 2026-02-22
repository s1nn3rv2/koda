use rusqlite::Result as SqliteResult;

use super::{row_to_track, TrackRepository, TRACK_SELECT};
use crate::database::models::{AlbumWithCount, PaginatedTracks, Track};

impl TrackRepository {
    pub fn get_all_albums(
        &self,
        query: Option<&str>,
        sort_column: Option<&str>,
        sort_dir: Option<&str>,
    ) -> SqliteResult<Vec<AlbumWithCount>> {
        let conn = self.conn.lock().unwrap();
        let base_sort = super::get_album_sort_clause(sort_column, sort_dir);

        if let Some(q) = query {
            let sort_clause = if sort_column.is_none() {
                format!(
                    "ORDER BY 
                    CASE 
                        WHEN al.name LIKE ?1 THEN 1
                        WHEN al.id IN (
                            SELECT aa.album_id FROM album_artists aa
                            JOIN artists ar3 ON ar3.id = aa.artist_id
                            WHERE ar3.name LIKE ?1
                        ) THEN 0
                        ELSE 2
                    END,
                    al.name COLLATE NOCASE"
                )
            } else {
                base_sort.clone()
            };

            let pattern = format!("%{}%", q);
            let sql = format!(
                "SELECT al.id, al.name, 
                        COALESCE(al.cover_hash, (
                            SELECT t2.cover_hash FROM tracks t2 
                            WHERE t2.album_id = al.id AND t2.cover_hash IS NOT NULL 
                            ORDER BY t2.release_date DESC, t2.added_at DESC LIMIT 1
                        )) AS cover_hash,
                        COUNT(DISTINCT t.id) AS track_count,
                        (SELECT GROUP_CONCAT(name, '; ') FROM (
                            SELECT DISTINCT ar.name
                            FROM track_artists ta
                            JOIN artists ar ON ar.id = ta.artist_id
                            WHERE ta.track_id IN (SELECT t2.id FROM tracks t2 WHERE t2.album_id = al.id)
                            ORDER BY ar.name
                        )) AS artist_name,
                        (SELECT GROUP_CONCAT(artist_id, ';') FROM album_artists WHERE album_id = al.id) AS album_artist_ids,
                        (SELECT GROUP_CONCAT(ar.name, ';') FROM album_artists aa JOIN artists ar ON ar.id = aa.artist_id WHERE aa.album_id = al.id) AS album_artist_names,
                        COALESCE(al.release_date, (SELECT MAX(t2.release_date) FROM tracks t2 WHERE t2.album_id = al.id)) AS release_date,
                        al.mbid
                 FROM albums al
                 JOIN tracks t ON t.album_id = al.id
                 WHERE t.title LIKE ?1
                    OR al.name LIKE ?1
                    OR t.id IN (
                        SELECT ta2.track_id FROM track_artists ta2
                        JOIN artists ar2 ON ar2.id = ta2.artist_id
                        WHERE ar2.name LIKE ?1
                    )
                 GROUP BY al.id
                 {}", sort_clause
            );

            let mut stmt = conn.prepare(&sql)?;

            let rows = stmt.query_map([&pattern], |row| {
                let artist_ids_raw: Option<String> = row.get(5)?;
                let album_artist_ids = artist_ids_raw
                    .map(|s| s.split(';').map(|id| id.to_string()).collect())
                    .unwrap_or_default();

                let artist_names_raw: Option<String> = row.get(6)?;
                let album_artist_names = artist_names_raw
                    .map(|s| s.split(';').map(|n| n.to_string()).collect())
                    .unwrap_or_default();

                Ok(AlbumWithCount {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    cover_hash: row.get(2)?,
                    track_count: row.get(3)?,
                    artist_name: row.get(4)?,
                    album_artist_ids,
                    album_artist_names,
                    release_date: row.get(7)?,
                    mbid: row.get(8)?,
                })
            })?;
            return rows.collect();
        }

        let sql = format!(
            "SELECT al.id, al.name, 
                    COALESCE(al.cover_hash, (
                        SELECT t2.cover_hash FROM tracks t2 
                        WHERE t2.album_id = al.id AND t2.cover_hash IS NOT NULL 
                        ORDER BY t2.release_date DESC, t2.added_at DESC LIMIT 1
                    )) AS cover_hash,
                    COUNT(t.id) AS track_count,
                    (SELECT GROUP_CONCAT(name, '; ') FROM (
                        SELECT DISTINCT ar.name
                        FROM track_artists ta
                        JOIN artists ar ON ar.id = ta.artist_id
                        WHERE ta.track_id IN (SELECT t2.id FROM tracks t2 WHERE t2.album_id = al.id)
                        ORDER BY ar.name
                    )) AS artist_name,
                    (SELECT GROUP_CONCAT(artist_id, ';') FROM album_artists WHERE album_id = al.id) AS album_artist_ids,
                    (SELECT GROUP_CONCAT(ar.name, ';') FROM album_artists aa JOIN artists ar ON ar.id = aa.artist_id WHERE aa.album_id = al.id) AS album_artist_names,
                    COALESCE(al.release_date, (SELECT MAX(t2.release_date) FROM tracks t2 WHERE t2.album_id = al.id)) AS release_date,
                    al.mbid
             FROM albums al
             LEFT JOIN tracks t ON t.album_id = al.id
             GROUP BY al.id
             {}", base_sort
        );

        let mut stmt = conn.prepare(&sql)?;

        let rows = stmt.query_map([], |row| {
            let artist_ids_raw: Option<String> = row.get(5)?;
            let album_artist_ids = artist_ids_raw
                .map(|s| s.split(';').map(|id| id.to_string()).collect())
                .unwrap_or_default();

            let artist_names_raw: Option<String> = row.get(6)?;
            let album_artist_names = artist_names_raw
                .map(|s| s.split(';').map(|n| n.to_string()).collect())
                .unwrap_or_default();

            Ok(AlbumWithCount {
                id: row.get(0)?,
                name: row.get(1)?,
                cover_hash: row.get(2)?,
                track_count: row.get(3)?,
                artist_name: row.get(4)?,
                album_artist_ids,
                album_artist_names,
                release_date: row.get(7)?,
                mbid: row.get(8)?,
            })
        })?;

        rows.collect()
    }

    pub fn get_tracks_by_album(&self, album_id: &str) -> SqliteResult<Vec<Track>> {
        let conn = self.conn.lock().unwrap();
        let sql = format!(
            "{} WHERE t.album_id = ?1
             GROUP BY t.id
             ORDER BY t.disc_number, t.track_number, t.title",
            TRACK_SELECT
        );

        let mut stmt = conn.prepare(&sql)?;
        let tracks = stmt.query_map([album_id], row_to_track)?;
        tracks.collect()
    }

    pub fn get_tracks_by_album_paginated(
        &self,
        album_id: &str,
        limit: i64,
        offset: i64,
        sort_column: Option<&str>,
        sort_dir: Option<&str>,
    ) -> SqliteResult<PaginatedTracks> {
        let conn = self.conn.lock().unwrap();

        let total: i64 = conn.query_row(
            "SELECT COUNT(*) FROM tracks WHERE album_id = ?1",
            [album_id],
            |row| row.get(0),
        )?;

        let sort_clause = if sort_column.is_none() {
            "ORDER BY t.disc_number ASC, t.track_number ASC".to_string()
        } else {
            super::get_track_sort_clause(sort_column, sort_dir)
        };

        let sql = format!(
            "{} WHERE t.album_id = ?1
             GROUP BY t.id
             {}
             LIMIT ?2 OFFSET ?3",
            TRACK_SELECT, sort_clause
        );

        let mut stmt = conn.prepare(&sql)?;
        let tracks: Vec<Track> = stmt
            .query_map(rusqlite::params![album_id, limit, offset], row_to_track)?
            .collect::<SqliteResult<Vec<Track>>>()?;

        Ok(PaginatedTracks { tracks, total })
    }

    pub fn get_albums_by_artist(
        &self,
        artist_id: &str,
        query: Option<&str>,
        sort_column: Option<&str>,
        sort_dir: Option<&str>,
    ) -> SqliteResult<Vec<AlbumWithCount>> {
        let conn = self.conn.lock().unwrap();
        let base_sort = super::get_album_sort_clause(sort_column, sort_dir);

        if let Some(q) = query {
            let sort_clause = if sort_column.is_none() {
                format!("ORDER BY 
                    CASE WHEN al.id IN (SELECT album_id FROM album_artists WHERE artist_id = ?1) THEN 0 ELSE 1 END,
                    al.name COLLATE NOCASE")
            } else {
                base_sort.clone()
            };

            let pattern = format!("%{}%", q);
            let sql = format!(
                "SELECT al.id, al.name, 
                        COALESCE(al.cover_hash, (
                            SELECT t2.cover_hash FROM tracks t2 
                            WHERE t2.album_id = al.id AND t2.cover_hash IS NOT NULL 
                            ORDER BY t2.release_date DESC, t2.added_at DESC LIMIT 1
                        )) AS cover_hash,
                        COUNT(t.id) AS track_count,
                        (SELECT GROUP_CONCAT(name, '; ') FROM (
                            SELECT DISTINCT ar.name
                            FROM track_artists ta
                            JOIN artists ar ON ar.id = ta.artist_id
                            WHERE ta.track_id IN (SELECT t2.id FROM tracks t2 WHERE t2.album_id = al.id)
                            ORDER BY ar.name
                        )) AS artist_name,
                        (SELECT GROUP_CONCAT(artist_id, ';') FROM album_artists WHERE album_id = al.id) AS album_artist_ids,
                        (SELECT GROUP_CONCAT(ar.name, ';') FROM album_artists aa JOIN artists ar ON ar.id = aa.artist_id WHERE aa.album_id = al.id) AS album_artist_names,
                        COALESCE(al.release_date, (SELECT MAX(t2.release_date) FROM tracks t2 WHERE t2.album_id = al.id)) AS release_date,
                        al.mbid
                 FROM albums al
                 JOIN tracks t ON t.album_id = al.id
                 WHERE t.id IN (
                    SELECT ta.track_id FROM track_artists ta WHERE ta.artist_id = ?1
                 )
                 AND (
                    t.title LIKE ?2
                    OR al.name LIKE ?2
                    OR t.id IN (
                        SELECT ta2.track_id FROM track_artists ta2
                        JOIN artists ar2 ON ar2.id = ta2.artist_id
                        WHERE ar2.name LIKE ?2
                    )
                 )
                 GROUP BY al.id
                 {}", sort_clause
            );

            let mut stmt = conn.prepare(&sql)?;

            let rows = stmt.query_map(rusqlite::params![artist_id, pattern], |row| {
                let artist_ids_raw: Option<String> = row.get(5)?;
                let album_artist_ids = artist_ids_raw
                    .map(|s| s.split(';').map(|id| id.to_string()).collect())
                    .unwrap_or_default();

                let artist_names_raw: Option<String> = row.get(6)?;
                let album_artist_names = artist_names_raw
                    .map(|s| s.split(';').map(|n| n.to_string()).collect())
                    .unwrap_or_default();

                Ok(AlbumWithCount {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    cover_hash: row.get(2)?,
                    track_count: row.get(3)?,
                    artist_name: row.get(4)?,
                    album_artist_ids,
                    album_artist_names,
                    release_date: row.get(7)?,
                    mbid: row.get(8)?,
                })
            })?;
            return rows.collect();
        }

        let sql = format!(
            "SELECT al.id, al.name, 
                    COALESCE(al.cover_hash, (
                        SELECT t2.cover_hash FROM tracks t2 
                        WHERE t2.album_id = al.id AND t2.cover_hash IS NOT NULL 
                        ORDER BY t2.release_date DESC, t2.added_at DESC LIMIT 1
                    )) AS cover_hash,
                    COUNT(t.id) AS track_count,
                    (SELECT GROUP_CONCAT(name, '; ') FROM (
                        SELECT DISTINCT ar.name
                        FROM track_artists ta
                        JOIN artists ar ON ar.id = ta.artist_id
                        WHERE ta.track_id IN (SELECT t2.id FROM tracks t2 WHERE t2.album_id = al.id)
                        ORDER BY ar.name
                    )) AS artist_name,
                    (SELECT GROUP_CONCAT(artist_id, ';') FROM album_artists WHERE album_id = al.id) AS album_artist_ids,
                    (SELECT GROUP_CONCAT(ar.name, ';') FROM album_artists aa JOIN artists ar ON ar.id = aa.artist_id WHERE aa.album_id = al.id) AS album_artist_names,
                    COALESCE(al.release_date, (SELECT MAX(t2.release_date) FROM tracks t2 WHERE t2.album_id = al.id)) AS release_date,
                    al.mbid
             FROM albums al
             JOIN tracks t ON t.album_id = al.id
             WHERE t.id IN (
                SELECT ta.track_id FROM track_artists ta WHERE ta.artist_id = ?1
             )
             GROUP BY al.id
             {}", base_sort
        );

        let mut stmt = conn.prepare(&sql)?;

        let rows = stmt.query_map([artist_id], |row| {
            let artist_ids_raw: Option<String> = row.get(5)?;
            let album_artist_ids = artist_ids_raw
                .map(|s| s.split(';').map(|id| id.to_string()).collect())
                .unwrap_or_default();

            let artist_names_raw: Option<String> = row.get(6)?;
            let album_artist_names = artist_names_raw
                .map(|s| s.split(';').map(|n| n.to_string()).collect())
                .unwrap_or_default();

            Ok(AlbumWithCount {
                id: row.get(0)?,
                name: row.get(1)?,
                cover_hash: row.get(2)?,
                track_count: row.get(3)?,
                artist_name: row.get(4)?,
                album_artist_ids,
                album_artist_names,
                release_date: row.get(7)?,
                mbid: row.get(8)?,
            })
        })?;

        rows.collect()
    }

    pub fn get_albums_by_genre(
        &self,
        genre_id: &str,
        query: Option<&str>,
        sort_column: Option<&str>,
        sort_dir: Option<&str>,
    ) -> SqliteResult<Vec<AlbumWithCount>> {
        let conn = self.conn.lock().unwrap();
        let base_sort = super::get_album_sort_clause(sort_column, sort_dir);

        if let Some(q) = query {
            let sort_clause = if sort_column.is_none() {
                "ORDER BY al.name COLLATE NOCASE".to_string()
            } else {
                base_sort.clone()
            };

            let pattern = format!("%{}%", q);
            let sql = format!(
                "SELECT al.id, al.name, 
                        COALESCE(al.cover_hash, (
                            SELECT t2.cover_hash FROM tracks t2 
                            WHERE t2.album_id = al.id AND t2.cover_hash IS NOT NULL 
                            ORDER BY t2.release_date DESC, t2.added_at DESC LIMIT 1
                        )) AS cover_hash,
                        COUNT(t.id) AS track_count,
                        (SELECT GROUP_CONCAT(name, '; ') FROM (
                            SELECT DISTINCT ar.name
                            FROM track_artists ta
                            JOIN artists ar ON ar.id = ta.artist_id
                            WHERE ta.track_id IN (SELECT t2.id FROM tracks t2 WHERE t2.album_id = al.id)
                            ORDER BY ar.name
                        )) AS artist_name,
                        (SELECT GROUP_CONCAT(artist_id, ';') FROM album_artists WHERE album_id = al.id) AS album_artist_ids,
                        (SELECT GROUP_CONCAT(ar.name, ';') FROM album_artists aa JOIN artists ar ON ar.id = aa.artist_id WHERE aa.album_id = al.id) AS album_artist_names,
                        COALESCE(al.release_date, (SELECT MAX(t2.release_date) FROM tracks t2 WHERE t2.album_id = al.id)) AS release_date,
                        al.mbid
                 FROM albums al
                 JOIN tracks t ON t.album_id = al.id
                 WHERE t.genre_id = ?1
                 AND (
                    t.title LIKE ?2
                    OR al.name LIKE ?2
                    OR t.id IN (
                        SELECT ta2.track_id FROM track_artists ta2
                        JOIN artists ar2 ON ar2.id = ta2.artist_id
                        WHERE ar2.name LIKE ?2
                    )
                 )
                 GROUP BY al.id
                 {}", sort_clause
            );

            let mut stmt = conn.prepare(&sql)?;

            let rows = stmt.query_map(rusqlite::params![genre_id, pattern], |row| {
                let artist_ids_raw: Option<String> = row.get(5)?;
                let album_artist_ids = artist_ids_raw
                    .map(|s| s.split(';').map(|id| id.to_string()).collect())
                    .unwrap_or_default();

                let artist_names_raw: Option<String> = row.get(6)?;
                let album_artist_names = artist_names_raw
                    .map(|s| s.split(';').map(|n| n.to_string()).collect())
                    .unwrap_or_default();

                Ok(AlbumWithCount {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    cover_hash: row.get(2)?,
                    track_count: row.get(3)?,
                    artist_name: row.get(4)?,
                    album_artist_ids,
                    album_artist_names,
                    release_date: row.get(7)?,
                    mbid: row.get(8)?,
                })
            })?;
            return rows.collect();
        }

        let sql = format!(
            "SELECT al.id, al.name, 
                    COALESCE(al.cover_hash, (
                        SELECT t2.cover_hash FROM tracks t2 
                        WHERE t2.album_id = al.id AND t2.cover_hash IS NOT NULL 
                        ORDER BY t2.release_date DESC, t2.added_at DESC LIMIT 1
                    )) AS cover_hash,
                    COUNT(t.id) AS track_count,
                    (SELECT GROUP_CONCAT(name, '; ') FROM (
                        SELECT DISTINCT ar.name
                        FROM track_artists ta
                        JOIN artists ar ON ar.id = ta.artist_id
                        WHERE ta.track_id IN (SELECT t2.id FROM tracks t2 WHERE t2.album_id = al.id)
                        ORDER BY ar.name
                    )) AS artist_name,
                    (SELECT GROUP_CONCAT(artist_id, ';') FROM album_artists WHERE album_id = al.id) AS album_artist_ids,
                    (SELECT GROUP_CONCAT(ar.name, ';') FROM album_artists aa JOIN artists ar ON ar.id = aa.artist_id WHERE aa.album_id = al.id) AS album_artist_names,
                    COALESCE(al.release_date, (SELECT MAX(t2.release_date) FROM tracks t2 WHERE t2.album_id = al.id)) AS release_date,
                    al.mbid
             FROM albums al
             JOIN tracks t ON t.album_id = al.id
             WHERE t.genre_id = ?1
             GROUP BY al.id
             {}", base_sort
        );

        let mut stmt = conn.prepare(&sql)?;

        let rows = stmt.query_map([genre_id], |row| {
            let artist_ids_raw: Option<String> = row.get(5)?;
            let album_artist_ids = artist_ids_raw
                .map(|s| s.split(';').map(|id| id.to_string()).collect())
                .unwrap_or_default();

            let artist_names_raw: Option<String> = row.get(6)?;
            let album_artist_names = artist_names_raw
                .map(|s| s.split(';').map(|n| n.to_string()).collect())
                .unwrap_or_default();

            Ok(AlbumWithCount {
                id: row.get(0)?,
                name: row.get(1)?,
                cover_hash: row.get(2)?,
                track_count: row.get(3)?,
                artist_name: row.get(4)?,
                album_artist_ids,
                album_artist_names,
                release_date: row.get(7)?,
                mbid: row.get(8)?,
            })
        })?;

        rows.collect()
    }

    pub fn get_album_by_id(&self, album_id: &str) -> SqliteResult<Option<AlbumWithCount>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT al.id, al.name, 
                    COALESCE(al.cover_hash, (
                        SELECT t2.cover_hash FROM tracks t2 
                        WHERE t2.album_id = al.id AND t2.cover_hash IS NOT NULL 
                        ORDER BY t2.release_date DESC, t2.added_at DESC LIMIT 1
                    )) AS cover_hash,
                    COUNT(t.id) AS track_count,
                    (SELECT GROUP_CONCAT(name, '; ') FROM (
                        SELECT DISTINCT ar.name
                        FROM track_artists ta
                        JOIN artists ar ON ar.id = ta.artist_id
                        WHERE ta.track_id IN (SELECT t2.id FROM tracks t2 WHERE t2.album_id = al.id)
                        ORDER BY ar.name
                    )) AS artist_name,
                    (SELECT GROUP_CONCAT(artist_id, ';') FROM album_artists WHERE album_id = al.id) AS album_artist_ids,
                    (SELECT GROUP_CONCAT(ar.name, ';') FROM album_artists aa JOIN artists ar ON ar.id = aa.artist_id WHERE aa.album_id = al.id) AS album_artist_names,
                    COALESCE(al.release_date, (SELECT MAX(t2.release_date) FROM tracks t2 WHERE t2.album_id = al.id)) AS release_date,
                    al.mbid
             FROM albums al
             LEFT JOIN tracks t ON t.album_id = al.id
             WHERE al.id = ?1
             GROUP BY al.id",
        )?;

        let mut rows = stmt.query_map([album_id], |row| {
            let artist_ids_raw: Option<String> = row.get(5)?;
            let album_artist_ids = artist_ids_raw
                .map(|s| s.split(';').map(|id| id.to_string()).collect())
                .unwrap_or_default();

            let artist_names_raw: Option<String> = row.get(6)?;
            let album_artist_names = artist_names_raw
                .map(|s| s.split(';').map(|n| n.to_string()).collect())
                .unwrap_or_default();

            Ok(AlbumWithCount {
                id: row.get(0)?,
                name: row.get(1)?,
                cover_hash: row.get(2)?,
                track_count: row.get(3)?,
                artist_name: row.get(4)?,
                album_artist_ids,
                album_artist_names,
                release_date: row.get(7)?,
                mbid: row.get(8)?,
            })
        })?;

        match rows.next() {
            Some(row) => Ok(Some(row?)),
            None => Ok(None),
        }
    }

    #[allow(dead_code)]
    pub fn update_album_cover(&self, album_id: &str, cover_hash: Option<&str>) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE albums SET cover_hash = ?1 WHERE id = ?2",
            rusqlite::params![cover_hash, album_id],
        )?;
        Ok(())
    }
}
