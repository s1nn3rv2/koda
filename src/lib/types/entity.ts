export interface Track {
  id: string;
  path: string;
  title: string | null;
  artists: string | null;
  album: string | null;
  album_artist: string | null;
  album_id: string | null;
  album_artist_ids?: string[];
  genre: string | null;
  genre_id: string | null;
  duration: number | null;
  track_number?: number | null;
  disc_number?: number | null;
  added_at?: number;
  cover_hash?: string | null;
  release_date?: string | null;
}

export interface ArtistWithCount {
  id: string;
  name: string;
  track_count: number;
  album_count: number;
  image_hash: string | null;
  mbid: string | null;
}

export interface AlbumWithCount {
  id: string;
  name: string;
  cover_hash: string | null;
  release_date: string | null;
  track_count: number;
  artist_name: string | null;
  album_artist_ids: string[];
  album_artist_names: string[];
  mbid: string | null;
}

export interface GenreWithCount {
  id: string;
  name: string;
  track_count: number;
  album_count: number;
}
