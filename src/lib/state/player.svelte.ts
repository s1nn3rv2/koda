import { invoke } from "@tauri-apps/api/core";

class PlayerState {
    currentTrackPath = $state<string | null>(null);
    trackName = $state("No track selected");
    artistName = $state("Unknown Artist");
    isPlaying = $state(false);
    isPaused = $state(false);
    errorMsg = $state("");

    isActuallyPlaying = $derived(this.isPlaying && !this.isPaused);

    async togglePlayPause() {
        try {
            if (!this.isPlaying) {
                if (!this.currentTrackPath) return;
                await invoke("play_file", {
                    path: this.currentTrackPath,
                });
                this.isPlaying = true;
                this.isPaused = false;
            } else if (this.isPaused) {
                await invoke("resume");
                this.isPaused = false;
            } else {
                await invoke("pause");
                this.isPaused = true;
            }
            this.errorMsg = "";
        } catch (e) {
            this.errorMsg = String(e);
        }
    }

    async nextTrack() {
        // Implementation for next track
    }

    async prevTrack() {
        // Implementation for prev track
    }
}

export const playerState = new PlayerState();
