use std::{fs::File, sync::Mutex};

use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
use tauri::State;

pub struct PlayerState {
    pub stream: Mutex<Option<OutputStream>>,
    pub sink: Mutex<Option<Sink>>,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            stream: Mutex::new(None),
            sink: Mutex::new(None),
        }
    }
}

#[tauri::command]
pub fn play_file(path: String, player: State<PlayerState>) -> Result<(), String> {
    let stream = OutputStreamBuilder::open_default_stream().map_err(|e| e.to_string())?;
    let sink = Sink::connect_new(stream.mixer());

    let file = File::open(&path).map_err(|e| format!("failed to open file: {e}"))?;
    let source = Decoder::try_from(file).map_err(|e| format!("decode failed: {e}"))?;

    sink.append(source);
    sink.play();

    *player.stream.lock().map_err(|e| e.to_string())? = Some(stream);
    *player.sink.lock().map_err(|e| e.to_string())? = Some(sink);

    Ok(())
}

#[tauri::command]
pub fn pause(player: State<PlayerState>) -> Result<(), String> {
    if let Some(sink) = player.sink.lock().map_err(|e| e.to_string())?.as_ref() {
        sink.pause();
    }
    Ok(())
}

#[tauri::command]
pub fn resume(player: State<PlayerState>) -> Result<(), String> {
    if let Some(sink) = player.sink.lock().map_err(|e| e.to_string())?.as_ref() {
        sink.play();
    }
    Ok(())
}

#[tauri::command]
pub fn stop(player: State<PlayerState>) -> Result<(), String> {
    if let Some(sink) = player.sink.lock().map_err(|e| e.to_string())?.take() {
        sink.stop();
    }
    Ok(())
}
