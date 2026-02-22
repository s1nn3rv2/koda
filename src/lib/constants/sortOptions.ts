import type { SortColumn, SortDirection } from "$lib/types";

export interface SortOption {
    label: string;
    column: SortColumn;
    defaultDir: SortDirection;
}

export const TRACK_SORT_OPTIONS: SortOption[] = [
    { label: "Default", column: "default", defaultDir: "desc" },
    { label: "Title", column: "title", defaultDir: "asc" },
    { label: "Date Added", column: "added_at", defaultDir: "desc" },
    { label: "Release Date", column: "release_date", defaultDir: "desc" },
    { label: "Duration", column: "duration", defaultDir: "desc" },
];

export const ALBUM_SORT_OPTIONS: SortOption[] = [
    { label: "Release Date", column: "release_date", defaultDir: "desc" },
    { label: "Title", column: "title", defaultDir: "asc" },
    { label: "Date Added", column: "added_at", defaultDir: "desc" },
];
