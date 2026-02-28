use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use rayon::prelude::*;
use serde::Serialize;
use tauri::{AppHandle, Emitter, State, command};

use crate::cache::CacheManager;
use crate::cache::cover_art::CoverArtCache;
use crate::database::{
    AlbumWithCount, ArtistWithCount, GenreWithCount, LibraryStats, PaginatedTracks, Track,
    TrackRepository,
};
use lofty::file::TaggedFileExt;
use lofty::prelude::AudioFile;
use lofty::tag::{Accessor, ItemKey};

use super::scanner;
use crate::settings::database::DEFAULT_PAGE_LIMIT;

#[derive(Clone, Serialize)]
struct ScanProgress {
    current: usize,
    total: usize,
    current_file: String,
}

#[command]
pub async fn scan_and_save_library(
    paths: Vec<String>,
    app_handle: AppHandle,
    db: State<'_, Arc<TrackRepository>>,
    cache_manager: State<'_, Arc<CacheManager>>,
) -> Result<usize, String> {
    let music_dirs: Vec<PathBuf> = paths.into_iter().map(PathBuf::from).collect();

    let audio_files = scanner::find_audio_files(&music_dirs);
    let total = audio_files.len();

    if total == 0 {
        return Ok(0);
    }

    db.prepare_for_bulk_insert()
        .map_err(|e: rusqlite::Error| format!("Failed to prepare for bulk insert: {}", e))?;

    let cover_cache = Arc::new(CoverArtCache::new(cache_manager.cover_arts_dir())?);
    let progress_counter = Arc::new(AtomicUsize::new(0));
    let processed_hashes = Arc::new(Mutex::new(HashSet::new()));

    let tracks: Vec<Track> = audio_files
        .into_par_iter()
        .filter_map(|path| {
            let mut result = scanner::scan_file(&path)?;
            let current = progress_counter.fetch_add(1, Ordering::Relaxed) + 1;

            if current % 10 == 0 || current == total {
                let _ = app_handle.emit(
                    "scan-progress",
                    ScanProgress {
                        current,
                        total,
                        current_file: result.track.path.clone(),
                    },
                );
            }

            if let Some(cover_data) = result.cover_data.take() {
                let hash = cover_cache.hash_cover_data(&cover_data);
                let already_cached = cover_cache.has_thumbnail(&hash);

                let is_new = {
                    let mut hashes = processed_hashes.lock().unwrap();
                    hashes.insert(hash.clone())
                };

                if is_new && !already_cached {
                    if let Err(e) = cover_cache.save_thumbnail_with_hash(&cover_data, &hash) {
                        eprintln!(
                            "Thumbnail generation failed for {:?}: {}",
                            result.track.title, e
                        );
                    }
                }

                result.track.cover_hash = Some(hash);
            }

            Some(result.track)
        })
        .collect();

    let inserted = db
        .insert_batch(&tracks)
        .map_err(|e: rusqlite::Error| format!("Failed to insert tracks: {}", e))?;

    db.finish_bulk_insert()
        .map_err(|e: rusqlite::Error| format!("Failed to recreate indices: {}", e))?;

    Ok(inserted)
}

#[command]
pub fn get_all_tracks(db: State<'_, Arc<TrackRepository>>) -> Result<Vec<Track>, String> {
    db.get_all().map_err(|e: rusqlite::Error| e.to_string())
}

#[command]
pub fn get_random_track(db: State<'_, Arc<TrackRepository>>) -> Result<Option<Track>, String> {
    db.get_random_track().map_err(|e| format!("DB error: {}", e))
}

#[command]
pub fn get_track_by_id(
    id: String,
    db: State<'_, Arc<TrackRepository>>,
) -> Result<Option<Track>, String> {
    db.get_by_id(&id)
        .map_err(|e: rusqlite::Error| e.to_string())
}

#[command]
pub fn search_tracks(
    query: String,
    db: State<'_, Arc<TrackRepository>>,
) -> Result<Vec<Track>, String> {
    db.search(&query).map_err(|e| format!("Search error: {}", e))
}

