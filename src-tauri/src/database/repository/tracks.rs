use rusqlite::Result as SqliteResult;
use std::collections::HashMap;

use super::helpers::{resolve_or_create_album, resolve_or_create_artist, resolve_or_create_genre};
use super::{TRACK_SELECT, TrackRepository, row_to_track};
use crate::database::models::Track;
use crate::database::schema;

impl TrackRepository {
    pub fn insert_batch(&self, tracks: &[Track]) -> SqliteResult<usize> {
        let mut conn = self.conn.lock().unwrap();

        // build a map of existing path -> id so we can preserve IDs on rescan
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
                    Some(resolve_or_create_album(
                        &tx,
                        name,
                        track.cover_hash.as_deref(),
                    )?)
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
                    (id, path, title, album_id, genre_id, duration, track_number, added_at, cover_hash)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                rusqlite::params![
                    track_id,
                    &track.path,
                    &track.title,
                    &album_id,
                    &genre_id,
                    &track.duration,
                    &track.track_number,
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

    pub fn search(&self, query: &str) -> SqliteResult<Vec<Track>> {
        let conn = self.conn.lock().unwrap();
        let pattern = format!("%{}%", query);

        let sql = format!(
            "{} WHERE t.title LIKE ?1
                OR al.name LIKE ?1
                OR t.id IN (
                    SELECT ta2.track_id FROM track_artists ta2
                    JOIN artists ar2 ON ar2.id = ta2.artist_id
                    WHERE ar2.name LIKE ?1
                )
             GROUP BY t.id
             ORDER BY artists, al.name, t.track_number",
            TRACK_SELECT
        );

        let mut stmt = conn.prepare(&sql)?;
        let tracks = stmt.query_map([&pattern], row_to_track)?;
        tracks.collect()
    }

    pub fn delete(&self, id: &str) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        // track_artists rows are deleted by ON DELETE CASCADE
        conn.execute("DELETE FROM tracks WHERE id = ?1", [id])?;
        Ok(())
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
}
