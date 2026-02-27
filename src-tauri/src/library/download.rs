use crate::cache::CacheManager;
use crate::cache::cover_art::CoverArtCache;
use crate::database::{Track, TrackRepository};
use crate::library::scanner;
use lofty::file::TaggedFileExt;
use lofty::prelude::AudioFile;
use lofty::tag::{Accessor, ItemKey};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::State;
use tauri::{AppHandle, Emitter, command};
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

pub struct DownloadStateManager {
    pub tokens: Arc<Mutex<HashMap<String, CancellationToken>>>,
}

#[derive(Clone, Serialize)]
pub struct DownloadProgress {
    pub download_id: String,
    pub current: u64,
    pub total: Option<u64>,
    pub status: String,
}

#[command]
pub async fn download_track(
    app_handle: AppHandle,
    state: State<'_, DownloadStateManager>,
    url: String,
    download_id: String,
    temp_path: String,
) -> Result<String, String> {
    let cancel_token = CancellationToken::new();
    {
        let mut tokens = state.tokens.lock().await;
        tokens.insert(download_id.clone(), cancel_token.clone());
    }

    let client = Client::new();
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!(
            "Download failed with status: {}",
            response.status()
        ));
    }

    let total = response.content_length();
    let mut current: u64 = 0;

    let path = PathBuf::from(&temp_path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let mut file = std::fs::File::create(&path).map_err(|e| e.to_string())?;

    let mut stream = response.bytes_stream();
    use futures_util::StreamExt;

    while let Some(item) = stream.next().await {
        if cancel_token.is_cancelled() {
            let _ = std::fs::remove_file(&path);
            return Err("Download cancelled".to_string());
        }

        let chunk = item.map_err(|e: reqwest::Error| e.to_string())?;
        file.write_all(&chunk).map_err(|e| e.to_string())?;
        current += chunk.len() as u64;

        let _ = app_handle.emit(
            "download-progress",
            DownloadProgress {
                download_id: download_id.clone(),
                current,
                total,
                status: "downloading".to_string(),
            },
        );
    }

    {
        let mut tokens = state.tokens.lock().await;
        tokens.remove(&download_id);
    }

    Ok(temp_path)
}