#[command]
pub async fn delete_track_file(id: String, db: State<'_, Arc<TrackRepository>>) -> Result<(), String> {
    let track = db.get_by_id(&id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Track not found".to_string())?;

    let path = std::path::Path::new(&track.path);
    if path.exists() {
        std::fs::remove_file(path).map_err(|e| format!("Failed to delete file: {}", e))?;
    }

    db.delete(&id).map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub fn delete_track(id: String, db: State<'_, Arc<TrackRepository>>) -> Result<(), String> {
    db.delete(&id).map_err(|e| format!("Delete failed: {}", e))
}

#[command]
pub fn clear_library(db: State<'_, Arc<TrackRepository>>) -> Result<(), String> {
    db.clear().map_err(|e: rusqlite::Error| e.to_string())
}

#[command]
pub fn get_library_stats(db: State<'_, Arc<TrackRepository>>) -> Result<LibraryStats, String> {
    db.get_stats().map_err(|e: rusqlite::Error| e.to_string())
}

#[command]
pub fn get_all_artists(
    query: Option<String>,
    db: State<'_, Arc<TrackRepository>>,
) -> Result<Vec<ArtistWithCount>, String> {
    db.get_all_artists(query.as_deref()).map_err(|e| e.to_string())
}

#[command]
pub fn get_tracks_by_artist(
    artist_id: String,
    db: State<'_, Arc<TrackRepository>>,
) -> Result<Vec<Track>, String> {
    db.get_tracks_by_artist(&artist_id)
        .map_err(|e: rusqlite::Error| e.to_string())
}

#[command]
pub fn get_all_albums(
    query: Option<String>,
    sort_column: Option<String>,
    sort_dir: Option<String>,
    db: State<'_, Arc<TrackRepository>>,
) -> Result<Vec<AlbumWithCount>, String> {
    db.get_all_albums(
        query.as_deref(),
        sort_column.as_deref(),
        sort_dir.as_deref(),
    )
    .map_err(|e: rusqlite::Error| e.to_string())
}

#[command]
pub fn get_tracks_by_album(
    album_id: String,
    db: State<'_, Arc<TrackRepository>>,
) -> Result<Vec<Track>, String> {
    db.get_tracks_by_album(&album_id)
        .map_err(|e: rusqlite::Error| e.to_string())
}

#[command]
pub fn get_albums_by_artist(
    artist_id: String,
    query: Option<String>,
    sort_column: Option<String>,
    sort_dir: Option<String>,
    db: State<'_, Arc<TrackRepository>>,
) -> Result<Vec<AlbumWithCount>, String> {
    db.get_albums_by_artist(
        &artist_id,
        query.as_deref(),
        sort_column.as_deref(),
        sort_dir.as_deref(),
    )
    .map_err(|e: rusqlite::Error| e.to_string())
}

#[command]
pub fn get_albums_by_genre(
    genre_id: String,
    query: Option<String>,
    sort_column: Option<String>,
    sort_dir: Option<String>,
    db: State<'_, Arc<TrackRepository>>,
) -> Result<Vec<AlbumWithCount>, String> {
    db.get_albums_by_genre(
        &genre_id,
        query.as_deref(),
        sort_column.as_deref(),
        sort_dir.as_deref(),
    )
    .map_err(|e: rusqlite::Error| e.to_string())
}

#[command]
pub fn get_all_genres(
    query: Option<String>,
    db: State<'_, Arc<TrackRepository>>,
) -> Result<Vec<GenreWithCount>, String> {
    db.get_all_genres(query.as_deref())
        .map_err(|e: rusqlite::Error| e.to_string())
}

#[command]
pub fn get_tracks_by_genre(
    genre_id: String,
    db: State<'_, Arc<TrackRepository>>,
) -> Result<Vec<Track>, String> {
    db.get_tracks_by_genre(&genre_id)
        .map_err(|e: rusqlite::Error| e.to_string())
}

#[command]
pub fn get_artist_by_name(
    name: String,
    db: State<'_, Arc<TrackRepository>>,
) -> Result<Option<ArtistWithCount>, String> {
    db.get_artist_by_name(&name)
        .map_err(|e: rusqlite::Error| e.to_string())
}

#[command]
pub fn get_album_details(
    album_id: String,
    db: State<'_, Arc<TrackRepository>>,
) -> Result<Option<AlbumWithCount>, String> {
    db.get_album_by_id(&album_id)
        .map_err(|e: rusqlite::Error| e.to_string())
}

#[command]
pub fn get_tracks_page(
    limit: i64,
    offset: i64,
    sort_column: Option<String>,
    sort_dir: Option<String>,
    db: State<'_, Arc<TrackRepository>>,
) -> Result<PaginatedTracks, String> {
    let actual_limit = if limit <= 0 {
        DEFAULT_PAGE_LIMIT
    } else {
        limit
    };
    db.get_all_paginated(
        actual_limit,
        offset,
        sort_column.as_deref(),
        sort_dir.as_deref(),
    )
    .map_err(|e: rusqlite::Error| e.to_string())
}

#[command]
pub fn search_tracks_paginated(
    query: String,
    limit: i64,
    offset: i64,
    sort_column: Option<String>,
    sort_dir: Option<String>,
    db: State<'_, Arc<TrackRepository>>,
) -> Result<PaginatedTracks, String> {
    let actual_limit = if limit <= 0 {
        DEFAULT_PAGE_LIMIT
    } else {
        limit
    };
    db.search_paginated(
        &query,
        actual_limit,
        offset,
        sort_column.as_deref(),
        sort_dir.as_deref(),
    )
    .map_err(|e: rusqlite::Error| e.to_string())
}

#[command]
pub fn get_tracks_by_artist_page(
    artist_id: String,
    limit: i64,
    offset: i64,
    sort_column: Option<String>,
    sort_dir: Option<String>,
    db: State<'_, Arc<TrackRepository>>,
) -> Result<PaginatedTracks, String> {
    let actual_limit = if limit <= 0 {
        DEFAULT_PAGE_LIMIT
    } else {
        limit
    };
    db.get_tracks_by_artist_paginated(
        &artist_id,
        actual_limit,
        offset,
        sort_column.as_deref(),
        sort_dir.as_deref(),
    )
    .map_err(|e: rusqlite::Error| e.to_string())
}

#[command]
pub fn get_tracks_by_album_page(
    album_id: String,
    limit: i64,
    offset: i64,
    sort_column: Option<String>,
    sort_dir: Option<String>,
    db: State<'_, Arc<TrackRepository>>,
) -> Result<PaginatedTracks, String> {
    let actual_limit = if limit <= 0 {
        DEFAULT_PAGE_LIMIT
    } else {
        limit
    };
    db.get_tracks_by_album_paginated(
        &album_id,
        actual_limit,
        offset,
        sort_column.as_deref(),
        sort_dir.as_deref(),
    )
    .map_err(|e: rusqlite::Error| e.to_string())
}

#[command]
pub fn get_tracks_by_genre_page(
    genre_id: String,
    limit: i64,
    offset: i64,
    sort_column: Option<String>,
    sort_dir: Option<String>,
    db: State<'_, Arc<TrackRepository>>,
) -> Result<PaginatedTracks, String> {
    let actual_limit = if limit <= 0 {
        DEFAULT_PAGE_LIMIT
    } else {
        limit
    };
    db.get_tracks_by_genre_paginated(
        &genre_id,
        actual_limit,
        offset,
        sort_column.as_deref(),
        sort_dir.as_deref(),
    )
    .map_err(|e: rusqlite::Error| e.to_string())
}

#[command]
pub async fn get_cover_art(
    id: String,
    size: Option<u32>,
    db: State<'_, Arc<TrackRepository>>,
    cache_manager: State<'_, Arc<CacheManager>>,
) -> Result<String, String> {
    let track = db
        .get_by_id(&id)
        .map_err(|e: rusqlite::Error| e.to_string())?
        .ok_or_else(|| format!("Track not found: {}", id))?;

    let hash = track
        .cover_hash
        .ok_or_else(|| "Track has no cover art".to_string())?;

    let cover_dir = cache_manager.cover_arts_dir();
    let track_path = track.path.clone();

    let data = tokio::task::spawn_blocking(move || {
        let cover_cache = CoverArtCache::new(cover_dir)?;

        if let Some(requested_size) = size {
            if requested_size > 128 {
                let original_data =
                    scanner::get_cover_art_from_file(std::path::Path::new(&track_path))
                        .ok_or_else(|| "Failed to extract cover from file".to_string())?;

                return cover_cache.read_cover_art(
                    &hash,
                    Some(&original_data),
                    Some(requested_size),
                );
            }
        }

        cover_cache.read_cover_art(&hash, None, size)
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))??;

    Ok(base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        data,
    ))
}

#[command]
pub async fn get_image_by_hash(
    hash: String,
    size: Option<u32>,
    cache_manager: State<'_, Arc<CacheManager>>,
) -> Result<String, String> {
    let cover_dir = cache_manager.cover_arts_dir();

    let data = tokio::task::spawn_blocking(move || {
        let cover_cache = CoverArtCache::new(cover_dir)?;
        cover_cache.read_cover_art(&hash, None, size)
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))??;

    Ok(base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        data,
    ))
}

