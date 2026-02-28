use rusqlite::Result as SqliteResult;
use std::collections::HashMap;

use super::helpers::{resolve_or_create_album, resolve_or_create_artist, resolve_or_create_genre};
use super::{row_to_track, TrackRepository, TRACK_SELECT};
use crate::database::models::{PaginatedTracks, Track};
use crate::database::schema;

impl TrackRepository {
    pub fn insert_batch(&self, tracks: &[Track]) -> SqliteResult<usize> {
        let mut conn = self.conn.lock().unwrap();

        let existing_ids: HashMap<String, String> = {
            let mut stmt = conn.prepare("SELECT path, id FROM tracks")?;
            let rows = stmt.query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?;
            rows.filter_map(|r| r.ok()).collect()
        };

        let tx = conn.transaction()?;

        let mut inserted = 0;
        for track in tracks {
            // avoid orphaned cache entries
            let track_id = existing_ids.get(&track.path).unwrap_or(&track.id);

            let album_id = if let Some(ref album_name) = track.album {
                let name = album_name.trim();
                if name.is_empty() {
                    None
                } else {
                    let aid = resolve_or_create_album(&tx, name)?;

                    tx.execute("DELETE FROM album_artists WHERE album_id = ?1", [&aid])?;

                    if let Some(ref aa_raw) = track.album_artist {
                        let names = schema::split_artist_names(aa_raw);
                        for aa_name in names {
                            let aa_id = resolve_or_create_artist(&tx, &aa_name)?;
                            tx.execute(
                                "INSERT OR IGNORE INTO album_artists (album_id, artist_id) VALUES (?1, ?2)",
                                rusqlite::params![&aid, aa_id],
                            )?;

                            tx.execute(
                                "UPDATE albums SET artist_id = ?1 WHERE id = ?2",
                                rusqlite::params![aa_id, &aid],
                            )?;
                        }
                    }

                    Some(aid)
                }
            } else {
                None
            };

            let genre_id = if let Some(ref genre_name) = track.genre {
                let name = genre_name.trim();
                if name.is_empty() {
                    None
                } else {
                    Some(resolve_or_create_genre(&tx, name)?)
                }
            } else {
                None
            };

            // in case of rescan
            tx.execute("DELETE FROM track_artists WHERE track_id = ?1", [track_id])?;

            tx.execute(
                "INSERT OR REPLACE INTO tracks
                    (id, path, title, album_id, genre_id, duration, track_number, disc_number, release_date, added_at, cover_hash)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                rusqlite::params![
                    track_id,
                    &track.path,
                    &track.title,
                    &album_id,
                    &genre_id,
                    &track.duration,
                    &track.track_number,
                    &track.disc_number,
                    &track.release_date,
                    &track.added_at,
                    &track.cover_hash,
                ],
            )?;

            if let Some(ref raw_artists) = track.artists {
                let names = schema::split_artist_names(raw_artists);
                for name in &names {
                    let artist_id = resolve_or_create_artist(&tx, name)?;
                    tx.execute(
                        "INSERT OR IGNORE INTO track_artists (track_id, artist_id) VALUES (?1, ?2)",
                        rusqlite::params![track_id, artist_id],
                    )?;
                }
            }

            inserted += 1;
        }

        tx.commit()?;
        Ok(inserted)
    }

    pub fn get_by_id(&self, id: &str) -> SqliteResult<Option<Track>> {
        let conn = self.conn.lock().unwrap();
        let sql = format!("{} WHERE t.id = ?1 GROUP BY t.id", TRACK_SELECT);
        let mut stmt = conn.prepare(&sql)?;
        let mut rows = stmt.query([id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(row_to_track(row)?))
        } else {
            Ok(None)
        }
    }

    pub fn get_all(&self) -> SqliteResult<Vec<Track>> {
        let conn = self.conn.lock().unwrap();
        let sql = format!(
            "{} GROUP BY t.id ORDER BY artists, al.name, t.track_number",
            TRACK_SELECT
        );
        let mut stmt = conn.prepare(&sql)?;
        let tracks = stmt.query_map([], row_to_track)?;
        tracks.collect()
    }

    pub fn get_all_paginated(
        &self,
        limit: i64,
        offset: i64,
        sort_column: Option<&str>,
        sort_dir: Option<&str>,
    ) -> SqliteResult<PaginatedTracks> {
        let conn = self.conn.lock().unwrap();

        let total: i64 = conn.query_row("SELECT COUNT(*) FROM tracks", [], |row| row.get(0))?;

        let sort_clause = super::get_track_sort_clause(sort_column, sort_dir);
        let sql = format!(
            "{} GROUP BY t.id {} LIMIT ?1 OFFSET ?2",
            TRACK_SELECT, sort_clause
        );
        let mut stmt = conn.prepare(&sql)?;
        let tracks: Vec<Track> = stmt
            .query_map(rusqlite::params![limit, offset], row_to_track)?
            .collect::<SqliteResult<Vec<_>>>()?;

        Ok(PaginatedTracks { tracks, total })
    }