#[command]
pub async fn download_mpd_track(
    app_handle: AppHandle,
    state: State<'_, DownloadStateManager>,
    init_url: String,
    media_template: String,
    start_number: u32,
    download_id: String,
    temp_path: String,
) -> Result<String, String> {
    let cancel_token = CancellationToken::new();
    {
        let mut tokens = state.tokens.lock().await;
        tokens.insert(download_id.clone(), cancel_token.clone());
    }

    let client = Client::new();
    let path = PathBuf::from(&temp_path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let mut file = std::fs::File::create(&path).map_err(|e| e.to_string())?;

    let mut current: u64 = 0;
    let mut segment_index = start_number;
    let mut is_init = true;

    loop {
        if cancel_token.is_cancelled() {
            let _ = std::fs::remove_file(&path);
            return Err("Download cancelled".to_string());
        }

        let current_url = if is_init {
            init_url.clone()
        } else {
            media_template.replace("$Number$", &segment_index.to_string())
        };

        let response = match client.get(&current_url).send().await {
            Ok(res) if res.status().is_success() => res,
            _ => break,
        };

        let bytes = response.bytes().await.map_err(|e| e.to_string())?;
        if bytes.is_empty() {
            break;
        }

        file.write_all(&bytes).map_err(|e| e.to_string())?;
        current += bytes.len() as u64;

        let _ = app_handle.emit(
            "download-progress",
            DownloadProgress {
                download_id: download_id.clone(),
                current,
                total: None,
                status: "downloading".to_string(),
            },
        );

        if is_init {
            is_init = false;
        } else {
            segment_index += 1;
        }

        if segment_index > start_number + 500 {
            break;
        }
    }

    {
        let mut tokens = state.tokens.lock().await;
        tokens.remove(&download_id);
    }

    Ok(temp_path)
}

#[command]
pub async fn cancel_download(
    state: State<'_, DownloadStateManager>,
    download_id: String,
) -> Result<(), String> {
    let mut tokens = state.tokens.lock().await;
    if let Some(token) = tokens.remove(&download_id) {
        token.cancel();
    }
    Ok(())
}

#[derive(Deserialize)]
pub struct ImportMetadata {
    pub title: Option<String>,
    pub artists: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub genre: Option<String>,
    pub track_number: Option<i32>,
    pub disc_number: Option<i32>,
    pub release_date: Option<String>,
    pub cover_url: Option<String>,
}

#[command]
pub async fn import_downloaded_track(
    temp_path: String,
    target_folder: String,
    metadata: ImportMetadata,
    db: State<'_, Arc<TrackRepository>>,
    cache_manager: State<'_, Arc<CacheManager>>,
) -> Result<Track, String> {
    let temp_path = std::path::Path::new(&temp_path);
    if !temp_path.exists() {
        return Err("Temporary file not found".to_string());
    }

    let artists = metadata
        .artists
        .clone()
        .unwrap_or_else(|| "Unknown Artist".to_string());
    let title = metadata
        .title
        .clone()
        .unwrap_or_else(|| "Unknown Track".to_string());

    let safe_filename = format!("{} - {}.flac", artists, title)
        .replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");
    let target_dir = std::path::Path::new(&target_folder);
    if !target_dir.exists() {
        std::fs::create_dir_all(target_dir).map_err(|e| e.to_string())?;
    }

    let target_path = target_dir.join(safe_filename);

    if let Err(e) = std::fs::rename(temp_path, &target_path) {
        if e.raw_os_error() == Some(18) {
            std::fs::copy(temp_path, &target_path).map_err(|e| e.to_string())?;
            let _ = std::fs::remove_file(temp_path);
        } else {
            return Err(e.to_string());
        }
    }

    let mut tagged_file = lofty::probe::Probe::open(&target_path)
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

    if let Some(ref t) = metadata.title {
        tag.set_title(t.clone());
    }
    if let Some(ref a) = metadata.artists {
        tag.set_artist(a.clone());
    }

    let album_name = metadata.album.as_ref().filter(|s| !s.trim().is_empty());

    if let Some(al) = album_name {
        tag.set_album(al.clone());
        if let Some(n) = metadata.track_number {
            tag.set_track(n as u32);
        }
        if let Some(n) = metadata.disc_number {
            tag.set_disk(n as u32);
        }
        if let Some(ref aa) = metadata.album_artist {
            tag.insert_text(ItemKey::AlbumArtist, aa.clone());
        }
    } else {
        tag.remove_album();
        tag.remove_track();
        tag.remove_disk();
        tag.remove_key(ItemKey::AlbumArtist);
    }

    if let Some(ref g) = metadata.genre {
        tag.set_genre(g.clone());
    }

    if let Some(ref rd) = metadata.release_date {
        tag.insert_text(ItemKey::ReleaseDate, rd.clone());
    }

    //embed
    if let Some(ref cover_url) = metadata.cover_url {
        if let Ok(response) = reqwest::get(cover_url).await {
            if let Ok(bytes) = response.bytes().await {
                let mut cursor = std::io::Cursor::new(bytes.to_vec());
                if let Ok(picture) = lofty::picture::Picture::from_reader(&mut cursor) {
                    tag.push_picture(picture);
                }
            }
        }
    }

    tagged_file
        .save_to_path(&target_path, lofty::config::WriteOptions::default())
        .map_err(|e| e.to_string())?;

    let scan_result = scanner::scan_file(&target_path)
        .ok_or_else(|| "Failed to scan imported file".to_string())?;

    let mut track = scan_result.track;

    track.artists = metadata.artists;
    track.album = album_name.cloned();

    if track.album.is_some() {
        track.album_artist = metadata.album_artist;
        track.track_number = metadata.track_number;
        track.disc_number = metadata.disc_number;
    } else {
        track.album_artist = None;
        track.track_number = None;
        track.disc_number = None;
    }

    track.title = metadata.title;
    track.release_date = metadata.release_date;
    track.genre = metadata.genre;

    // cache
    if let Some(cover_data) = scan_result.cover_data {
        let cover_cache = CoverArtCache::new(cache_manager.cover_arts_dir())?;
        let hash = cover_cache.hash_cover_data(&cover_data);
        if !cover_cache.has_thumbnail(&hash) {
            let _ = cover_cache.save_thumbnail_with_hash(&cover_data, &hash);
        }
        track.cover_hash = Some(hash);
    }

    db.insert_batch(&[track.clone()])
        .map_err(|e| e.to_string())?;

    Ok(track)
}

#[command]
pub async fn write_file_bytes(path: String, bytes: Vec<u8>) -> Result<(), String> {
    let path = std::path::Path::new(&path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    std::fs::write(path, bytes).map_err(|e| e.to_string())
}
