use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: String,
    pub path: String,
    pub title: Option<String>,
    pub artists: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub album_artist_ids: Vec<String>,
    pub album_id: Option<String>,
    pub genre: Option<String>,
    pub genre_id: Option<String>,
    pub duration: Option<i64>,
    pub track_number: Option<i32>,
    pub disc_number: Option<i32>,
    pub release_date: Option<String>,
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
            album_artist: None,
            album_artist_ids: Vec::new(),
            album_id: None,
            genre: None,
            genre_id: None,
            duration: None,
            track_number: None,
            disc_number: None,
            release_date: None,
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
    pub image_hash: Option<String>,
    pub mbid: Option<String>,
}

#[allow(dead_code)]
impl Artist {
    pub fn new(name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            image_hash: None,
            mbid: None,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub cover_hash: Option<String>,
    pub release_date: Option<String>,
    pub mbid: Option<String>,
}

#[allow(dead_code)]
impl Album {
    pub fn new(name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            cover_hash: None,
            release_date: None,
            mbid: None,
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
    pub image_hash: Option<String>,
    pub track_count: i64,
    pub album_count: i64,
    pub mbid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumWithCount {
    pub id: String,
    pub name: String,
    pub cover_hash: Option<String>,
    pub release_date: Option<String>,
    pub track_count: i64,
    pub artist_name: Option<String>,
    pub album_artist_ids: Vec<String>,
    pub album_artist_names: Vec<String>,
    pub mbid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenreWithCount {
    pub id: String,
    pub name: String,
    pub track_count: i64,
    pub album_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryStats {
    pub total_tracks: i64,
    pub total_duration: i64,
    pub unique_artists: i64,
    pub unique_albums: i64,
    pub unique_genres: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedTracks {
    pub tracks: Vec<Track>,
    pub total: i64,
}
