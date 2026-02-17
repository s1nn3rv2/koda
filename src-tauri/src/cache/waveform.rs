use std::fs;
use std::path::PathBuf;

use super::super::waveform::models::WaveformData;

pub struct WaveformCache {
    cache_dir: PathBuf,
}

impl WaveformCache {
    pub fn new(cache_dir: PathBuf) -> Result<Self, String> {
        fs::create_dir_all(&cache_dir).map_err(|e| format!("Failed to create cache dir: {}", e))?;
        Ok(Self { cache_dir })
    }

    fn get_cache_path(&self, track_id: &str) -> PathBuf {
        self.cache_dir.join(format!("{}.json", track_id))
    }

    pub fn has_cached(&self, track_id: &str) -> bool {
        self.get_cache_path(track_id).exists()
    }

    pub fn read(&self, track_id: &str) -> Result<WaveformData, String> {
        let cache_path = self.get_cache_path(track_id);
        let data = fs::read_to_string(&cache_path)
            .map_err(|e| format!("Failed to read waveform cache: {}", e))?;

        serde_json::from_str(&data).map_err(|e| format!("Failed to parse waveform cache: {}", e))
    }

    pub fn write(&self, track_id: &str, waveform: &WaveformData) -> Result<(), String> {
        let cache_path = self.get_cache_path(track_id);
        let json_data = serde_json::to_string(waveform)
            .map_err(|e| format!("Failed to serialize waveform: {}", e))?;

        fs::write(&cache_path, json_data)
            .map_err(|e| format!("Failed to write waveform cache: {}", e))
    }
}
