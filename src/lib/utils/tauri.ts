import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { Track, ArtistWithCount, AlbumWithCount, WaveformData } from "$lib/types";

export interface ScanProgress {
    current: number;
    total: number;
    current_file: string;
}

export class TauriService {
    static async playFile(path: string, trackData: any): Promise<void> {
        return invoke("play_file", { path, trackData });
    }

    static async pause(): Promise<void> {
        return invoke("pause");
    }

    static async resume(): Promise<void> {
        return invoke("resume");
    }

    static async stop(): Promise<void> {
        return invoke("stop");
    }

    static async seek(position: number): Promise<void> {
        return invoke("seek", { position });
    }

    static async setVolume(volume: number): Promise<void> {
        return invoke("set_volume", { volume });
    }

    static async getPosition(): Promise<number> {
        return invoke("get_position");
    }

    static async getCurrentTrack(): Promise<Track | null> {
        return invoke("get_current_track");
    }

    static async getRandomTrack(): Promise<Track | null> {
        return invoke("get_random_track");
    }

    static async getArtistByName(name: string): Promise<ArtistWithCount | null> {
        return invoke("get_artist_by_name", { name });
    }

    static async getAlbumDetails(albumId: string): Promise<AlbumWithCount | null> {
        return invoke("get_album_details", { albumId });
    }

    static async deleteTrackFile(id: string): Promise<void> {
        return invoke("delete_track_file", { id });
    }

    static async onScanProgress(callback: (progress: ScanProgress) => void): Promise<UnlistenFn> {
        return listen<ScanProgress>("scan-progress", (event) => callback(event.payload));
    }
}
