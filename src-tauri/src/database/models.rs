use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: String,
    pub path: String,
    pub title: Option<String>,
    pub artists: Option<String>,
    pub album: Option<String>,
    pub duration: Option<i64>,
    pub track_number: Option<i32>,
    pub added_at: i64,
    pub cover_hash: Option<String>, // sha256 hash for deduplication
}

impl Track {
    pub fn new(path: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            path,
            title: None,
            artists: None,
            album: None,
            duration: None,
            track_number: None,
            added_at: chrono::Utc::now().timestamp(),
            cover_hash: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryStats {
    pub total_tracks: i64,
    pub total_duration: i64,
    pub unique_artists: i64,
    pub unique_albums: i64,
}
