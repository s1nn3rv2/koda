use crate::cache::CacheManager;
use crate::cache::cover_art::CoverArtCache;
use crate::database::TrackRepository;
use crate::sources::{itunes, musicbrainz};
use lazy_static::lazy_static;
use musicbrainz_rs::client::MusicBrainzClient;
use serde::Deserialize;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use tauri::State;

lazy_static! {
    static ref ONGOING_ALBUM_FETCHES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
    static ref ONGOING_ARTIST_FETCHES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MetadataProvider {
    MusicBrainz,
    ITunes,
    Auto,
}

impl From<Option<String>> for MetadataProvider {
    fn from(s: Option<String>) -> Self {
        match s.as_deref() {
            Some("musicbrainz") => MetadataProvider::MusicBrainz,
            Some("itunes") => MetadataProvider::ITunes,
            _ => MetadataProvider::Auto,
        }
    }
}

#[tauri::command]
pub async fn fetch_artist_metadata(
    artist_id: String,
    provider: Option<String>,
    force: Option<bool>,
    db: State<'_, Arc<TrackRepository>>,
    mb_client: State<'_, Arc<MusicBrainzClient>>,
    cache_manager: State<'_, Arc<CacheManager>>,
) -> Result<Option<String>, String> {
    println!(
        "[Metadata] fetch_artist_metadata called for ID: {} with provider: {:?}, force: {:?}",
        artist_id, provider, force
    );
    let p = MetadataProvider::from(provider);
    let force = force.unwrap_or(false);

    loop {
        let is_ongoing = {
            let ongoing = ONGOING_ARTIST_FETCHES.lock().unwrap();
            ongoing.contains(&artist_id)
        };

        if !is_ongoing {
            break;
        }

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        let artists = db.get_all_artists(None).map_err(|e| e.to_string())?;
        if let Some(artist) = artists.into_iter().find(|a| a.id == artist_id) {
            if !force && artist.image_hash.is_some() && p == MetadataProvider::Auto {
                return Ok(artist.image_hash);
            }
        }
    }

    {
        let mut ongoing = ONGOING_ARTIST_FETCHES.lock().unwrap();
        ongoing.insert(artist_id.clone());
    }

    let _guard = scopeguard::guard(artist_id.clone(), |id| {
        ONGOING_ARTIST_FETCHES.lock().unwrap().remove(&id);
    });

    let artist = db
        .get_all_artists(None)
        .map_err(|e| e.to_string())?
        .into_iter()
        .find(|a| a.id == artist_id)
        .ok_or_else(|| "Artist not found".to_string())?;

    if !force && artist.image_hash.is_some() && p == MetadataProvider::Auto {
        return Ok(artist.image_hash);
    }

    let mut final_hash = artist.image_hash.clone();
    let mut current_mbid = artist.mbid.clone();

    if (p == MetadataProvider::Auto || p == MetadataProvider::MusicBrainz)
        && (force || final_hash.is_none() || p != MetadataProvider::Auto)
    {
        println!("[Metadata] Trying MusicBrainz for artist: {}", artist.name);
        if current_mbid.is_none() {
            if let Some(metadata) =
                musicbrainz::search_artist_metadata(&mb_client, &artist.name).await
            {
                current_mbid = Some(metadata.mbid.clone());
                let conn = db.conn.lock().unwrap();
                conn.execute(
                    "UPDATE artists SET mbid = ?1 WHERE id = ?2",
                    rusqlite::params![metadata.mbid, artist.id],
                )
                .ok();
            }
        }

        if let Some(ref _mb_id) = current_mbid {
            if let Some(metadata) =
                musicbrainz::search_artist_metadata(&mb_client, &artist.name).await
            {
                if let Some(image_url) = metadata.image_url {
                    final_hash =
                        save_artist_image(&artist.id, &image_url, &db, &cache_manager).await;
                }
            }
        }
    }

    if (p == MetadataProvider::Auto || p == MetadataProvider::ITunes)
        && (force || final_hash.is_none() || p != MetadataProvider::Auto)
    {
        println!("[Metadata] Trying iTunes for artist: {}", artist.name);
        if let Some(image_url) = itunes::search_artist_image(&artist.name).await {
            final_hash = save_artist_image(&artist.id, &image_url, &db, &cache_manager).await;
        }
    }

    Ok(final_hash)
}

async fn save_artist_image(
    artist_id: &str,
    image_url: &str,
    db: &TrackRepository,
    cache_manager: &CacheManager,
) -> Option<String> {
    if let Some(bytes) = musicbrainz::download_image(image_url).await {
        if let Ok(cover_cache) = CoverArtCache::new(cache_manager.cover_arts_dir()) {
            let hash = cover_cache.hash_cover_data(&bytes);
            if cover_cache.save_thumbnail_with_hash(&bytes, &hash).is_ok() {
                let conn = db.conn.lock().unwrap();
                conn.execute(
                    "UPDATE artists SET image_hash = ?1 WHERE id = ?2",
                    rusqlite::params![Some(&hash), artist_id],
                )
                .ok();
                println!(
                    "[Metadata] Successfully saved artist image for ID: {}",
                    artist_id
                );
                return Some(hash);
            }
        }
    }
    None
}

async fn save_album_cover(
    album_id: &str,
    cover_url: &str,
    db: &TrackRepository,
    cache_manager: &CacheManager,
) -> Option<String> {
    if let Some(bytes) = musicbrainz::download_image(cover_url).await {
        if let Ok(cover_cache) = CoverArtCache::new(cache_manager.cover_arts_dir()) {
            let hash = cover_cache.hash_cover_data(&bytes);
            if cover_cache.save_thumbnail_with_hash(&bytes, &hash).is_ok() {
                let conn = db.conn.lock().unwrap();
                conn.execute(
                    "UPDATE albums SET cover_hash = ?1 WHERE id = ?2",
                    rusqlite::params![Some(&hash), album_id],
                )
                .ok();
                println!(
                    "[Metadata] Successfully saved album cover for ID: {}",
                    album_id
                );
                return Some(hash);
            }
        }
    }
    None
}

#[tauri::command]
pub async fn fetch_album_metadata(
    album_id: String,
    provider: Option<String>,
    force: Option<bool>,
    db: State<'_, Arc<TrackRepository>>,
    mb_client: State<'_, Arc<MusicBrainzClient>>,
    cache_manager: State<'_, Arc<CacheManager>>,
) -> Result<(Option<String>, Option<String>), String> {
    println!(
        "[Metadata] fetch_album_metadata called for ID: {} with provider: {:?}, force: {:?}",
        album_id, provider, force
    );
    let album = db
        .get_album_by_id(&album_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Album not found".to_string())?;

    let p = MetadataProvider::from(provider);
    let force = force.unwrap_or(false);

    let mut has_valid_cover = false;
    if let Some(ref hash) = album.cover_hash {
        has_valid_cover = CoverArtCache::new(cache_manager.cover_arts_dir())
            .map(|cache| cache.has_thumbnail(hash))
            .unwrap_or(false);
    }

    if !force && p == MetadataProvider::Auto && has_valid_cover && album.release_date.is_some() {
        println!(
            "[Metadata] Returning cached metadata for album: {}",
            album.name
        );
        return Ok((album.release_date, album.cover_hash));
    }

    println!(
        "[Metadata] Fetching fresh metadata for album: {} (Provider: {:?})",
        album.name, p
    );

    loop {
        let is_ongoing = {
            let ongoing = ONGOING_ALBUM_FETCHES.lock().unwrap();
            ongoing.contains(&album_id)
        };

        if !is_ongoing {
            break;
        }

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        if let Ok(Some(album)) = db.get_album_by_id(&album_id) {
            if !force
                && p == MetadataProvider::Auto
                && album.cover_hash.is_some()
                && album.release_date.is_some()
            {
                println!(
                    "[Metadata] Returning now-available cached metadata for album: {}",
                    album.name
                );
                return Ok((album.release_date, album.cover_hash));
            }
        }
    }

    {
        let mut ongoing = ONGOING_ALBUM_FETCHES.lock().unwrap();
        ongoing.insert(album_id.clone());
    }

    let _guard = scopeguard::guard(album_id.clone(), |id| {
        ONGOING_ALBUM_FETCHES.lock().unwrap().remove(&id);
    });

    let mut final_date = album.release_date.clone();
    let mut final_hash = album.cover_hash.clone();
    let mut current_mbid = album.mbid.clone();

    let search_artist = if !album.album_artist_names.is_empty() {
        Some(album.album_artist_names[0].trim().to_string())
    } else {
        album
            .artist_name
            .as_ref()
            .and_then(|s| s.split(';').next().map(|s| s.trim().to_string()))
    };

    if (p == MetadataProvider::Auto || p == MetadataProvider::MusicBrainz)
        && (force || final_hash.is_none() || p != MetadataProvider::Auto)
    {
        println!(
            "[Metadata] Trying MusicBrainz for album: {} - {:?}",
            album.name, search_artist
        );
        if current_mbid.is_none() {
            if let Some(ref artist_name) = search_artist {
                if let Some(metadata) =
                    musicbrainz::search_album_metadata(&mb_client, &album.name, artist_name).await
                {
                    current_mbid = Some(metadata.mbid.clone());

                    if let Some(ref new_date) = metadata.release_date {
                        final_date = Some(new_date.clone());
                    }

                    let conn = db.conn.lock().unwrap();
                    conn.execute(
                        "UPDATE albums SET mbid = ?1, release_date = COALESCE(?2, release_date) WHERE id = ?3",
                        rusqlite::params![current_mbid, final_date, album.id],
                    ).ok();
                }
            }
        }

        if let Some(ref mbid) = current_mbid {
            if final_hash.is_none() || p != MetadataProvider::Auto {
                if let Some(cover_url) =
                    musicbrainz::fetch_album_cover_by_id(&mb_client, mbid).await
                {
                    final_hash = save_album_cover(&album.id, &cover_url, &db, &cache_manager).await;
                }
            }
        }
    }

    if (p == MetadataProvider::Auto || p == MetadataProvider::ITunes)
        && (force || final_hash.is_none() || p != MetadataProvider::Auto)
    {
        if let Some(ref artist_name) = search_artist {
            println!(
                "[Metadata] Trying iTunes for album: {} - {}",
                artist_name, album.name
            );
            if let Some((itunes_url, release_date)) =
                itunes::search_album_metadata(artist_name, &album.name).await
            {
                let formatted_date = release_date.split('T').next().map(|s| s.to_string());
                if let Some(ref date) = formatted_date {
                    final_date = Some(date.clone());
                    let conn = db.conn.lock().unwrap();
                    conn.execute(
                        "UPDATE albums SET release_date = ?1 WHERE id = ?2",
                        rusqlite::params![final_date, album.id],
                    )
                    .ok();
                }

                final_hash = save_album_cover(&album.id, &itunes_url, &db, &cache_manager).await;
            }
        }
    }

    Ok((final_date, final_hash))
}
