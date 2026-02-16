use tauri::Manager;
use walkdir::WalkDir;

#[tauri::command]
pub fn list_music_files(app_handle: tauri::AppHandle) -> Result<Vec<String>, String> {
    let music_dir = app_handle.path().audio_dir().map_err(|e| e.to_string())?;

    let mut file_names = Vec::new();
    for entry in WalkDir::new(music_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() {
            if let Some(path) = entry.path().to_str() {
                file_names.push(path.to_string());
            }
        }
    }

    Ok(file_names)
}
