export interface Track {
  id: string;
  path: string;
  title: string | null;
  artists: string | null;
  album: string | null;
  album_id: string | null;
  genre: string | null;
  genre_id: string | null;
  duration: number | null;
  track_number?: number | null;
  added_at?: number;
  cover_hash?: string | null;
}

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

export interface WaveformData {
  samples: number[];
  duration: number;
}

export interface ArtistWithCount {
  id: string;
  name: string;
  track_count: number;
}

export interface AlbumWithCount {
  id: string;
  name: string;
  cover_hash: string | null;
  track_count: number;
  artist_name: string | null;
}

export interface GenreWithCount {
  id: string;
  name: string;
  track_count: number;
}
