use std::collections::HashSet;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use rayon::prelude::*;
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, State};

use crate::cache::CacheManager;
use crate::cache::cover_art::CoverArtCache;
use crate::database::{LibraryStats, Track, TrackRepository};

use super::scanner;

#[derive(Clone, Serialize)]
struct ScanProgress {
    current: usize,
    total: usize,
    current_file: String,
}

#[tauri::command]
pub async fn scan_and_save_library(
    app_handle: AppHandle,
    db: State<'_, TrackRepository>,
    cache_manager: State<'_, CacheManager>,
) -> Result<usize, String> {
    let music_dir = app_handle
        .path()
        .audio_dir()
        .map_err(|e: tauri::Error| e.to_string())?;

    let audio_files = scanner::find_audio_files(&music_dir);
    let total = audio_files.len();

    if total == 0 {
        return Ok(0);
    }

    db.prepare_for_bulk_insert()
        .map_err(|e: rusqlite::Error| format!("Failed to prepare for bulk insert: {}", e))?;

    let scan_results = scanner::scan_files_parallel(&audio_files);

    let cover_cache = CoverArtCache::new(cache_manager.cover_arts_dir())?;
    let progress_counter = Arc::new(AtomicUsize::new(0));
    let processed_hashes = Arc::new(Mutex::new(HashSet::new()));
    let covers_processed = Arc::new(AtomicUsize::new(0));

    let tracks: Vec<Track> = scan_results
        .into_par_iter()
        .map(|mut result| {
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

            if let Some(cover_data) = result.cover_data {
                let hash = cover_cache.hash_cover_data(&cover_data);
                let already_cached = cover_cache.has_thumbnail(&hash);
                let is_new = processed_hashes.lock().unwrap().insert(hash.clone());

                if is_new && !already_cached {
                    if cover_cache
                        .save_thumbnail_with_hash(&cover_data, &hash)
                        .is_ok()
                    {
                        covers_processed.fetch_add(1, Ordering::Relaxed);
                    }
                }

                result.track.cover_hash = Some(hash);
            }

            result.track
        })
        .collect();

    let inserted = db
        .insert_batch(&tracks)
        .map_err(|e: rusqlite::Error| format!("Failed to insert tracks: {}", e))?;

    db.finish_bulk_insert()
        .map_err(|e: rusqlite::Error| format!("Failed to recreate indices: {}", e))?;

    Ok(inserted)
}

#[tauri::command]
pub fn get_all_tracks(db: State<'_, TrackRepository>) -> Result<Vec<Track>, String> {
    db.get_all().map_err(|e: rusqlite::Error| e.to_string())
}

#[tauri::command]
pub fn get_track_by_id(
    id: String,
    db: State<'_, TrackRepository>,
) -> Result<Option<Track>, String> {
    db.get_by_id(&id)
        .map_err(|e: rusqlite::Error| e.to_string())
}

#[tauri::command]
pub fn search_tracks(query: String, db: State<'_, TrackRepository>) -> Result<Vec<Track>, String> {
    db.search(&query)
        .map_err(|e: rusqlite::Error| e.to_string())
}

#[tauri::command]
pub fn delete_track(id: String, db: State<'_, TrackRepository>) -> Result<(), String> {
    db.delete(&id).map_err(|e: rusqlite::Error| e.to_string())
}

#[tauri::command]
pub fn clear_library(db: State<'_, TrackRepository>) -> Result<(), String> {
    db.clear().map_err(|e: rusqlite::Error| e.to_string())
}

#[tauri::command]
pub fn get_library_stats(db: State<'_, TrackRepository>) -> Result<LibraryStats, String> {
    db.get_stats().map_err(|e: rusqlite::Error| e.to_string())
}

#[tauri::command]
pub async fn get_cover_art(
    id: String,
    size: Option<u32>,
    db: State<'_, TrackRepository>,
    cache_manager: State<'_, CacheManager>,
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
