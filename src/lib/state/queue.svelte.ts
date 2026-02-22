import { playbackState } from "./playback.svelte";
import { TauriService } from "$lib/utils/tauri";
import type { Track } from "$lib/types";

export class QueueState {
    manualQueue = $state<Track[]>(JSON.parse(localStorage.getItem("queue_manual") || "[]"));
    playbackContext = $state<Track[]>(JSON.parse(localStorage.getItem("queue_context") || "[]"));
    contextIndex = $state(Number(localStorage.getItem("queue_context_index")) || 0);
    isPlayingManual = $state(localStorage.getItem("queue_is_playing_manual") === "true");
    shuffleMode = $state<"smart" | "category" | "random">((localStorage.getItem("shuffleMode") as any) || "smart");

    queue = $derived(this.manualQueue);
    queueIndex = $derived(0);

    constructor() {
        $effect.root(() => {
            $effect(() => {
                localStorage.setItem("queue_manual", JSON.stringify(this.manualQueue));
                localStorage.setItem("queue_context", JSON.stringify(this.playbackContext));
                localStorage.setItem("queue_context_index", this.contextIndex.toString());
                localStorage.setItem("queue_is_playing_manual", this.isPlayingManual.toString());
                localStorage.setItem("shuffleMode", this.shuffleMode);
            });
        });

        $effect.root(() => {
            $effect(() => {
                if (
                    playbackState.currentTrack?.duration &&
                    playbackState.currentPosition >= playbackState.currentTrack.duration &&
                    playbackState.isActuallyPlaying
                ) {
                    this.nextTrack();
                }
            });
        });
    }

    async playTrack(track: Track, context: Track[] | null = null, fromManual = false) {
        if (context) {
            this.playbackContext = [...context];
            this.contextIndex = this.playbackContext.findIndex(t => t.id === track.id);
            this.isPlayingManual = false;
        } else if (fromManual) {
            this.isPlayingManual = true;
        } else {
            const indexInContext = this.playbackContext.findIndex(t => t.id === track.id);
            if (indexInContext !== -1) {
                this.contextIndex = indexInContext;
            } else {
                this.playbackContext = [track];
                this.contextIndex = 0;
            }
            this.isPlayingManual = false;
        }

        await playbackState.play(track);
    }

    async nextTrack() {
        if (this.isPlayingManual) {
            const newManualQueue = [...this.manualQueue];
            newManualQueue.shift();
            this.manualQueue = newManualQueue;
        }

        if (this.manualQueue.length > 0) {
            await this.playTrack(this.manualQueue[0], null, true);
            return;
        }

        if (this.shuffleMode === "random") {
            try {
                const track = await TauriService.getRandomTrack();
                if (track) {
                    await this.playTrack(track, null, false);
                    return;
                }
            } catch (e) {
                console.error("Failed to fetch random track:", e);
            }
        }

        if (this.playbackContext.length > 0) {
            if (this.shuffleMode === "category") {
                let nextIndex = Math.floor(Math.random() * this.playbackContext.length);
                if (this.playbackContext.length > 1 && nextIndex === this.contextIndex) {
                    nextIndex = (nextIndex + 1) % this.playbackContext.length;
                }
                this.contextIndex = nextIndex;
                await this.playTrack(this.playbackContext[this.contextIndex], null, false);
                return;
            }

            this.contextIndex++;
            if (this.contextIndex >= 0 && this.contextIndex < this.playbackContext.length) {
                await this.playTrack(this.playbackContext[this.contextIndex], null, false);
                return;
            }
        }

        if (this.shuffleMode !== "random") {
            try {
                const track = await TauriService.getRandomTrack();
                if (track) {
                    await this.playTrack(track, null, false);
                    return;
                }
            } catch (e) {
                console.error("Failed to fetch random track:", e);
            }
        }

        playbackState.stop();
    }

    toggleShuffleMode() {
        if (this.shuffleMode === "smart") this.shuffleMode = "category";
        else if (this.shuffleMode === "category") this.shuffleMode = "random";
        else this.shuffleMode = "smart";
        localStorage.setItem("shuffleMode", this.shuffleMode);
    }

    async prevTrack() {
        if (playbackState.currentPosition > 3) {
            await playbackState.seek(0);
            return;
        }

        if (!this.isPlayingManual && this.contextIndex > 0) {
            this.contextIndex--;
            await this.playTrack(this.playbackContext[this.contextIndex], null, false);
        } else {
            await playbackState.seek(0);
        }
    }

    addToQueue(track: Track) {
        this.manualQueue = [...this.manualQueue, track];
    }

    removeFromQueue(index: number) {
        if (index < 0 || index >= this.manualQueue.length) return;

        if (this.isPlayingManual && index === 0) {
            this.nextTrack();
        } else {
            const newManualQueue = [...this.manualQueue];
            newManualQueue.splice(index, 1);
            this.manualQueue = newManualQueue;
        }
    }

    reorderQueue(fromIndex: number, toIndex: number) {
        if (fromIndex === toIndex || fromIndex < 0 || toIndex < 0 || fromIndex >= this.manualQueue.length || toIndex >= this.manualQueue.length) return;

        const newManualQueue = [...this.manualQueue];
        const [movedTrack] = newManualQueue.splice(fromIndex, 1);
        newManualQueue.splice(toIndex, 0, movedTrack);
        this.manualQueue = newManualQueue;
    }

    async playQueueIndex(index: number) {
        if (this.isPlayingManual) {
            if (index < 0 || index >= this.manualQueue.length) return;
            const track = this.manualQueue[index];
            this.manualQueue = this.manualQueue.slice(index);
            await this.playTrack(track, null, true);
        } else {
            if (index < 0 || index >= this.playbackContext.length) return;
            this.contextIndex = index;
            await this.playTrack(this.playbackContext[index], null, false);
        }
    }

    clearQueue() {
        this.manualQueue = [];
    }
}

export const queueState = new QueueState();
