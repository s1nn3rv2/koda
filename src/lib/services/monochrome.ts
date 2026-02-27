import { settingsState } from "../state/settings.svelte";
import type { Track } from "../types/entity";

export interface MonochromeTrack {
  id: number;
  title: string;
  duration: number;
  streamStartDate?: string;
  release_date?: string;
  trackNumber?: number;
  volumeNumber?: number;
  artist?: {
    id: number;
    name: string;
  };
  artists?: Array<{
    id: number;
    name: string;
  }>;
  album: {
    id: number;
    title: string;
    cover: string;
    release_date?: string;
    release_year?: number;
  };
}

export interface MonochromeSearchResponse {
  items: MonochromeTrack[];
  totalNumberOfItems: number;
}

export interface CoverImage {
  80?: string;
  160?: string;
  320?: string;
  640?: string;
  1080?: string;
  1280?: string;
}

class MonochromeService {
  private get baseUrl() {
    return settingsState.monochromeInstance;
  }

  async searchTracks(
    query: string,
    limit: number = 25,
    offset: number = 0,
  ): Promise<MonochromeSearchResponse> {
    const response = await fetch(
      `${this.baseUrl}/search/?s=${encodeURIComponent(query)}&limit=${limit}&offset=${offset}`,
    );
    if (!response.ok) {
      throw new Error("Search failed");
    }
    const data = await response.json();

    let tracks;
    if (data.data && Array.isArray(data.data.items)) {
      tracks = {
        items: data.data.items,
        totalNumberOfItems:
          data.data.totalNumberOfItems || data.data.items.length,
      };
    } else if (Array.isArray(data.items)) {
      tracks = {
        items: data.items,
        totalNumberOfItems: data.totalNumberOfItems || data.items.length,
      };
    } else {
      tracks = { items: [], totalNumberOfItems: 0 };
    }

    return tracks;
  }

  async getStreamUrl(trackId: number): Promise<string> {
    const response = await fetch(
      `${this.baseUrl}/track/?id=${trackId}&quality=${settingsState.audioQuality}`,
    );
    if (!response.ok) throw new Error("Failed to get track info");
    const data = await response.json();

    const manifest = data.data?.manifest || data.manifest;
    if (manifest) {
      return this.extractUrlFromManifest(manifest);
    }

    throw new Error("No stream URL found");
  }

  private extractUrlFromManifest(manifestBase64: string): string {
    try {
      const decoded = atob(manifestBase64);
      try {
        const parsed = JSON.parse(decoded);
        if (parsed.urls?.[0]) return parsed.urls[0];
      } catch {
        const urlMatch = decoded.match(
          /https?:\/\/[^\s<>"]+\.(flac|mp4|m4a|aac)[^\s<>"]*/,
        );
        if (urlMatch) return urlMatch[0];
      }
    } catch (e) {
      console.error("Manifest decoding failed", e);
    }
    throw new Error("Could not extract stream URL from manifest");
  }

  async getCoverUrl(
    trackId: number,
    preferredSize: number = 1280,
  ): Promise<string | null> {
    try {
      const response = await fetch(`${this.baseUrl}/cover/?id=${trackId}`);
      if (!response.ok) return null;
      const data = await response.json();

      if (data && data.covers && data.covers.length > 0) {
        const covers = data.covers[0];
        if (covers[preferredSize]) return covers[preferredSize];

        const sizes = [1280, 1080, 640, 320, 160, 80] as const;
        for (const size of sizes) {
          if (covers[size]) return covers[size];
        }
      }
    } catch (e) {
      console.error("Failed to get cover from Monochrome", e);
    }
    return null;
  }

  mapToTrack(ot: MonochromeTrack): Track {
    const artistName = ot.artists
      ? ot.artists.map((a) => a.name).join("; ")
      : ot.artist?.name || "Unknown Artist";

    let releaseDate =
      ot.streamStartDate ||
      ot.release_date ||
      ot.album.release_date ||
      (ot.album.release_year ? ot.album.release_year.toString() : null);

    if (releaseDate && releaseDate.includes("T")) {
      releaseDate = releaseDate.split("T")[0];
    }

    const albumTitle = ot.album.title || null;

    return {
      id: `online:${ot.id}`,
      title: ot.title,
      artists: artistName,
      album: albumTitle,
      duration: ot.duration,
      cover_hash: `online-cover:${ot.id}`,
      path: `online:${ot.id}`,
      added_at: Math.floor(Date.now() / 1000),
      album_id: `online_album:${ot.album.id}`,
      genre_id: null,
      genre: "Online",
      track_number: ot.trackNumber || null,
      disc_number: ot.volumeNumber || null,
      release_date: releaseDate,
      album_artist: artistName,
    };
  }
}

export const monochromeService = new MonochromeService();
