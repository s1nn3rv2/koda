mod cache;
mod database;
mod library;
mod music_player;
mod settings;
mod sources;
mod waveform;

use tauri::Manager;
// use tauri_plugin_opener::OpenerExt;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use musicbrainz_rs::client::MusicBrainzClient;
use rusqlite::Connection;
use std::sync::Arc;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .register_uri_scheme_protocol("library-asset", move |app, request| {
            let uri = request.uri().path();
            let parts: Vec<&str> = uri.split('/').filter(|s| !s.is_empty()).collect();

            if parts.len() >= 2 {
                let hash = parts[0];
                let size = parts[1];

                if let Ok(app_data_dir) = app.app_handle().path().app_data_dir() {
                    let cover_arts_dir = app_data_dir.join("cache").join("cover_arts");

                    let mut file_path = cover_arts_dir.join(format!("{}_{}.jpg", hash, size));

                    if !file_path.exists() {
                        file_path = cover_arts_dir.join(format!("{}_thumb.jpg", hash));
                    }

                    if !file_path.exists() {
                        file_path = cover_arts_dir.join(format!("{}_128.jpg", hash));
                    }

                    if let Ok(data) = std::fs::read(file_path) {
                        return tauri::http::Response::builder()
                            .header("Content-Type", "image/jpeg")
                            .header("Cache-Control", "public, max-age=31536000, immutable")
                            .body(data)
                            .unwrap();
                    }
                }
            }

            tauri::http::Response::builder()
                .status(404)
                .body(Vec::new())
                .unwrap()
        })
        .setup(|app| {
            let cache_manager = Arc::new(
                cache::CacheManager::new(&app.handle())
                    .expect("Failed to initialize cache manager"),
            );
            cache_manager
                .init()
                .expect("Failed to initialize cache directories");

            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");

            let db_path = app_data_dir.join("music.db");
            let conn = Connection::open(db_path).expect("Failed to open database");

            let db = Arc::new(
                database::TrackRepository::new(conn).expect("Failed to initialize database"),
            );
            let mut client = MusicBrainzClient::default();
            client
                .set_user_agent("Koda/1.0.0 (https://github.com/s1nn3rv2/koda)")
                .expect("Failed to set musicbrainz user agent");

            let mb_client = Arc::new(client);

            app.manage(db);
            app.manage(cache_manager);
            app.manage(mb_client);

            let stream = rodio::OutputStreamBuilder::open_default_stream()
                .expect("Failed to initialize audio output");
            let stream: &'static _ = Box::leak(Box::new(stream));
            let mixer = stream.mixer();

            let player = music_player::PlayerState::new(mixer);
            app.manage(player);

            app.manage(library::download::DownloadStateManager {
                tokens: Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new())),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // library management
            library::scan_and_save_library,
            library::get_all_tracks,
            library::get_random_track,
            library::get_track_by_id,
            library::search_tracks,
            library::delete_track,
            library::delete_track_file,
            library::clear_library,
            library::get_library_stats,
            library::update_track_metadata,
            library::get_cover_art,
            library::get_image_by_hash,
            library::get_image_from_url,
            // browse by artist/album/genre
            library::get_all_artists,
            library::get_tracks_by_artist,
            library::get_all_albums,
            library::get_tracks_by_album,
            library::get_albums_by_artist,
            library::get_albums_by_genre,
            library::get_all_genres,
            library::get_tracks_by_genre,
            // lookup by name/id
            library::get_artist_by_name,
            library::get_album_details,
            // paginated
            library::get_tracks_page,
            library::search_tracks_paginated,
            library::get_tracks_by_artist_page,
            library::get_tracks_by_album_page,
            library::get_tracks_by_genre_page,
            library::download_track,
            library::download_mpd_track,
            library::cancel_download,
            library::import_downloaded_track,
            library::write_file_bytes,
            library::get_subdirectories,
            library::get_embedded_lyrics,
            library::embed_lyrics,
            sources::metadata::fetch_artist_metadata,
            sources::metadata::fetch_album_metadata,
            sources::lrclib::fetch_lyrics,
            // music player
            music_player::play_file,
            music_player::get_current_track,
            music_player::pause,
            music_player::resume,
            music_player::stop,
            music_player::set_volume,
            music_player::seek,
            music_player::get_position,
            // waveform generation
            waveform::get_waveform,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
