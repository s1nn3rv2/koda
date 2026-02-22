use std::time::Duration;

use rodio::{Decoder, Sink};
use tauri::{State, command};

use super::state::{CurrentTrack, PlayerState};

#[command]
pub async fn play_file(
    path: String,
    track_data: Option<CurrentTrack>,
    player: State<'_, PlayerState>,
) -> Result<(), String> {
    enum PlaySource {
        Url(Decoder<std::io::Cursor<Vec<u8>>>),
        File(Decoder<std::io::BufReader<std::fs::File>>),
    }

    let source_url = path.clone();

    let source = tokio::task::spawn_blocking(move || {
        if path.starts_with("http://") || path.starts_with("https://") {
            println!("Fetching remote stream...");
            let response =
                reqwest::blocking::get(&path).map_err(|e| format!("Failed to fetch URL: {}", e))?;
            let data = response
                .bytes()
                .map_err(|e| format!("Failed to read bytes: {}", e))?
                .to_vec();
            println!("Loaded {} bytes. Decoding...", data.len());
            let cursor = std::io::Cursor::new(data);
            let decoder = Decoder::try_from(cursor).map_err(|e| format!("Decode failed: {}", e))?;
            Ok::<_, String>(PlaySource::Url(decoder))
        } else {
            println!("Reading local file...");
            let file =
                std::fs::File::open(&path).map_err(|e| format!("Failed to open file: {}", e))?;
            let decoder = Decoder::try_from(file).map_err(|e| format!("Decode failed: {}", e))?;
            Ok::<_, String>(PlaySource::File(decoder))
        }
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))??;
    println!("Track decoded successfully. Updating player state...");

    let mut state = player.inner.lock().map_err(|e| e.to_string())?;

    if let Some(old_sink) = state.sink.take() {
        old_sink.clear();
        drop(old_sink);
    }

    let sink = Sink::connect_new(&player.stream.mixer());
    match source {
        PlaySource::Url(s) => sink.append(s),
        PlaySource::File(s) => sink.append(s),
    }
    sink.play();

    state.sink = Some(sink);
    state.current_track = track_data;
    state.source_url = Some(source_url);
    state.reset_position();

    Ok(())
}

#[command]
pub fn set_volume(volume: f32, player: State<PlayerState>) -> Result<(), String> {
    let state = player.inner.lock().map_err(|e| e.to_string())?;

    if let Some(ref sink) = state.sink {
        sink.set_volume(volume);
    }

    Ok(())
}

#[command]
pub fn get_current_track(player: State<PlayerState>) -> Result<Option<CurrentTrack>, String> {
    let state = player.inner.lock().map_err(|e| e.to_string())?;
    Ok(state.current_track.clone())
}

#[command]
pub fn pause(player: State<PlayerState>) -> Result<(), String> {
    let mut state = player.inner.lock().map_err(|e| e.to_string())?;

    if let Some(ref sink) = state.sink {
        sink.pause();
        state.pause();
    }

    Ok(())
}

#[command]
pub fn resume(player: State<PlayerState>) -> Result<(), String> {
    let mut state = player.inner.lock().map_err(|e| e.to_string())?;

    if let Some(ref sink) = state.sink {
        sink.play();
        state.resume();
    }

    Ok(())
}

#[command]
pub fn stop(player: State<PlayerState>) -> Result<(), String> {
    let mut state = player.inner.lock().map_err(|e| e.to_string())?;

    if let Some(sink) = state.sink.take() {
        sink.stop();
    }

    state.clear();

    Ok(())
}

#[command]
pub async fn seek(position: f64, player: State<'_, PlayerState>) -> Result<(), String> {
    let duration = Duration::from_secs_f64(position);

    let (was_paused, needs_reload, track_data, source_url) = {
        let state = player.inner.lock().map_err(|e| e.to_string())?;
        let needs_reload = state.sink.as_ref().map_or(false, |s| s.empty());
        let was_paused = state.is_paused;
        let track_data = state.current_track.clone();
        let source_url = state.source_url.clone();
        (was_paused, needs_reload, track_data, source_url)
    };

    if needs_reload {
        let track = track_data.ok_or("No track data available to reload")?;
        let reload_path = source_url.unwrap_or_else(|| track.path.clone());
        play_file(reload_path, Some(track), player.clone()).await?;
    }

    let mut state = player.inner.lock().map_err(|e| e.to_string())?;

    if state.sink.is_none() {
        return Ok(());
    }

    let seek_result = state.sink.as_ref().unwrap().try_seek(duration);

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

#[command]
pub fn get_position(player: State<PlayerState>) -> Result<f64, String> {
    let state = player.inner.lock().map_err(|e| e.to_string())?;
    Ok(state.position())
}
