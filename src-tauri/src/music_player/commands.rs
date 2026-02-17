use std::{fs::File, time::Duration};

use rodio::{Decoder, Sink};
use tauri::State;

use super::state::{CurrentTrack, PlayerState};

#[tauri::command]
pub async fn play_file(
    path: String,
    track_data: Option<CurrentTrack>,
    player: State<'_, PlayerState>,
) -> Result<(), String> {
    let source = tokio::task::spawn_blocking(move || {
        let file = File::open(&path).map_err(|e| format!("Failed to open file: {}", e))?;
        Decoder::new(std::io::BufReader::new(file)).map_err(|e| format!("Decode failed: {}", e))
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))??;

    let mut state = player.inner.lock().map_err(|e| e.to_string())?;

    let sink = Sink::connect_new(&player.stream.mixer());
    sink.append(source);
    sink.play();

    state.sink = Some(sink);
    state.current_track = track_data;
    state.reset_position();

    Ok(())
}

#[tauri::command]
pub fn set_volume(volume: f32, player: State<PlayerState>) -> Result<(), String> {
    let state = player.inner.lock().map_err(|e| e.to_string())?;

    if let Some(ref sink) = state.sink {
        sink.set_volume(volume);
    }

    Ok(())
}

#[tauri::command]
pub fn get_current_track(player: State<PlayerState>) -> Result<Option<CurrentTrack>, String> {
    let state = player.inner.lock().map_err(|e| e.to_string())?;
    Ok(state.current_track.clone())
}

#[tauri::command]
pub fn pause(player: State<PlayerState>) -> Result<(), String> {
    let mut state = player.inner.lock().map_err(|e| e.to_string())?;

    if let Some(ref sink) = state.sink {
        sink.pause();
        state.pause();
    }

    Ok(())
}

#[tauri::command]
pub fn resume(player: State<PlayerState>) -> Result<(), String> {
    let mut state = player.inner.lock().map_err(|e| e.to_string())?;

    if let Some(ref sink) = state.sink {
        sink.play();
        state.resume();
    }

    Ok(())
}

#[tauri::command]
pub fn stop(player: State<PlayerState>) -> Result<(), String> {
    let mut state = player.inner.lock().map_err(|e| e.to_string())?;

    if let Some(sink) = state.sink.take() {
        sink.stop();
    }

    state.clear();

    Ok(())
}

#[tauri::command]
pub async fn seek(position: f64, player: State<'_, PlayerState>) -> Result<(), String> {
    let duration = Duration::from_secs_f64(position);

    let (current_pos, was_paused, needs_reload, track_data) = {
        let state = player.inner.lock().map_err(|e| e.to_string())?;
        let needs_reload = state.sink.as_ref().map_or(false, |s| s.empty());
        let current_pos = state.position();
        let was_paused = state.is_paused;
        let track_data = state.current_track.clone();
        (current_pos, was_paused, needs_reload, track_data)
    };

    let is_backward = position < current_pos;

    if needs_reload || is_backward {
        let track = track_data.ok_or("No track data available to reload")?;
        play_file(track.path.clone(), Some(track), player.clone()).await?;
    }

    let mut state = player.inner.lock().map_err(|e| e.to_string())?;

    let seek_result = state
        .sink
        .as_ref()
        .ok_or_else(|| "No track currently playing".to_string())?
        .try_seek(duration);

    match seek_result {
        Ok(_) => {
            state.seek(position);
            if was_paused {
                if let Some(ref sink) = state.sink {
                    sink.pause();
                }
                state.pause();
            }
        }
        Err(e) => return Err(format!("Seek failed: {}", e)),
    }

    Ok(())
}

#[tauri::command]
pub fn get_position(player: State<PlayerState>) -> Result<f64, String> {
    let state = player.inner.lock().map_err(|e| e.to_string())?;
    Ok(state.position())
}
