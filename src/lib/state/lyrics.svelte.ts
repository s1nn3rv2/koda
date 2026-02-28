import { TauriService } from "$lib/utils/tauri";
import { playbackState } from "./playback.svelte";
import type { Track } from "$lib/types";

export interface LyricLine {
    time: number; // in seconds
    text: string;
}

class LyricsState {
    isLoading = $state(false);
    syncedLyrics = $state<LyricLine[] | null>(null);
    error = $state<string | null>(null);

    private lastTrackId: string | null = null;
    private abortController: AbortController | null = null;

    constructor() {
        $effect.root(() => {
            $effect(() => {
                const track = playbackState.currentTrack;
                if (track && track.id !== this.lastTrackId) {
                    this.lastTrackId = track.id;
                    this.loadLyrics(track);
                } else if (!track) {
                    this.lastTrackId = null;
                    this.clear();
                }
            })
        });
    }

    clear() {
        this.syncedLyrics = null;
        this.error = null;
        this.isLoading = false;
    }

    parseSyncedLyrics(lrc: string): LyricLine[] {
        const lines = lrc.split("\n");
        const parsed: LyricLine[] = [];
        const timeRegex = /\[(\d+):(\d+\.\d+)\](.*)/;

        for (const line of lines) {
            const match = line.match(timeRegex);
            if (match) {
                const minutes = parseInt(match[1], 10);
                const seconds = parseFloat(match[2]);
                const text = match[3].trim();
                parsed.push({
                    time: minutes * 60 + seconds,
                    text,
                });
            }
        }
        return parsed.sort((a, b) => a.time - b.time);
    }

    async loadLyrics(track: Track) {
        if (this.abortController) {
            this.abortController.abort();
        }
        this.abortController = new AbortController();
        const signal = this.abortController.signal;

        this.clear();
        this.isLoading = true;

        try {
            if (track.path && !track.id.startsWith("online:")) {
                const embedded = await TauriService.getEmbeddedLyrics(track.path);
                if (embedded && embedded.includes("[00:")) {
                    this.processLyricsData(embedded, signal);
                    return;
                }
            }

            if (!track.title || !track.artists) {
                this.error = "Missing metadata for lyrics search";
                this.isLoading = false;
                return;
            }

            const data = await TauriService.fetchLyrics(
                track.title,
                track.artists,
                track.album || null,
                track.duration || null
            );

            if (signal.aborted) return;

            if (!data || !data.syncedLyrics) {
                this.error = "No synced lyrics found";
                this.isLoading = false;
                return;
            }

            const lyricsStr = data.syncedLyrics;
            this.processLyricsData(lyricsStr, signal);

            if (track.path && !track.id.startsWith("online:")) {
                try {
                    await TauriService.embedLyrics(track.path, data.syncedLyrics);
                } catch (embedError) {
                    console.error("Failed to embed lyrics:", embedError);
                }
            }
        } catch (e: any) {
            if (e.name === "AbortError") return;
            this.error = String(e);
            this.isLoading = false;
        }
    }

    private processLyricsData(lyricsStr: string, signal: AbortSignal) {
        if (signal.aborted) return;

        const parsed = this.parseSyncedLyrics(lyricsStr);
        if (parsed.length > 0) {
            this.syncedLyrics = parsed;
        } else {
            this.error = "No synced lyrics available";
        }

        this.isLoading = false;
    }
}

export const lyricsState = new LyricsState();
