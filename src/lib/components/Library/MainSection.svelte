<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount, untrack } from "svelte";
    import { Globe } from "@lucide/svelte";
    import { uiState } from "$lib/state/player.svelte";
    import type {
        Track,
        AlbumWithCount,
        PaginatedTracks,
        Selection,
        SortColumn,
        SortDirection,
    } from "$lib/types";
    import { libraryState } from "$lib/state/library.svelte";
    import { monochromeService } from "$lib/services/monochrome";
    import { TauriService } from "$lib/utils/tauri";
    import { pluralize } from "$lib/utils/format";
    import { splitArtists } from "$lib/utils/split_artists";
    import TrackList from "./TrackList.svelte";
    import AllTracksPage from "./Pages/AllTracksPage.svelte";
    import AlbumPage from "./Pages/AlbumPage.svelte";
    import ArtistPage from "./Pages/ArtistPage.svelte";
    import GenrePage from "./Pages/GenrePage.svelte";
    import QueuePage from "./Pages/QueuePage.svelte";
    import ContentHeader from "./ContentHeader.svelte";
    import SortDropdown from "./SortDropdown.svelte";

    const PAGE_SIZE = 50;

    let sortColumn = $state<SortColumn>(
        libraryState.selection.type === "all"
            ? libraryState.allTracksSortColumn
            : "default",
    );
    let sortDirection = $state<SortDirection>("desc");

    $effect.root(() => {
        const key = getSelectionKey(libraryState.selection);
        const savedSort = libraryState.sortModes[key];
        if (savedSort) {
            sortColumn = savedSort.column;
            sortDirection = savedSort.direction;
        } else if (libraryState.selection.type === "all") {
            sortColumn = libraryState.allTracksSortColumn;
            sortDirection = libraryState.allTracksSortDirection;
        } else {
            sortColumn = "default";
            sortDirection = "desc";
        }
    });
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
    const hasMoreOnlineSearch = $derived(
        libraryState.onlineSearchResults.length <
            libraryState.onlineSearchTotal &&
            libraryState.onlineSearchResults.length > 0,
    );

    let isLoadingContent = $state(false);
    let errorMsg = $state("");
    let sectionElement = $state<HTMLElement>();

    function getSelectionKey(sel: Selection): string {
        switch (sel.type) {
            case "all":
                return "all";
            case "artist":
                return `artist:${sel.artist.id}`;
            case "album":
                return `album:${sel.album.id}`;
            case "artist-album":
                return `album:${sel.album.id}`;
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

    async function fetchTracksPage(
        sel: Selection,
        limit: number,
        offset: number,
    ): Promise<PaginatedTracks> {
        switch (sel.type) {
            case "all":
                return await invoke<PaginatedTracks>("get_tracks_page", {
                    limit,
                    offset,
                    sortColumn: sortColumn === "default" ? null : sortColumn,
                    sortDir: sortDirection,
                });
            case "artist":
                return await invoke<PaginatedTracks>(
                    "get_tracks_by_artist_page",
                    {
                        artistId: sel.artist.id,
                        limit,
                        offset,
                        sortColumn:
                            sortColumn === "default" ? null : sortColumn,
                        sortDir: sortDirection,
                    },
                );
            case "artist-album":
            case "genre-album":
            case "album":
                return await invoke<PaginatedTracks>(
                    "get_tracks_by_album_page",
                    {
                        albumId: sel.album.id,
                        limit,
                        offset,
                        sortColumn:
                            sortColumn === "default" ? null : sortColumn,
                        sortDir: sortDirection,
                    },
                );
            case "genre":
                return await invoke<PaginatedTracks>(
                    "get_tracks_by_genre_page",
                    {
                        genreId: sel.genre.id,
                        limit,
                        offset,
                        sortColumn:
                            sortColumn === "default" ? null : sortColumn,
                        sortDir: sortDirection,
                    },
                );
            case "queue":
                return { tracks: [], total: 0 };
        }
    }

    async function loadContent() {
        if (libraryState.selection.type === "queue") {
            isLoadingContent = false;
            errorMsg = "";
            return;
        }

        try {
            if (contentTracks.length === 0) {
                isLoadingContent = true;
            }
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

            const key = getSelectionKey(libraryState.selection);
            const savedScroll = libraryState.scrollPositions[key] || 0;
            if (sectionElement) {
                setTimeout(() => {
                    if (sectionElement) sectionElement.scrollTop = savedScroll;
                }, 50);
            }
        }
    }

    async function loadMoreContent() {
        if (isLoadingMore || !hasMoreContent) return;

        try {
            isLoadingMore = true;
            const nextOffset = contentTracks.length;

            const result = await fetchTracksPage(
                libraryState.selection,
                PAGE_SIZE,
                nextOffset,
            );
            contentTracks = [...contentTracks, ...result.tracks];
            totalTracks = result.total;
        } catch (e) {
            console.error("Failed to load more tracks:", e);
        } finally {
            isLoadingMore = false;
        }
    }

    $effect(() => {
        const selection = libraryState.selection;
        untrack(() => {
            const key = getSelectionKey(selection);
            const savedSort = libraryState.sortModes[key];
            if (savedSort) {
                sortColumn = savedSort.column;
                sortDirection = savedSort.direction;
            } else if (selection.type === "all") {
                sortColumn = libraryState.allTracksSortColumn;
                sortDirection = libraryState.allTracksSortDirection;
            } else {
                sortColumn = "default";
                sortDirection = "desc";
            }
        });
    });

    $effect(() => {
        const key = getSelectionKey(libraryState.selection);
        libraryState.sortModes[key] = {
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
            if (sectionElement) {
                const key = getSelectionKey(currentSelection);
                libraryState.scrollPositions[key] = sectionElement.scrollTop;
            }
        };
    });

    let searchTimeout: number | null = null;

    const searchSortOptions = [
        {
            label: "Default",
            column: "default" as SortColumn,
            defaultDir: "desc" as SortDirection,
        },
        {
            label: "Title",
            column: "title" as SortColumn,
            defaultDir: "asc" as SortDirection,
        },
        {
            label: "Date Added",
            column: "added_at" as SortColumn,
            defaultDir: "desc" as SortDirection,
        },
        {
            label: "Release Date",
            column: "release_date" as SortColumn,
            defaultDir: "desc" as SortDirection,
        },
        {
            label: "Duration",
            column: "duration" as SortColumn,
            defaultDir: "desc" as SortDirection,
        },
    ];

    $effect(() => {
        const query = libraryState.searchQuery.trim();
        const mode = uiState.libraryMode;
        void sortColumn;
        void sortDirection;
        if (searchTimeout) clearTimeout(searchTimeout);

        if (query.length === 0) {
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
                const result = await invoke<PaginatedTracks>(
                    "search_tracks_paginated",
                    {
                        query,
                        limit: PAGE_SIZE,
                        offset: 0,
                        sortColumn:
                            sortColumn === "default" ? null : sortColumn,
                        sortDir: sortDirection,
                    },
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

    async function loadMoreSearchResults() {
        if (isSearchingMore || !hasMoreSearch) return;

        const query = libraryState.searchQuery.trim();
        if (query.length === 0) return;

        try {
            isSearchingMore = true;
            const nextOffset = searchResults.length;

            const result = await invoke<PaginatedTracks>(
                "search_tracks_paginated",
                {
                    query,
                    limit: PAGE_SIZE,
                    offset: nextOffset,
                    sortColumn: sortColumn === "default" ? null : sortColumn,
                    sortDir: sortDirection,
                },
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
        if (query.length === 0) return;

        try {
            isOnlineSearchingMore = true;
            const nextOffset = libraryState.onlineSearchResults.length;
            await libraryState.searchOnline(query, PAGE_SIZE, nextOffset);
        } catch (e) {
            console.error("Failed to load more online tracks:", e);
        } finally {
            isOnlineSearchingMore = false;
        }
    }

    async function handleDelete(id: string) {
        try {
            await invoke("delete_track", { id });
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

    function removeTrackFromUI(id: string) {
        contentTracks = contentTracks.filter((t) => t.id !== id);
        searchResults = searchResults.filter((t) => t.id !== id);
        totalTracks = Math.max(0, totalTracks - 1);
        searchTotal = Math.max(0, searchTotal - 1);
    }

    async function handleArtistClick(artistName: string) {
        await libraryState.selectArtist(artistName);
    }

    async function handleAlbumClick(albumId: string) {
        await libraryState.selectAlbum(albumId);
    }
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
        <div class="mb-4">
            <ContentHeader
                title="Search Results"
                subtitle={isSearching || libraryState.isOnlineSearching
                    ? "Searching..."
                    : uiState.libraryMode === "online"
                      ? `${libraryState.onlineSearchTotal} ${pluralize(libraryState.onlineSearchTotal, "result")} for "${libraryState.searchQuery.trim()}"`
                      : `${searchTotal} ${pluralize(searchTotal, "result")} for "${libraryState.searchQuery.trim()}"`}
                totalTracks={uiState.libraryMode === "online"
                    ? libraryState.onlineSearchTotal
                    : searchTotal}
                contentAlbum={null}
                showCoverArt={false}
            >
                {#snippet actions()}
                    <SortDropdown
                        options={searchSortOptions}
                        bind:column={sortColumn}
                        bind:direction={sortDirection}
                    />
                {/snippet}
            </ContentHeader>
            <div
                class={isSearching || libraryState.isOnlineSearching
                    ? "opacity-50 pointer-events-none transition-opacity"
                    : "transition-opacity"}
            >
                <TrackList
                    tracks={uiState.libraryMode === "online"
                        ? libraryState.onlineSearchResults.map((t) =>
                              monochromeService.mapToTrack(t),
                          )
                        : searchResults}
                    ondelete={handleDelete}
                    ondeletefile={handleDeleteFile}
                    onloadmore={uiState.libraryMode === "online"
                        ? loadMoreOnlineTracks
                        : loadMoreSearchResults}
                    hasMore={uiState.libraryMode === "online"
                        ? hasMoreOnlineSearch
                        : hasMoreSearch}
                    isLoadingMore={uiState.libraryMode === "online"
                        ? isOnlineSearchingMore
                        : isSearchingMore}
                    onartistclick={handleArtistClick}
                    onalbumclick={handleAlbumClick}
                />
            </div>
        </div>
    {:else if uiState.libraryMode === "online"}
        <div
            class="flex flex-col items-center justify-center flex-1 h-full text-center p-12"
        >
            <div
                class="w-20 h-20 bg-indigo-500/10 rounded-3xl flex items-center justify-center mb-6 border border-indigo-500/20"
            >
                <Globe size={40} class="text-indigo-400" />
            </div>
            <h2 class="text-2xl font-bold text-white mb-2">Online Library</h2>
            <p class="text-gray-400 max-w-sm">
                Search for tracks, artists, or albums to browse the Monochrome
                database.
            </p>
        </div>
    {:else if isLoadingContent}
        <div class="text-center py-12">
            <p class="text-gray-400">Loading...</p>
        </div>
    {:else if contentTracks.length === 0}
        <div class="text-center py-12">
            <p class="text-gray-400 text-lg mb-4">
                {#if libraryState.selection.type === "all"}
                    Your library is empty
                {:else}
                    No tracks found
                {/if}
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
