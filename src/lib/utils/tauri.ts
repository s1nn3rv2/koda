import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type {
  Track,
  ArtistWithCount,
  AlbumWithCount,
  WaveformData,
  GenreWithCount,
  PaginatedTracks,
  LibraryStats,
  SortColumn,
  SortDirection,
} from "$lib/types";

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

  static async getAlbumDetails(
    albumId: string,
  ): Promise<AlbumWithCount | null> {
    return invoke("get_album_details", { albumId });
  }

  static async deleteTrackFile(id: string): Promise<void> {
    return invoke("delete_track_file", { id });
  }

  static async onScanProgress(
    callback: (progress: ScanProgress) => void,
  ): Promise<UnlistenFn> {
    return listen<ScanProgress>("scan-progress", (event) =>
      callback(event.payload),
    );
  }

  static async downloadTrack(
    url: string,
    downloadId: string,
    tempPath: string,
  ): Promise<string> {
    return invoke("download_track", { url, downloadId, tempPath });
  }

  static async downloadMpdTrack(
    initUrl: string,
    mediaTemplate: string,
    startNumber: number,
    downloadId: string,
    tempPath: string,
  ): Promise<string> {
    return invoke("download_mpd_track", {
      initUrl,
      mediaTemplate,
      startNumber,
      downloadId,
      tempPath,
    });
  }

  static async cancelDownload(downloadId: string): Promise<void> {
    return invoke("cancel_download", { downloadId });
  }

  static async writeFileBytes(path: string, bytes: number[]): Promise<void> {
    return invoke("write_file_bytes", { path, bytes });
  }

  static async importDownloadedTrack(
    tempPath: string,
    targetFolder: string,
    metadata: any,
  ): Promise<Track> {
    return invoke("import_downloaded_track", {
      tempPath,
      targetFolder,
      metadata,
    });
  }

  static async getSubdirectories(path: string): Promise<string[]> {
    return invoke("get_subdirectories", { path });
  }

  static async getWaveform(id: string): Promise<WaveformData> {
    return invoke("get_waveform", { id });
  }

  static async getImageByHash(
    hash: string,
    size?: number,
  ): Promise<string | null> {
    return invoke("get_image_by_hash", { hash, size });
  }

  static async getCoverArt(id: string, size?: number): Promise<string> {
    return invoke("get_cover_art", { id, size });
  }

  static async getImageFromUrl(url: string): Promise<string | null> {
    return invoke("get_image_from_url", { url });
  }

  static async searchTracks(query: string): Promise<Track[]> {
    return invoke("search_tracks", { query });
  }

  static async searchTracksPaginated(
    query: string,
    limit: number,
    offset: number,
    sortColumn: SortColumn | null = null,
    sortDir: SortDirection | null = null,
  ): Promise<PaginatedTracks> {
    return invoke("search_tracks_paginated", {
      query,
      limit,
      offset,
      sortColumn,
      sortDir,
    });
  }

  static async getAllArtists(
    query: string | null = null,
  ): Promise<ArtistWithCount[]> {
    return invoke("get_all_artists", { query });
  }

  static async getTracksByArtist(artistId: string): Promise<Track[]> {
    return invoke("get_tracks_by_artist", { artistId });
  }

  static async getTracksByArtistPage(
    artistId: string,
    limit: number,
    offset: number,
    sortColumn: SortColumn | null = null,
    sortDir: SortDirection | null = null,
  ): Promise<PaginatedTracks> {
    return invoke("get_tracks_by_artist_page", {
      artistId,
      limit,
      offset,
      sortColumn,
      sortDir,
    });
  }

  static async getAllAlbums(
    query: string | null = null,
  ): Promise<AlbumWithCount[]> {
    return invoke("get_all_albums", { query });
  }

  static async getAlbumsByArtist(
    artistId: string,
    query: string | null = null,
    sortColumn: SortColumn | null = null,
    sortDir: SortDirection | null = null,
  ): Promise<AlbumWithCount[]> {
    return invoke("get_albums_by_artist", {
      artistId,
      query,
      sortColumn,
      sortDir,
    });
  }

  static async getAlbumsByGenre(
    genreId: string,
    query: string | null = null,
    sortColumn: SortColumn | null = null,
    sortDir: SortDirection | null = null,
  ): Promise<AlbumWithCount[]> {
    return invoke("get_albums_by_genre", {
      genreId,
      query,
      sortColumn,
      sortDir,
    });
  }

  static async getTracksByAlbum(albumId: string): Promise<Track[]> {
    return invoke("get_tracks_by_album", { albumId });
  }

  static async getTracksByAlbumPage(
    albumId: string,
    limit: number,
    offset: number,
    sortColumn: SortColumn | null = null,
    sortDir: SortDirection | null = null,
  ): Promise<PaginatedTracks> {
    return invoke("get_tracks_by_album_page", {
      albumId,
      limit,
      offset,
      sortColumn,
      sortDir,
    });
  }

  static async getAllGenres(
    query: string | null = null,
  ): Promise<GenreWithCount[]> {
    return invoke("get_all_genres", { query });
  }

  static async getTracksByGenre(genreId: string): Promise<Track[]> {
    return invoke("get_tracks_by_genre", { genreId });
  }

  static async getTracksByGenrePage(
    genreId: string,
    limit: number,
    offset: number,
    sortColumn: SortColumn | null = null,
    sortDir: SortDirection | null = null,
  ): Promise<PaginatedTracks> {
    return invoke("get_tracks_by_genre_page", {
      genreId,
      limit,
      offset,
      sortColumn,
      sortDir,
    });
  }

  static async getTracksPage(
    limit: number,
    offset: number,
    sortColumn: SortColumn | null = null,
    sortDir: SortDirection | null = null,
  ): Promise<PaginatedTracks> {
    return invoke("get_tracks_page", { limit, offset, sortColumn, sortDir });
  }

  static async deleteTrack(id: string): Promise<void> {
    return invoke("delete_track", { id });
  }

  static async scanAndSaveLibrary(paths: string[]): Promise<number> {
    return invoke("scan_and_save_library", { paths });
  }

  static async getLibraryStats(): Promise<LibraryStats> {
    return invoke("get_library_stats");
  }

  static async updateTrackMetadata(id: string, metadata: any): Promise<void> {
    return invoke("update_track_metadata", { id, metadata });
  }

  static async fetchArtistMetadata(
    artistId: string,
    provider: "musicbrainz" | "itunes" | "auto" = "auto",
  ): Promise<string | null> {
    return invoke("fetch_artist_metadata", { artistId, provider });
  }

  static async fetchAlbumMetadata(
    albumId: string,
    provider: "musicbrainz" | "itunes" | "auto" = "auto",
    force: boolean = false,
  ): Promise<[string | null, string | null]> {
    return invoke("fetch_album_metadata", { albumId, provider, force });
  }
}
