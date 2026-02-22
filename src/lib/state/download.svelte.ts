import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { tempDir } from "@tauri-apps/api/path";
import { settingsState } from "./settings.svelte";
import { monochromeService } from "$lib/services/monochrome";
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

        const dl: DownloadInfo = {
            id,
            track,
            current: 0,
            total: 0,
            status: 'resolving',
            tempPath,
            quality: settingsState.downloadQuality,
            fileType: 'loading...'
        };

        this.downloads.push(dl);

        try {
            const onlineId = parseInt(track.id.split(":")[1]);
            const response = await fetch(`${settingsState.monochromeInstance}/track/?id=${onlineId}&quality=${settingsState.downloadQuality}`);
            if (!response.ok) throw new Error("Failed to get track info");
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

                await invoke("download_track", {
                    url,
                    downloadId: id,
                    tempPath
                });
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

        const chunks: Uint8Array[] = [];

        const segmentUrls: string[] = [initUrl];

        dl.status = 'downloading';
        let receivedBytes = 0;
        let segmentIndex = startNumber;

        while (true) {
            const currentUrl = segmentUrls.length === 1
                ? initUrl
                : mediaTemplate.replace('$Number$', segmentIndex.toString());

            if (segmentUrls.length === 1) dl.fileType = 'DASH (FLAC)';

            try {
                const res = await fetch(currentUrl);
                if (!res.ok) break;

                const buffer = await res.arrayBuffer();
                const chunk = new Uint8Array(buffer);
                chunks.push(chunk);
                receivedBytes += chunk.byteLength;

                dl.current = receivedBytes;

                if (segmentUrls.length === 1) {
                    segmentUrls.push(mediaTemplate); // Mark that we found init
                } else {
                    segmentIndex++;
                }

                if (segmentIndex > startNumber + 500) break;
            } catch {
                break;
            }
        }

        const totalBytes = chunks.reduce((acc, c) => acc + c.byteLength, 0);
        const merged = new Uint8Array(totalBytes);
        let offset = 0;
        for (const chunk of chunks) {
            merged.set(chunk, offset);
            offset += chunk.byteLength;
        }

        await invoke("write_file_bytes", {
            path: dl.tempPath,
            bytes: Array.from(merged)
        });
    }

    async clearFinished(id: string) {
        this.downloads = this.downloads.filter(d => d.id !== id);
    }
}

export const downloadState = new DownloadState();
