use std::sync::Arc;
use tauri::State;

use super::generator;
use crate::cache::CacheManager;
use crate::cache::waveform::WaveformCache;
use crate::database::TrackRepository;

#[tauri::command]
pub async fn get_waveform(
    id: String,
    db: State<'_, Arc<TrackRepository>>,
    cache_manager: State<'_, Arc<CacheManager>>,
) -> Result<super::models::WaveformData, String> {
    let track = db
        .get_by_id(&id)
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| format!("Track not found: {}", id))?;

    let waveform_cache = WaveformCache::new(cache_manager.waveforms_dir())?;

    if waveform_cache.has_cached(&id) {
        return waveform_cache.read(&id);
    }

    let track_path = track.path.clone();
    let waveform = tokio::task::spawn_blocking(move || generator::generate(&track_path))
        .await
        .map_err(|e| format!("Task failed: {}", e))??;

    waveform_cache.write(&id, &waveform)?;

    Ok(waveform)
}
