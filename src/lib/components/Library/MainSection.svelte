<script lang="ts">
    import { untrack } from "svelte";
    import { uiState } from "$lib/state/player.svelte";
    import type {
        Track,
        PaginatedTracks,
        Selection,
        SortColumn,
        SortDirection,
    } from "$lib/types";
    import { libraryState } from "$lib/state/library.svelte";
    import { TauriService } from "$lib/utils/tauri";
    import { TRACK_SORT_OPTIONS } from "$lib/constants/sortOptions";
    import AllTracksPage from "./Pages/AllTracksPage.svelte";
    import AlbumPage from "./Pages/AlbumPage.svelte";
    import ArtistPage from "./Pages/ArtistPage.svelte";
    import GenrePage from "./Pages/GenrePage.svelte";
    import QueuePage from "./Pages/QueuePage.svelte";
    import SearchResultsView from "./SearchResultsView.svelte";
    import OnlinePlaceholder from "./OnlinePlaceholder.svelte";

    const PAGE_SIZE = 50;

    let sortColumn = $state<SortColumn>(
        libraryState.selection.type === "all"
            ? libraryState.allTracksSortColumn
            : "default",
    );
    let sortDirection = $state<SortDirection>("desc");
    let contentTracks = $state<Track[]>([]);
    let totalTracks = $state(0);
    let currentOffset = $state(0);
    let isLoadingMore = $state(false);
    let searchResults = $state<Track[]>([]);
    let searchTotal = $state(0);
    let searchOffset = $state(0);
    let isSearchingMore = $state(false);
    let isSearching = $state(false);
    let isOnlineSearchingMore = $state(false);
    let isLoadingContent = $state(false);
    let errorMsg = $state("");
    let sectionElement = $state<HTMLElement>();
    let searchTimeout: number | null = null;

    const hasMoreOnlineSearch = $derived(
        libraryState.onlineSearchResults.length <
            libraryState.onlineSearchTotal &&
            libraryState.onlineSearchResults.length > 0,
    );
    const showingSearch = $derived(
        (libraryState.selection.type === "all" ||
            uiState.libraryMode === "online") &&
            libraryState.searchQuery.trim().length > 0,
    );
    const hasMoreContent = $derived(
        currentOffset + contentTracks.length < totalTracks &&
            contentTracks.length > 0,
    );
    const hasMoreSearch = $derived(
        searchOffset + searchResults.length < searchTotal &&
            searchResults.length > 0,
    );

    function getSelectionKey(sel: Selection): string {
        switch (sel.type) {
            case "all":
                return "all";
            case "artist":
                return `artist:${sel.artist.id}`;
            case "album":
            case "artist-album":
            case "genre-album":
                return `album:${sel.album.id}`;
            case "genre":
                return `genre:${sel.genre.id}`;
            case "queue":
                return "queue";
            default:
                return "unknown";
        }
    }

    function sortParams() {
        return {
            sortColumn: sortColumn === "default" ? null : sortColumn,
            sortDir: sortDirection,
        };
    }

    async function fetchTracksPage(
        sel: Selection,
        limit: number,
        offset: number,
    ): Promise<PaginatedTracks> {
        const sort = sortParams();
        switch (sel.type) {
            case "all":
                return TauriService.getTracksPage(limit, offset, sort.sortColumn, sort.sortDir);
            case "artist":
                return TauriService.getTracksByArtistPage(sel.artist.id, limit, offset, sort.sortColumn, sort.sortDir);
            case "artist-album":
            case "genre-album":
            case "album":
                return TauriService.getTracksByAlbumPage(sel.album.id, limit, offset, sort.sortColumn, sort.sortDir);
            case "genre":
                return TauriService.getTracksByGenrePage(sel.genre.id, limit, offset, sort.sortColumn, sort.sortDir);
            case "queue":
                return { tracks: [], total: 0 };
        }
    }

    // content loading
    async function loadContent() {
        if (libraryState.selection.type === "queue") {
            isLoadingContent = false;
            errorMsg = "";
            return;
        }
        try {
            if (contentTracks.length === 0) isLoadingContent = true;
            errorMsg = "";
            currentOffset = 0;
            const result = await fetchTracksPage(
                libraryState.selection,
                PAGE_SIZE,
                0,
            );
            contentTracks = result.tracks;
            totalTracks = result.total;
        } catch (e) {
            errorMsg = String(e);
        } finally {
            isLoadingContent = false;
            const savedScroll =
                libraryState.scrollPositions[
                    getSelectionKey(libraryState.selection)
                ] || 0;
            if (sectionElement)
                setTimeout(() => {
                    if (sectionElement) sectionElement.scrollTop = savedScroll;
                }, 50);
        }
    }

    async function loadMoreContent() {
        if (isLoadingMore || !hasMoreContent) return;
        try {
            isLoadingMore = true;
            const result = await fetchTracksPage(
                libraryState.selection,
                PAGE_SIZE,
                contentTracks.length,
            );
            contentTracks = [...contentTracks, ...result.tracks];
            totalTracks = result.total;
        } catch (e) {
            console.error("Failed to load more tracks:", e);
        } finally {
            isLoadingMore = false;
        }
    }

    // search
    async function loadMoreSearchResults() {
        if (isSearchingMore || !hasMoreSearch) return;
        const query = libraryState.searchQuery.trim();
        if (!query) return;
        try {
            isSearchingMore = true;
            const sort = sortParams();
            const result = await TauriService.searchTracksPaginated(
                query,
                PAGE_SIZE,
                searchResults.length,
                sort.sortColumn,
                sort.sortDir
            );
            searchResults = [...searchResults, ...result.tracks];
            searchTotal = result.total;
        } catch (e) {
            console.error("Failed to load more search results:", e);
        } finally {
            isSearchingMore = false;
        }
    }

    async function loadMoreOnlineTracks() {
        if (isOnlineSearchingMore || !hasMoreOnlineSearch) return;
        const query = libraryState.searchQuery.trim();
        if (!query) return;
        try {
            isOnlineSearchingMore = true;
            await libraryState.searchOnline(
                query,
                PAGE_SIZE,
                libraryState.onlineSearchResults.length,
            );
        } catch (e) {
            console.error("Failed to load more online tracks:", e);
        } finally {
            isOnlineSearchingMore = false;
        }
    }

    function removeTrackFromUI(id: string) {
        contentTracks = contentTracks.filter((t) => t.id !== id);
        searchResults = searchResults.filter((t) => t.id !== id);
        totalTracks = Math.max(0, totalTracks - 1);
        searchTotal = Math.max(0, searchTotal - 1);
    }

    async function handleDelete(id: string) {
        try {
            await TauriService.deleteTrack(id);
            removeTrackFromUI(id);
            libraryState.refresh();
        } catch (e) {
            errorMsg = String(e);
        }
    }

    async function handleDeleteFile(id: string) {
        try {
            await TauriService.deleteTrackFile(id);
            removeTrackFromUI(id);
            libraryState.refresh();
        } catch (e) {
            errorMsg = String(e);
        }
    }

    async function handleArtistClick(name: string) {
        await libraryState.selectArtist(name);
    }
    async function handleAlbumClick(id: string) {
        await libraryState.selectAlbum(id);
    }

    $effect.root(() => {
        const key = getSelectionKey(libraryState.selection);
        const saved = libraryState.sortModes[key];
        if (saved) {
            sortColumn = saved.column;
            sortDirection = saved.direction;
        } else if (libraryState.selection.type === "all") {
            sortColumn = libraryState.allTracksSortColumn;
            sortDirection = libraryState.allTracksSortDirection;
        } else {
            sortColumn = "default";
            sortDirection = "desc";
        }
    });

    $effect(() => {
        const sel = libraryState.selection;
        untrack(() => {
            const key = getSelectionKey(sel);
            const saved = libraryState.sortModes[key];
            if (saved) {
                sortColumn = saved.column;
                sortDirection = saved.direction;
            } else if (sel.type === "all") {
                sortColumn = libraryState.allTracksSortColumn;
                sortDirection = libraryState.allTracksSortDirection;
            } else {
                sortColumn = "default";
                sortDirection = "desc";
            }
        });
    });

    $effect(() => {
        libraryState.sortModes[getSelectionKey(libraryState.selection)] = {
            column: sortColumn,
            direction: sortDirection,
        };
    });

    $effect(() => {
        if (libraryState.selection.type === "all") {
            libraryState.allTracksSortColumn = sortColumn;
            libraryState.allTracksSortDirection = sortDirection;
            localStorage.setItem("allTracksSortColumn", sortColumn);
            localStorage.setItem("allTracksSortDirection", sortDirection);
        }
    });

    $effect(() => {
        void libraryState.refreshTrigger;
        void sortColumn;
        void sortDirection;
        const currentSelection = libraryState.selection;
        untrack(() => {
            loadContent();
        });
        return () => {
            if (sectionElement)
                libraryState.scrollPositions[
                    getSelectionKey(currentSelection)
                ] = sectionElement.scrollTop;
        };
    });

    $effect(() => {
        const query = libraryState.searchQuery.trim();
        const mode = uiState.libraryMode;
        void sortColumn;
        void sortDirection;
        if (searchTimeout) clearTimeout(searchTimeout);

        if (!query) {
            searchResults = [];
            searchTotal = 0;
            searchOffset = 0;
            isSearching = false;
            if (mode === "online") {
                libraryState.onlineSearchResults = [];
                libraryState.onlineSearchTotal = 0;
            }
            return;
        }

        isSearching = true;
        searchTimeout = window.setTimeout(async () => {
            if (mode === "online") {
                await libraryState.searchOnline(query);
                isSearching = false;
                return;
            }
            try {
                searchOffset = 0;
                const sort = sortParams();
                const result = await TauriService.searchTracksPaginated(
                    query,
                    PAGE_SIZE,
                    0,
                    sort.sortColumn,
                    sort.sortDir
                );
                searchResults = result.tracks;
                searchTotal = result.total;
            } catch (e) {
                console.error("Search failed:", e);
            } finally {
                isSearching = false;
            }
        }, 250);
    });