#[command]
pub async fn update_track_metadata(
    track_id: String,
    title: Option<String>,
    artists: Option<String>,
    album: Option<String>,
    album_artist: Option<String>,
    track_number: Option<i32>,
    disc_number: Option<i32>,
    release_date: Option<String>,
    genre: Option<String>,
    db: State<'_, Arc<TrackRepository>>,
) -> Result<Track, String> {
    let mut track = db
        .get_by_id(&track_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Track not found".to_string())?;

    let path = std::path::Path::new(&track.path);
    if !path.exists() {
        return Err("File does not exist".to_string());
    }

    let mut tagged_file = lofty::probe::Probe::open(path)
        .map_err(|e| e.to_string())?
        .read()
        .map_err(|e| e.to_string())?;

    if tagged_file.primary_tag().is_none() && tagged_file.first_tag().is_none() {
        let tag_type = tagged_file.primary_tag_type();
        tagged_file.insert_tag(lofty::tag::Tag::new(tag_type));
    }

    let tag = if tagged_file.primary_tag().is_some() {
        tagged_file.primary_tag_mut().unwrap()
    } else {
        tagged_file.first_tag_mut().unwrap()
    };

    if let Some(ref t) = title {
        tag.set_title(t.clone());
    }
    if let Some(ref a) = artists {
        tag.set_artist(a.clone());
    }
    if let Some(ref al) = album {
        tag.set_album(al.clone());
    }
    if let Some(n) = track_number {
        tag.set_track(n as u32);
    }
    if let Some(n) = disc_number {
        tag.set_disk(n as u32);
    }
    if let Some(ref g) = genre {
        tag.set_genre(g.clone());
    }

    if let Some(ref aa) = album_artist {
        tag.insert_text(ItemKey::AlbumArtist, aa.clone());
    } else {
        tag.remove_key(ItemKey::AlbumArtist);
    }

    if let Some(ref rd) = release_date {
        tag.insert_text(ItemKey::ReleaseDate, rd.clone());
    } else {
        tag.remove_key(ItemKey::ReleaseDate);
    }

    tagged_file
        .save_to_path(path, lofty::config::WriteOptions::default())
        .map_err(|e: lofty::error::LoftyError| e.to_string())?;

    track.title = title;
    track.artists = artists;
    track.album = album;
    track.album_artist = album_artist;
    track.track_number = track_number;
    track.disc_number = disc_number;
    track.release_date = release_date;
    track.genre = genre;

    db.update(&track).map_err(|e| e.to_string())?;

    Ok(track)
}
#[command]
pub fn get_subdirectories(path: String) -> Result<Vec<String>, String> {
    let path = std::path::Path::new(&path);
    if !path.exists() || !path.is_dir() {
        return Err("Invalid directory path".to_string());
    }

    let mut dirs = Vec::new();
    let entries = std::fs::read_dir(path).map_err(|e| e.to_string())?;

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if !name.starts_with('.') {
                        dirs.push(name.to_string());
                    }
                }
            }
        }
    }
    
    dirs.sort_by_key(|a| a.to_lowercase());
    Ok(dirs)
}

