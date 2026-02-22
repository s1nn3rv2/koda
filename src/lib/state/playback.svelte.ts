import { TauriService } from "$lib/utils/tauri";
import type { Track } from "$lib/types";
import { monochromeService } from "$lib/services/monochrome";

export class PlaybackState {
    currentTrack = $state<Track | null>(JSON.parse(localStorage.getItem("player_current_track") || "null"));
    currentTrackId = $derived(this.currentTrack?.id || null);
    currentTrackPath = $derived(this.currentTrack?.path || null);

    isPlaying = $state(false);
    isPaused = $state(false);
    isActuallyPlaying = $derived(this.isPlaying && !this.isPaused);

    volume = $state(Number(localStorage.getItem("player_volume")) || 0.5);
    isMuted = $state(localStorage.getItem("player_muted") === "true");
    lastVolume = $state(Number(localStorage.getItem("player_last_volume")) || 0.5);

    currentPosition = $state(Number(localStorage.getItem("player_current_position")) || 0);
    dominantColor = $state<string | null>(localStorage.getItem("player_dominant_color"));
    errorMsg = $state("");

    private positionInterval: number | null = null;

    constructor() {
        this.syncVolume();

        $effect.root(() => {
            $effect(() => {
                localStorage.setItem("player_current_track", JSON.stringify(this.currentTrack));
                localStorage.setItem("player_dominant_color", this.dominantColor || "");
                localStorage.setItem("player_current_position", this.currentPosition.toString());
            });
        });
    }

    async syncVolume() {
        const targetVolume = this.isMuted ? 0 : this.volume;
        await TauriService.setVolume(targetVolume);

        localStorage.setItem("player_volume", this.volume.toString());
        localStorage.setItem("player_muted", this.isMuted.toString());
        localStorage.setItem("player_last_volume", this.lastVolume.toString());
    }

    async toggleMute() {
        if (this.isMuted) {
            this.isMuted = false;
            this.volume = this.lastVolume;
        } else {
            this.lastVolume = this.volume > 0 ? this.volume : 0.5;
            this.isMuted = true;
            this.volume = 0;
        }
        await this.syncVolume();
    }

    async togglePlayPause() {
        try {
            if (!this.isPlaying) {
                if (!this.currentTrackPath) {
                    const { queueState } = await import("./queue.svelte");
                    await queueState.nextTrack();
                    return;
                }

                let path = this.currentTrackPath;
                if (this.currentTrackId?.startsWith("online:")) {
                    const onlineId = parseInt(this.currentTrackId.split(":")[1]);
                    path = await monochromeService.getStreamUrl(onlineId);
                }

                await TauriService.playFile(path, this.currentTrack);

                if (this.currentPosition > 0) {
                    await TauriService.seek(this.currentPosition);
                }

                this.isPlaying = true;
                this.isPaused = false;
                this.startPositionTracking();
            } else if (this.isPaused) {
                await TauriService.resume();
                this.isPaused = false;
                this.startPositionTracking();
            } else {
                await TauriService.pause();
                this.isPaused = true;
                this.stopPositionTracking();
            }
        } catch (e) {
            this.errorMsg = String(e);
        }
    }

    async play(track: Track) {
        this.stopPositionTracking();
        this.currentTrack = track;
        this.currentPosition = 0;
        this.isPlaying = true;
        this.isPaused = false;

        try {
            let path = track.path;
            if (track.id.startsWith("online:")) {
                await TauriService.stop();
                const onlineId = parseInt(track.id.split(":")[1]);
                path = await monochromeService.getStreamUrl(onlineId);
            }
            await TauriService.playFile(path, track);
            this.startPositionTracking();
            this.errorMsg = "";
        } catch (e) {
            this.errorMsg = String(e);
            this.isPlaying = false;
        }
    }

    async seek(position: number) {
        try {
            await TauriService.seek(position);
            if (this.isPaused) {
                this.currentPosition = position;
            } else {
                this.isPlaying = true;
                this.isPaused = false;
                this.startPositionTracking();
            }
        } catch (e) {
            this.errorMsg = String(e);
        }
    }

    async updatePosition() {
        try {
            const position = await TauriService.getPosition();
            this.currentPosition = position;
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

    stop() {
        this.isPlaying = false;
        this.isPaused = false;
        this.currentPosition = 0;
        this.stopPositionTracking();
        TauriService.stop().catch(console.error);
    }
}

export const playbackState = new PlaybackState();
