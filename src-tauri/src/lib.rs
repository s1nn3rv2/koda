mod music_library;
mod music_player;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .manage(music_player::player::PlayerState::default())
        .invoke_handler(tauri::generate_handler![
            music_library::list_music_files,
            music_player::player::play_file,
            music_player::player::pause,
            music_player::player::resume,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
