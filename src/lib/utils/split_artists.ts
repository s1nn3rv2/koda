export function splitArtists(artists: string): string[] {
  return artists.split("; ").filter((a) => a.trim().length > 0);
}
