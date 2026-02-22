import type {
  Track,
  ArtistWithCount,
  AlbumWithCount,
  GenreWithCount,
} from "./entity";

export interface ScanProgress {
  current: number;
  total: number;
  current_file: string;
}

export interface LibraryStats {
  total_tracks: number;
  total_duration: number;
  unique_artists: number;
  unique_albums: number;
  unique_genres: number;
}

export interface PaginatedTracks {
  tracks: Track[];
  total: number;
}

export type Selection =
  | { type: "all" }
  | { type: "artist"; artist: ArtistWithCount }
  | {
    type: "artist-album";
    artist: ArtistWithCount;
    album: AlbumWithCount;
  }
  | { type: "album"; album: AlbumWithCount }
  | { type: "genre"; genre: GenreWithCount }
  | {
    type: "genre-album";
    genre: GenreWithCount;
    album: AlbumWithCount;
  }
  | { type: "queue" };

export type SortColumn = "added_at" | "release_date" | "title" | "duration" | "track_count" | "name" | "default";
export type SortDirection = "asc" | "desc";