#[command]
pub async fn get_image_from_url(url: String) -> Result<String, String> {
    let data = tokio::task::spawn_blocking(move || {
        let client = reqwest::blocking::Client::new();
        let response = client.get(&url).send().map_err(|e| e.to_string())?;
        
        if !response.status().is_success() {
            return Err(format!("Failed to fetch image: HTTP {}", response.status()));
        }
        
        response.bytes().map(|b| b.to_vec()).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))??;

    Ok(base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        data,
    ))
}

#[command]
pub fn get_embedded_lyrics(path: String) -> Result<Option<String>, String> {
    let p = std::path::Path::new(&path);
    if !p.exists() {
        return Err("File does not exist".to_string());
    }

    let tagged_file = lofty::probe::Probe::open(p)
        .map_err(|e| e.to_string())?
        .read()
        .map_err(|e| e.to_string())?;

    if let Some(tag) = tagged_file.primary_tag().or(tagged_file.first_tag()) {
        if let Some(lyrics) = tag.get_string(ItemKey::Lyrics) {
            return Ok(Some(lyrics.to_string()));
        }
    }

    Ok(None)
}

#[command]
pub fn embed_lyrics(path: String, lyrics: String) -> Result<(), String> {
    let p = std::path::Path::new(&path);
    if !p.exists() {
        return Err("File does not exist".to_string());
    }

    let mut tagged_file = lofty::probe::Probe::open(p)
        .map_err(|e| e.to_string())?
        .read()
        .map_err(|e| e.to_string())?;

    if tagged_file.primary_tag().is_none() && tagged_file.first_tag().is_none() {
        let tag_type = tagged_file.primary_tag_type();
        tagged_file.insert_tag(lofty::tag::Tag::new(tag_type));
    }

    let tag = if tagged_file.primary_tag().is_some() {
        tagged_file.primary_tag_mut().unwrap()
    } else {
        tagged_file.first_tag_mut().unwrap()
    };

    tag.insert_text(ItemKey::Lyrics, lyrics);

    tagged_file
        .save_to_path(p, lofty::config::WriteOptions::default())
        .map_err(|e: lofty::error::LoftyError| e.to_string())?;

    Ok(())
}
