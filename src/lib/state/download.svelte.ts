import { listen } from "@tauri-apps/api/event";
import { tempDir } from "@tauri-apps/api/path";
import { settingsState } from "./settings.svelte";
import { monochromeService } from "$lib/services/monochrome";
import { TauriService } from "$lib/utils/tauri";
import type { Track } from "$lib/types";

export interface DownloadInfo {
    id: string;
    track: Track;
    current: number;
    total: number;
    status: 'resolving' | 'downloading' | 'finished' | 'error';
    tempPath: string;
    error?: string;
    quality: string;
    fileType: string;
    coverUrl?: string;
}

class DownloadState {
    downloads = $state<DownloadInfo[]>([]);
    trackToImport = $state<DownloadInfo | null>(null);

    constructor() {
        this.setupListeners();
    }

    private async setupListeners() {
        if (typeof window === 'undefined') return;

        await listen<{ download_id: string, current: number, total: number | null, status: string }>(
            "download-progress",
            (event) => {
                const dl = this.downloads.find(d => d.id === event.payload.download_id);
                if (dl) {
                    dl.current = event.payload.current;
                    dl.total = event.payload.total || 0;
                    dl.status = 'downloading';
                }
            }
        );
    }

    async startDownload(track: Track) {
        const id = Math.random().toString(36).substring(7);
        const baseTemp = await tempDir();
        const filename = `${track.title || 'track'}_${id}.tmp`;
        const tempPath = `${baseTemp}/${filename}`;

        const initialDl: DownloadInfo = {
            id,
            track,
            current: 0,
            total: 0,
            status: 'resolving',
            tempPath,
            quality: settingsState.downloadQuality,
            fileType: 'loading...'
        };

        this.downloads.push(initialDl);
        const dl = this.downloads.find(d => d.id === id)!;

        try {
            const onlineId = parseInt(track.id.split(":")[1]);
            const response = await fetch(`${settingsState.monochromeInstance}/track/?id=${onlineId}&quality=${settingsState.downloadQuality}`);

            if (!response.ok) throw new Error(`Failed to get track info: ${response.statusText}`);
            const data = await response.json();
            const manifestBase64 = data.data?.manifest || data.manifest;

            if (!manifestBase64) throw new Error("No manifest found");

            const decoded = atob(manifestBase64);

            if (decoded.includes('<MPD')) {
                await this.downloadFromMpd(decoded, dl);
            } else {
                let url: string | null = null;
                try {
                    const parsed = JSON.parse(decoded);
                    if (parsed.urls?.[0]) url = parsed.urls[0];
                } catch {
                    const urlMatch = decoded.match(/https?:\/\/[^\s<>"]+\.(flac|mp4|m4a|aac)[^\s<>"]*/);
                    if (urlMatch) url = urlMatch[0];
                }

                if (!url) throw new Error("Could not extract stream URL");

                if (url.includes('.flac')) dl.fileType = 'FLAC';
                else if (url.includes('.m4a')) dl.fileType = 'M4A';
                else if (url.includes('.mp4')) dl.fileType = 'MP4';
                else if (url.includes('.aac')) dl.fileType = 'AAC';
                else dl.fileType = 'Audio';

                try {
                    const cover = await monochromeService.getCoverUrl(onlineId);
                    if (cover) dl.coverUrl = cover;
                } catch (e) {
                    console.warn("Failed to fetch cover URL:", e);
                }

                await TauriService.downloadTrack(
                    url,
                    id,
                    tempPath
                );
            }

            dl.status = 'finished';
            this.trackToImport = dl;
        } catch (e) {
            console.error("Download failed:", e);
            dl.status = 'error';
            dl.error = String(e);
        }
    }

    private async downloadFromMpd(manifestText: string, dl: DownloadInfo) {
        const baseUrlMatch = manifestText.match(/<BaseURL>(.*?)<\/BaseURL>/);
        const baseUrl = baseUrlMatch ? baseUrlMatch[1] : "";

        const initMatch = manifestText.match(/initialization="([^"]+)"/);
        const mediaMatch = manifestText.match(/media="([^"]+)"/);
        const startNumberMatch = manifestText.match(/startNumber="([^"]+)"/);

        if (!initMatch || !mediaMatch) throw new Error("Invalid MPD manifest");

        const initUrl = initMatch[1].startsWith('http') ? initMatch[1] : baseUrl + initMatch[1];
        const mediaTemplate = mediaMatch[1].startsWith('http') ? mediaMatch[1] : baseUrl + mediaMatch[1];
        const startNumber = parseInt(startNumberMatch ? startNumberMatch[1] : "1");

        dl.fileType = 'DASH (FLAC)';

        await TauriService.downloadMpdTrack(
            initUrl,
            mediaTemplate,
            startNumber,
            dl.id,
            dl.tempPath
        );
    }

    async clearFinished(id: string) {
        this.downloads = this.downloads.filter(d => d.id !== id);
    }

    async retryDownload(id: string) {
        const dl = this.downloads.find(d => d.id === id);
        if (dl) {
            const track = dl.track;
            this.downloads = this.downloads.filter(d => d.id !== id);
            await this.startDownload(track);
        }
    }

    async cancelDownload(id: string) {
        try {
            await TauriService.cancelDownload(id);
        } catch (e) {
            console.error("Failed to cancel download backend task:", e);
        } finally {
            this.downloads = this.downloads.filter(d => d.id !== id);
        }
    }
}

export function createDownloadState() {
    return new DownloadState();
}

export const downloadState = createDownloadState();
