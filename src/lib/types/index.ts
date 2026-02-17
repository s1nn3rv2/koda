export interface Track {
  id: string;
  path: string;
  title: string | null;
  artists: string | null;
  album: string | null;
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
}

export interface WaveformData {
  samples: number[];
  duration: number;
}
