import { TauriService, type ScanProgress } from "$lib/utils/tauri";
import type {
  ArtistWithCount,
  AlbumWithCount,
  Selection,
  SortColumn,
  SortDirection,
} from "$lib/types";
import type { Track } from "$lib/types/entity";
import {
  monochromeService,
  type MonochromeTrack,
} from "$lib/services/monochrome";

class LibraryState {
  selection = $state<Selection>(
    JSON.parse(localStorage.getItem("library_selection") || '{"type":"all"}'),
  );

  scrollPositions = $state<Record<string, number>>(
    JSON.parse(localStorage.getItem("library_scroll_positions") || "{}"),
  );
  sortModes = $state<
    Record<string, { column: SortColumn; direction: SortDirection }>
  >(JSON.parse(localStorage.getItem("library_sort_modes") || "{}"));
  searchQuery = $state(localStorage.getItem("search_query") || "");
  refreshTrigger = $state(0);
  relatedArtistIds = $state(new Set<string>());

  allTracksSortColumn = $state<SortColumn>(
    (localStorage.getItem("allTracksSortColumn") as SortColumn) ||
    "release_date",
  );
  allTracksSortDirection = $state<SortDirection>(
    (localStorage.getItem("allTracksSortDirection") as SortDirection) || "desc",
  );

  scanProgress = $state<ScanProgress | null>(null);
  libraryMode = $state<"local" | "online">("local");
  onlineSearchQuery = $state("");
  onlineSearchResults = $state<MonochromeTrack[]>([]);
  onlineSearchTotal = $state(0);
  isOnlineSearching = $state(false);

  trackToEdit = $state<Track | null>(null);

  constructor() {
    this.setupListeners();

    if (typeof window !== "undefined") {
      history.replaceState({ selection: $state.snapshot(this.selection) }, "");

      window.addEventListener("popstate", (e) => {
        if (e.state?.selection) {
          this.selection = e.state.selection;
        }
      });
    }

    $effect.root(() => {
      $effect(() => {
        localStorage.setItem(
          "library_selection",
          JSON.stringify(this.selection),
        );
        localStorage.setItem(
          "library_scroll_positions",
          JSON.stringify(this.scrollPositions),
        );
        localStorage.setItem(
          "library_sort_modes",
          JSON.stringify(this.sortModes),
        );
        localStorage.setItem("search_query", this.searchQuery);
      });
    });
  }

  async setupListeners() {
    await TauriService.onScanProgress((progress) => {
      this.scanProgress = progress;
      if (progress.current === progress.total) {
        setTimeout(() => {
          this.scanProgress = null;
        }, 3000);
        this.refresh();
      }
    });
  }

  async selectArtist(artistName: string) {
    try {
      const artist = await TauriService.getArtistByName(artistName);
      if (artist) {
        this.setSelection({ type: "artist", artist });
      }
    } catch (e) {
      console.error("Failed to select artist:", e);
    }
  }

  async selectAlbum(albumId: string) {
    try {
      const album = await TauriService.getAlbumDetails(albumId);
      if (album) {
        this.setSelection({ type: "album", album });
      }
    } catch (e) {
      console.error("Failed to select album:", e);
    }
  }

  setSelection(sel: Selection) {
    if (typeof window !== "undefined") {
      const current = this.selection;
      let isSame = false;
      if (current?.type === sel.type) {
        if (sel.type === "all" || sel.type === "queue") {
          isSame = true;
        } else if (sel.type === "artist" && current.type === "artist") {
          isSame = sel.artist.id === current.artist.id;
        } else if (sel.type === "album" && current.type === "album") {
          isSame = sel.album.id === current.album.id;
        }
      }

      if (!isSame) {
        history.pushState({ selection: sel }, "");
      }
    }

    this.selection = sel;
  }

  refresh() {
    this.refreshTrigger++;
  }

  editTrack(track: Track) {
    this.trackToEdit = track;
  }

  async searchOnline(query: string, limit: number = 25, offset: number = 0) {
    if (!query) {
      this.onlineSearchResults = [];
      this.onlineSearchTotal = 0;
      return;
    }

    this.onlineSearchQuery = query;
    if (offset === 0) {
      this.isOnlineSearching = true;
    }

    try {
      const results = await monochromeService.searchTracks(
        query,
        limit,
        offset,
      );

      if (offset === 0) {
        this.onlineSearchResults = results.items || [];
      } else {
        this.onlineSearchResults = [
          ...this.onlineSearchResults,
          ...(results.items || []),
        ];
      }

      this.onlineSearchTotal = results.totalNumberOfItems || 0;
    } catch (e) {
      console.error("Online search failed", e);
    } finally {
      this.isOnlineSearching = false;
    }
  }

  setLibraryMode(mode: "local" | "online") {
    this.libraryMode = mode;
  }
}

export const libraryState = new LibraryState();