    pub fn search(&self, query: &str) -> SqliteResult<Vec<Track>> {
        let conn = self.conn.lock().unwrap();
        
        let fts_query = query
            .split_whitespace()
            .filter(|w| !w.is_empty())
            .map(|w| format!("\"{}\"*", w.replace('"', "\"\"")))
            .collect::<Vec<_>>()
            .join(" ");

        if fts_query.is_empty() {
            return Ok(vec![]);
        }

        let sql = format!(
            "{} WHERE t.id IN (
                SELECT id FROM tracks_fts5 
                WHERE tracks_fts5 MATCH ?1
            )
             GROUP BY t.id
             ORDER BY artists, al.name, t.track_number",
            TRACK_SELECT
        );

        let mut stmt = conn.prepare(&sql)?;
        let tracks = stmt.query_map([&fts_query], row_to_track)?;
        tracks.collect()
    }

    pub fn search_paginated(
        &self,
        query: &str,
        limit: i64,
        offset: i64,
        sort_column: Option<&str>,
        sort_dir: Option<&str>,
    ) -> SqliteResult<PaginatedTracks> {
        let conn = self.conn.lock().unwrap();
        
        let fts_query = query
            .split_whitespace()
            .filter(|w| !w.is_empty())
            .map(|w| format!("\"{}\"*", w.replace('"', "\"\"")))
            .collect::<Vec<_>>()
            .join(" ");

        if fts_query.is_empty() {
            return Ok(PaginatedTracks { tracks: vec![], total: 0 });
        }

        let total: i64 = conn.query_row(
            "SELECT COUNT(*) FROM tracks_fts5 WHERE tracks_fts5 MATCH ?1",
            [&fts_query],
            |row| row.get(0),
        )?;

        let base_sort = super::get_track_sort_clause(sort_column, sort_dir);
        let sort_clause = if sort_column.is_none() {
            "ORDER BY rank".to_string()
        } else {
            base_sort
        };

        let sql = format!(
            "{} WHERE t.id IN (
                SELECT id FROM tracks_fts5 
                WHERE tracks_fts5 MATCH ?1
            )
             GROUP BY t.id
             {}
             LIMIT ?2 OFFSET ?3",
            TRACK_SELECT, sort_clause
        );

        let mut stmt = conn.prepare(&sql)?;
        let tracks: Vec<Track> = stmt
            .query_map(rusqlite::params![&fts_query, limit, offset], row_to_track)?
            .collect::<SqliteResult<Vec<_>>>()?;

        Ok(PaginatedTracks { tracks, total })
    }

    pub fn delete(&self, id: &str) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM tracks WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn get_random_track(&self) -> SqliteResult<Option<Track>> {
        let conn = self.conn.lock().unwrap();
        let sql = format!("{} GROUP BY t.id ORDER BY RANDOM() LIMIT 1", TRACK_SELECT);
        let mut stmt = conn.prepare(&sql)?;
        let mut rows = stmt.query([])?;

        if let Some(row) = rows.next()? {
            Ok(Some(row_to_track(row)?))
        } else {
            Ok(None)
        }
    }

    pub fn clear(&self) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "DELETE FROM track_artists;
             DELETE FROM tracks;
             DELETE FROM albums;
             DELETE FROM artists;
             DELETE FROM genres;",
        )?;
        Ok(())
    }

    pub fn update(&self, track: &Track) -> SqliteResult<()> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction()?;

        let album_id = if let Some(ref album_name) = track.album {
            let name = album_name.trim();
            if name.is_empty() {
                None
            } else {
                let aid = resolve_or_create_album(&tx, name)?;

                tx.execute("DELETE FROM album_artists WHERE album_id = ?1", [&aid])?;

                if let Some(ref aa_raw) = track.album_artist {
                    let names = schema::split_artist_names(aa_raw);
                    for aa_name in names {
                        let aa_id = resolve_or_create_artist(&tx, &aa_name)?;
                        tx.execute(
                            "INSERT OR IGNORE INTO album_artists (album_id, artist_id) VALUES (?1, ?2)",
                            rusqlite::params![&aid, aa_id],
                        )?;

                        tx.execute(
                            "UPDATE albums SET artist_id = ?1 WHERE id = ?2",
                            rusqlite::params![aa_id, &aid],
                        )?;
                    }
                }

                Some(aid)
            }
        } else {
            None
        };

        let genre_id = if let Some(ref genre_name) = track.genre {
            let name = genre_name.trim();
            if name.is_empty() {
                None
            } else {
                Some(resolve_or_create_genre(&tx, name)?)
            }
        } else {
            None
        };

        tx.execute(
            "UPDATE tracks SET
                title = ?1, album_id = ?2, genre_id = ?3, track_number = ?4,
                disc_number = ?5, release_date = ?6
             WHERE id = ?7",
            rusqlite::params![
                &track.title,
                &album_id,
                &genre_id,
                &track.track_number,
                &track.disc_number,
                &track.release_date,
                &track.id,
            ],
        )?;

        // update track_artists
        tx.execute("DELETE FROM track_artists WHERE track_id = ?1", [&track.id])?;
        if let Some(ref raw_artists) = track.artists {
            let names = schema::split_artist_names(raw_artists);
            for name in &names {
                let artist_id = resolve_or_create_artist(&tx, name)?;
                tx.execute(
                    "INSERT OR IGNORE INTO track_artists (track_id, artist_id) VALUES (?1, ?2)",
                    rusqlite::params![&track.id, artist_id],
                )?;
            }
        }

        tx.commit()?;
        Ok(())
    }
}
