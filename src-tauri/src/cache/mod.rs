use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub mod cover_art;
pub mod waveform;

pub struct CacheManager {
    cache_dir: PathBuf,
}

impl CacheManager {
    pub fn new(app_handle: &AppHandle) -> Result<Self, String> {
        let cache_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e: tauri::Error| e.to_string())?
            .join("cache");

        std::fs::create_dir_all(&cache_dir)
            .map_err(|e| format!("Failed to create cache directory: {}", e))?;

        Ok(Self { cache_dir })
    }

    pub fn cover_arts_dir(&self) -> PathBuf {
        self.cache_dir.join("cover_arts")
    }

    pub fn waveforms_dir(&self) -> PathBuf {
        self.cache_dir.join("waveforms")
    }

    pub fn init(&self) -> Result<(), String> {
        std::fs::create_dir_all(self.cover_arts_dir())
            .map_err(|e| format!("Failed to create cover_arts directory: {}", e))?;

        std::fs::create_dir_all(self.waveforms_dir())
            .map_err(|e| format!("Failed to create waveforms directory: {}", e))?;

        Ok(())
    }

    pub fn clear_all(&self) -> Result<(), String> {
        std::fs::remove_dir_all(&self.cache_dir)
            .map_err(|e| format!("Failed to clear cache: {}", e))?;
        self.init()
    }
}
