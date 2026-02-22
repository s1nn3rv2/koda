export function formatDuration(seconds: number | null | undefined): string {
  if (seconds == null) return "0:00";
  const mins = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${mins}:${secs.toString().padStart(2, "0")}`;
}

export const formatTime = formatDuration;

export function getFileName(path: string): string {
  return path.split(/[/\\]/).pop() || path;
}

export function pluralize(
  count: number,
  singular: string,
  plural?: string,
): string {
  if (count === 1) return singular;
  return plural || `${singular}s`;
}

export function formatDate(dateStr: string | null | undefined): string | null {
  if (!dateStr) return null;

  if (dateStr.includes("-")) {
    const parts = dateStr.split("-");
    if (parts.length === 3) {
      const [year, month, day] = parts;
      return `${day}-${month}-${year}`;
    }
    return dateStr;
  }

  return dateStr;
}
