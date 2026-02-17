import { invoke } from "@tauri-apps/api/core";
import type { Track } from "$lib/types";

interface CurrentTrackData {
  id: string;
  path: string;
  title: string | null;
  artists: string | null;
  album: string | null;
  duration: number | null;
}

class PlayerState {
  currentTrackPath = $state<string | null>(null);
  currentTrackId = $state<string | null>(null);
  currentTrack = $state<Track | null>(null);
  trackName = $state("No track selected");
  artistName = $state("Unknown Artist");
  isPlaying = $state(false);
  isPaused = $state(false);
  errorMsg = $state("");
  volume = $state(0.5);
  currentPosition = $state(0);
  private positionInterval: number | null = null;

  isActuallyPlaying = $derived(this.isPlaying && !this.isPaused);

  async getCurrentTrack() {
    try {
      const track: Track | null = await invoke("get_current_track");
      this.currentTrack = track;
      if (track) {
        this.trackName = track.title || "Unknown Track";
        this.artistName = track.artists || "Unknown Artist";
      }
      return track;
    } catch (e) {
      this.errorMsg = String(e);
      return null;
    }
  }

  async setVolume() {
    try {
      await invoke("set_volume", { volume: this.volume });
      this.errorMsg = "";
    } catch (e) {
      this.errorMsg = String(e);
    }
  }

  async togglePlayPause() {
    try {
      if (!this.isPlaying) {
        if (!this.currentTrackPath) return;
        await invoke("play_file", {
          path: this.currentTrackPath,
          trackData: this.currentTrack,
        });
        this.isPlaying = true;
        this.isPaused = false;
        this.startPositionTracking();
      } else if (this.isPaused) {
        await invoke("resume");
        this.isPaused = false;
        this.startPositionTracking();
      } else {
        await invoke("pause");
        this.isPaused = true;
        this.stopPositionTracking();
      }
      this.errorMsg = "";
    } catch (e) {
      this.errorMsg = String(e);
    }
  }

  async seek(position: number) {
    try {
      await invoke("seek", { position });

      if (this.isPaused) {
        this.currentPosition = position;
      } else {
        this.isPlaying = true;
        this.isPaused = false;
        this.startPositionTracking();
      }
      this.errorMsg = "";
    } catch (e) {
      this.errorMsg = String(e);
    }
  }

  async updatePosition() {
    try {
      const position: number = await invoke("get_position");
      this.currentPosition = position;

      if (
        this.currentTrack?.duration &&
        position >= this.currentTrack.duration
      ) {
        this.isPlaying = false;
        this.isPaused = false;
        this.stopPositionTracking();
      }
    } catch (e) {
      console.error("Failed to update position:", e);
    }
  }

  startPositionTracking() {
    this.stopPositionTracking();
    this.positionInterval = window.setInterval(async () => {
      if (this.isActuallyPlaying) {
        await this.updatePosition();
      }
    }, 100);
  }

  stopPositionTracking() {
    if (this.positionInterval !== null) {
      clearInterval(this.positionInterval);
      this.positionInterval = null;
    }
  }

  async playTrack(track: Track) {
    this.currentTrackPath = track.path;
    this.currentTrackId = track.id;
    this.currentPosition = 0;
    this.isPlaying = true;
    this.isPaused = false;

    this.currentTrack = track;
    this.trackName = track.title || "Unknown Track";
    this.artistName = track.artists || "Unknown Artist";

    const trackData: CurrentTrackData = {
      id: track.id,
      path: track.path,
      title: track.title,
      artists: track.artists,
      album: track.album,
      duration: track.duration,
    };

    try {
      await invoke("play_file", {
        path: track.path,
        trackData,
      });
      this.startPositionTracking();
      this.errorMsg = "";
    } catch (e) {
      this.errorMsg = String(e);
      this.isPlaying = false;
    }
  }

  async nextTrack() {
    // TODO: implement queue
  }

  async prevTrack() {
    // TODO: implement queue
  }
}

class UIState {
  isExpanded = $state(false);

  toggleExpanded() {
    this.isExpanded = !this.isExpanded;
  }
}

export const playerState = new PlayerState();
export const uiState = new UIState();
