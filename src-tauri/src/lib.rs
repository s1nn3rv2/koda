mod cache;
mod database;
mod library;
mod music_player;
mod waveform;

use rusqlite::Connection;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // initialize cache manager
            let cache_manager = cache::CacheManager::new(&app.handle())
                .expect("Failed to initialize cache manager");
            cache_manager
                .init()
                .expect("Failed to initialize cache directories");

            // initialize database
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");

            std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data directory");

            let db_path = app_data_dir.join("music.db");
            let conn = Connection::open(db_path).expect("Failed to open database");

            let db = database::TrackRepository::new(conn).expect("Failed to initialize database");

            app.manage(db);
            app.manage(cache_manager);

            // initialize player state
            let player = music_player::PlayerState::default();
            app.manage(player);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // library management
            library::scan_and_save_library,
            library::get_all_tracks,
            library::get_track_by_id,
            library::search_tracks,
            library::delete_track,
            library::clear_library,
            library::get_library_stats,
            library::get_cover_art,
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