</script>

<section
    bind:this={sectionElement}
    class="flex-1 min-w-0 overflow-y-auto pb-24"
>
    {#if errorMsg}
        <div class="mb-4">
            <div
                class="p-3 bg-red-500/10 border border-red-500/20 rounded-xl text-red-400 text-sm"
            >
                {errorMsg}
            </div>
        </div>
    {/if}

    {#if showingSearch}
        <SearchResultsView
            {searchResults}
            {searchTotal}
            {isSearching}
            sortOptions={TRACK_SORT_OPTIONS}
            bind:sortColumn
            bind:sortDirection
            {hasMoreOnlineSearch}
            {hasMoreSearch}
            {isOnlineSearchingMore}
            {isSearchingMore}
            ondelete={handleDelete}
            ondeletefile={handleDeleteFile}
            onloadmoreonline={loadMoreOnlineTracks}
            onloadmoresearch={loadMoreSearchResults}
            onartistclick={handleArtistClick}
            onalbumclick={handleAlbumClick}
        />
    {:else if uiState.libraryMode === "online"}
        <OnlinePlaceholder />
    {:else if isLoadingContent}
        <div class="text-center py-12">
            <p class="text-gray-400">Loading...</p>
        </div>
    {:else if contentTracks.length === 0}
        <div class="text-center py-12">
            <p class="text-gray-400 text-lg mb-4">
                {#if libraryState.selection.type === "all"}Your library is empty{:else}No
                    tracks found{/if}
            </p>
            {#if libraryState.selection.type === "all"}
                <p class="text-gray-500 text-sm">
                    Click "Scan Music" to add tracks from your Music folder
                </p>
            {/if}
        </div>
    {:else if libraryState.selection.type === "all"}
        <AllTracksPage
            tracks={contentTracks}
            {totalTracks}
            ondelete={handleDelete}
            ondeletefile={handleDeleteFile}
            onloadmore={loadMoreContent}
            hasMore={hasMoreContent}
            {isLoadingMore}
            onartistclick={handleArtistClick}
            onalbumclick={handleAlbumClick}
            bind:sortColumn
            bind:sortDirection
        />
    {:else if libraryState.selection.type === "artist"}
        <ArtistPage
            artist={libraryState.selection.artist}
            tracks={contentTracks}
            {totalTracks}
            ondelete={handleDelete}
            ondeletefile={handleDeleteFile}
            onalbumclick={handleAlbumClick}
            bind:sortColumn
            bind:sortDirection
        />
    {:else if libraryState.selection.type === "album" || libraryState.selection.type === "artist-album" || libraryState.selection.type === "genre-album"}
        <AlbumPage
            album={libraryState.selection.album}
            tracks={contentTracks}
            {totalTracks}
            ondelete={handleDelete}
            ondeletefile={handleDeleteFile}
            onartistclick={handleArtistClick}
        />
    {:else if libraryState.selection.type === "genre"}
        <GenrePage
            genre={libraryState.selection.genre}
            tracks={contentTracks}
            {totalTracks}
            ondelete={handleDelete}
            ondeletefile={handleDeleteFile}
            onartistclick={handleArtistClick}
            onalbumclick={handleAlbumClick}
            bind:sortColumn
            bind:sortDirection
        />
    {:else if libraryState.selection.type === "queue"}
        <QueuePage
            ondelete={handleDelete}
            onartistclick={handleArtistClick}
            onalbumclick={handleAlbumClick}
        />
    {/if}
</section>
