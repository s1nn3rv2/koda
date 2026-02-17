use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: String,
    pub path: String,
    pub title: Option<String>,
    pub artists: Option<String>,
    pub album: Option<String>,
    pub album_id: Option<String>,
    pub genre: Option<String>,
    pub genre_id: Option<String>,
    pub duration: Option<i64>,
    pub track_number: Option<i32>,
    pub added_at: i64,
    pub cover_hash: Option<String>,
}

impl Track {
    pub fn new(path: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            path,
            title: None,
            artists: None,
            album: None,
            album_id: None,
            genre: None,
            genre_id: None,
            duration: None,
            track_number: None,
            added_at: chrono::Utc::now().timestamp(),
            cover_hash: None,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artist {
    pub id: String,
    pub name: String,
}

#[allow(dead_code)]
impl Artist {
    pub fn new(name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub cover_hash: Option<String>,
}

#[allow(dead_code)]
impl Album {
    pub fn new(name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            cover_hash: None,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genre {
    pub id: String,
    pub name: String,
}

#[allow(dead_code)]
impl Genre {
    pub fn new(name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistWithCount {
    pub id: String,
    pub name: String,
    pub track_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumWithCount {
    pub id: String,
    pub name: String,
    pub cover_hash: Option<String>,
    pub track_count: i64,
    pub artist_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenreWithCount {
    pub id: String,
    pub name: String,
    pub track_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryStats {
    pub total_tracks: i64,
    pub total_duration: i64,
    pub unique_artists: i64,
    pub unique_albums: i64,
    pub unique_genres: i64,
}
