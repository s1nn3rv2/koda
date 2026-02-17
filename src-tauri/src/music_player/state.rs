use std::{sync::Mutex, time::Instant};

use rodio::{OutputStream, Sink};
use serde::{Deserialize, Serialize};

use crate::database::Track;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentTrack {
    pub id: String,
    pub path: String,
    pub title: Option<String>,
    pub artists: Option<String>,
    pub album: Option<String>,
    pub duration: Option<i64>,
}

impl From<Track> for CurrentTrack {
    fn from(track: Track) -> Self {
        Self {
            id: track.id,
            path: track.path,
            title: track.title,
            artists: track.artists,
            album: track.album,
            duration: track.duration,
        }
    }
}

pub(super) struct PlayerInnerState {
    pub sink: Option<Sink>,
    pub current_track: Option<CurrentTrack>,
    pub playback_start: Option<Instant>,
    pub seek_offset: f64,
    pub is_paused: bool,
    pub paused_position: f64,
}

impl Default for PlayerInnerState {
    fn default() -> Self {
        Self {
            sink: None,
            current_track: None,
            playback_start: None,
            seek_offset: 0.0,
            is_paused: false,
            paused_position: 0.0,
        }
    }
}

impl PlayerInnerState {
    pub fn position(&self) -> f64 {
        if self.is_paused {
            return self.paused_position;
        }

        if let Some(start) = self.playback_start {
            let elapsed = start.elapsed().as_secs_f64();
            self.seek_offset + elapsed
        } else {
            0.0
        }
    }

    pub fn reset_position(&mut self) {
        self.playback_start = Some(Instant::now());
        self.seek_offset = 0.0;
        self.is_paused = false;
        self.paused_position = 0.0;
    }

    pub fn pause(&mut self) {
        self.paused_position = self.position();
        self.is_paused = true;
    }

    pub fn resume(&mut self) {
        self.playback_start = Some(Instant::now());
        self.seek_offset = self.paused_position;
        self.is_paused = false;
    }

    pub fn seek(&mut self, position: f64) {
        self.playback_start = Some(Instant::now());
        self.seek_offset = position;

        if self.is_paused {
            self.paused_position = position;
        }
    }

    pub fn clear(&mut self) {
        self.sink = None;
        self.current_track = None;
        self.playback_start = None;
        self.seek_offset = 0.0;
        self.is_paused = false;
        self.paused_position = 0.0;
    }
}

// audio stream is created once at startup and lives for the entire app lifetime
pub struct PlayerState {
    pub(super) stream: OutputStream,
    pub(super) inner: Mutex<PlayerInnerState>,
}

impl PlayerState {
    pub fn new() -> Result<Self, String> {
        let stream = rodio::OutputStreamBuilder::open_default_stream()
            .map_err(|e| format!("Failed to initialize audio output: {}", e))?;

        Ok(Self {
            stream,
            inner: Mutex::new(PlayerInnerState::default()),
        })
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::new().expect("Failed to initialize audio player")
    }
}
