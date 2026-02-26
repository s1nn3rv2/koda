<script lang="ts">
    import { Library, ListMusic, Globe } from "@lucide/svelte";
    import { TauriService } from "$lib/utils/tauri";
    import type {
        ArtistWithCount,
        AlbumWithCount,
        GenreWithCount,
        Track,
        Selection,
    } from "$lib/types";
    import { splitArtists } from "$lib/utils/split_artists";
    import { libraryState } from "$lib/state/library.svelte";
    import { uiState, queueState } from "$lib/state/player.svelte";
    import ArtistsSection from "./ArtistsSection.svelte";
    import AlbumsSection from "./AlbumsSection.svelte";
    import GenresSection from "./GenresSection.svelte";

    interface Props {
        loadKey: number;
        selection: Selection;
        onselect: (selection: Selection) => void;
        searchQuery?: string;
    }

    let { loadKey, selection, onselect, searchQuery = "" }: Props = $props();

    let artistsExpanded = $state(false);
    let albumsExpanded = $state(false);
    let genresExpanded = $state(false);

    let artists = $state<ArtistWithCount[]>([]);
    let albums = $state<AlbumWithCount[]>([]);
    let genres = $state<GenreWithCount[]>([]);

    let relatedArtistNames = $state(new Set<string>());
    let relatedAlbumIds = $state(new Set<string>());
    let relatedGenreIds = $state(new Set<string>());
    let searchTimer: number | null = null;

    $effect(() => {
        if (!searchQuery) {
            relatedArtistNames = new Set();
            relatedAlbumIds = new Set();
            relatedGenreIds = new Set();
            libraryState.relatedArtistIds = new Set();
            if (uiState.libraryMode === "local") {
                loadAll();
            } else {
                libraryState.onlineSearchResults = [];
            }
            return;
        }

        if (searchTimer) clearTimeout(searchTimer);
        searchTimer = window.setTimeout(async () => {
            try {
                const tracks = await TauriService.searchTracks(searchQuery);
                const rArtists = new Set<string>();
                const rArtistIds = new Set<string>();
                const rAlbums = new Set<string>();
                const rGenres = new Set<string>();

                for (const track of tracks) {
                    if (track.artists) {
                        splitArtists(track.artists).forEach((a) =>
                            rArtists.add(a.toLowerCase()),
                        );
                    }

                    if (track.album_id) rAlbums.add(track.album_id);
                    if (track.genre_id) rGenres.add(track.genre_id);
                }

                const artistsResult = await TauriService.getAllArtists(searchQuery);
                for (const a of artistsResult) {
                    rArtistIds.add(a.id);
                }

                relatedArtistNames = rArtists;
                relatedAlbumIds = rAlbums;
                relatedGenreIds = rGenres;
                libraryState.relatedArtistIds = rArtistIds;

                await loadAll();
            } catch (e) {
                console.error(e);
            }
        }, 300);
    });

    let filteredArtists = $derived.by(() => {
        if (!searchQuery) return artists;
        const query = searchQuery.toLowerCase();
        return artists
            .filter(
                (a) =>
                    a.name.toLowerCase().includes(query) ||
                    relatedArtistNames.has(a.name.toLowerCase()),
            )
            .sort((a, b) => {
                const aName = a.name.toLowerCase();
                const bName = b.name.toLowerCase();

                // Exact match
                if (aName === query && bName !== query) return -1;
                if (bName === query && aName !== query) return 1;

                // Starts with
                if (aName.startsWith(query) && !bName.startsWith(query))
                    return -1;
                if (bName.startsWith(query) && !aName.startsWith(query))
                    return 1;

                // Both contain or both related - alphabetical
                return aName.localeCompare(bName);
            });
    });

    let filteredAlbums = $derived.by(() => {
        if (!searchQuery) return albums;
        const query = searchQuery.toLowerCase();
        return albums
            .filter(
                (a) =>
                    a.name.toLowerCase().includes(query) ||
                    (a.artist_name &&
                        a.artist_name.toLowerCase().includes(query)) ||
                    relatedAlbumIds.has(a.id),
            )
            .sort((a, b) => {
                const aName = a.name.toLowerCase();
                const bName = b.name.toLowerCase();

                // prioritize non-featured
                const aIsMain = a.album_artist_names.some((n) =>
                    n.toLowerCase().includes(query),
                );
                const bIsMain = b.album_artist_names.some((n) =>
                    n.toLowerCase().includes(query),
                );
                if (aIsMain && !bIsMain) return -1;
                if (!aIsMain && bIsMain) return 1;

                // do exact match on title
                if (aName === query && bName !== query) return -1;
                if (bName === query && aName !== query) return 1;

                // starts with on title
                if (aName.startsWith(query) && !bName.startsWith(query))
                    return -1;
                if (bName.startsWith(query) && !aName.startsWith(query))
                    return 1;

                return aName.localeCompare(bName);
            });
    });

    let filteredGenres = $derived.by(() => {
        if (!searchQuery) return genres;
        const query = searchQuery.toLowerCase();
        return genres
            .filter(
                (g) =>
                    g.name.toLowerCase().includes(query) ||
                    relatedGenreIds.has(g.id),
            )
            .sort((a, b) => {
                const aName = a.name.toLowerCase();
                const bName = b.name.toLowerCase();

                if (aName === query && bName !== query) return -1;
                if (bName === query && aName !== query) return 1;

                if (aName.startsWith(query) && !bName.startsWith(query))
                    return -1;
                if (bName.startsWith(query) && !aName.startsWith(query))
                    return 1;

                return aName.localeCompare(bName);
            });
    });

    $effect(() => {
        if (searchQuery.length > 0) {
            artistsExpanded = true;
            albumsExpanded = true;
            genresExpanded = true;
        }
    });

    let expandedArtistId = $state<string | null>(null);
    let expandedGenreId = $state<string | null>(null);

    let artistAlbums = $state<Map<string, AlbumWithCount[]>>(new Map());
    let genreAlbums = $state<Map<string, AlbumWithCount[]>>(new Map());

    $effect(() => {
        void loadKey;
        void uiState.libraryMode;
        loadAll();
    });

    async function loadAll() {
        if (uiState.libraryMode === "online") {
            artists = [];
            albums = [];
            genres = [];
            return;
        }
        try {
            const [a, al, g] = await Promise.all([
                TauriService.getAllArtists(searchQuery || null),
                TauriService.getAllAlbums(searchQuery || null),
                TauriService.getAllGenres(searchQuery || null),
            ]);
            artists = a;

            albums = al;
            genres = g;

            // refresh expanded subviews
            if (expandedArtistId) {
                await loadArtistAlbums(expandedArtistId);
            }
            if (expandedGenreId) {
                await loadGenreAlbums(expandedGenreId);
            }
        } catch (e) {
            console.error("Failed to load sidebar data:", e);
        }
    }

    async function loadArtistAlbums(artistId: string) {
        try {
            const result = await TauriService.getAlbumsByArtist(artistId, searchQuery || null);
            artistAlbums = new Map(artistAlbums).set(artistId, result);
        } catch (e) {
            console.error("Failed to load albums for artist:", e);
        }
    }

    async function loadGenreAlbums(genreId: string) {
        try {
            const result = await TauriService.getAlbumsByGenre(genreId, searchQuery || null);
            genreAlbums = new Map(genreAlbums).set(genreId, result);
        } catch (e) {
            console.error("Failed to load albums for genre:", e);
        }
    }

    function selectAll() {
        onselect({ type: "all" });
    }

    function selectArtist(artist: ArtistWithCount) {
        onselect({ type: "artist", artist });
    }

    function selectAlbum(album: AlbumWithCount) {
        onselect({ type: "album", album });
    }

    function selectGenre(genre: GenreWithCount) {
        onselect({ type: "genre", genre });
    }

    function selectArtistAlbum(artist: ArtistWithCount, album: AlbumWithCount) {
        onselect({ type: "artist-album", artist, album });
    }

    function selectGenreAlbum(genre: GenreWithCount, album: AlbumWithCount) {
        onselect({ type: "genre-album", genre, album });
    }

    async function toggleArtistAlbums(artist: ArtistWithCount) {
        if (expandedArtistId === artist.id) {
            expandedArtistId = null;
        } else {
            expandedArtistId = artist.id;
            if (!artistAlbums.has(artist.id)) {
                await loadArtistAlbums(artist.id);
            }
        }
    }

    async function toggleGenreAlbums(genre: GenreWithCount) {
        if (expandedGenreId === genre.id) {
            expandedGenreId = null;
        } else {
            expandedGenreId = genre.id;
            if (!genreAlbums.has(genre.id)) {
                await loadGenreAlbums(genre.id);
            }
        }
    }

    function isSelected(check: Selection): boolean {
        if (selection.type !== check.type) return false;
        switch (check.type) {
            case "all":
                return true;
            case "queue":
                return selection.type === "queue";
            case "artist":
                return (
                    selection.type === "artist" &&
                    selection.artist.id === check.artist.id
                );
            case "album":
                return (
                    selection.type === "album" &&
                    selection.album.id === check.album.id
                );
            case "genre":
                return (
                    selection.type === "genre" &&
                    selection.genre.id === check.genre.id
                );
            case "artist-album":
                return (
                    selection.type === "artist-album" &&
                    selection.artist.id === check.artist.id &&
                    selection.album.id === check.album.id
                );
            case "genre-album":
                return (
                    selection.type === "genre-album" &&
                    selection.genre.id === check.genre.id &&
                    selection.album.id === check.album.id
                );
        }
    }

    const itemBase =
        "flex items-center gap-2 w-full px-2 py-1.5 rounded-lg text-sm transition truncate";
    const itemIdle = "text-gray-400 hover:text-white hover:bg-white/5";
    const itemActive = "text-white bg-white/10";
</script>

{#if uiState.libraryMode === "local"}
    <nav class="flex flex-col gap-0.5 select-none text-sm">
        <button
            onclick={selectAll}
            class="{itemBase} {isSelected({ type: 'all' })
                ? itemActive
                : itemIdle}"
        >
            <Library size={14} class="shrink-0" />
            <span class="truncate">All Tracks</span>
        </button>

        <button
            onclick={() => onselect({ type: "queue" })}
            class="{itemBase} {isSelected({ type: 'queue' })
                ? itemActive
                : itemIdle} flex items-center justify-between"
        >
            <div class="flex items-center gap-2 min-w-0">
                <ListMusic size={14} class="shrink-0" />
                <span class="truncate">Queue</span>
            </div>
            {#if queueState.queue.length > 0}
                <span
                    class="text-xs bg-white/10 px-1.5 py-0.5 rounded-md font-medium shrink-0"
                >
                    {queueState.queue.length}
                </span>
            {/if}
        </button>

        <ArtistsSection
            artists={filteredArtists}
            expanded={artistsExpanded}
            onToggle={() => (artistsExpanded = !artistsExpanded)}
            {expandedArtistId}
            {artistAlbums}
            onToggleArtist={toggleArtistAlbums}
            onSelectArtist={selectArtist}
            onSelectAlbum={selectArtistAlbum}
            {isSelected}
            {itemBase}
            {itemIdle}
            {itemActive}
        />

        <AlbumsSection
            albums={filteredAlbums}
            expanded={albumsExpanded}
            onToggle={() => (albumsExpanded = !albumsExpanded)}
            onSelectAlbum={selectAlbum}
            {isSelected}
            {itemBase}
            {itemIdle}
            {itemActive}
            {searchQuery}
        />

        <GenresSection
            genres={filteredGenres}
            expanded={genresExpanded}
            onToggle={() => (genresExpanded = !genresExpanded)}
            {expandedGenreId}
            {genreAlbums}
            onToggleGenre={toggleGenreAlbums}
            onSelectGenre={selectGenre}
            onSelectAlbum={selectGenreAlbum}
            {isSelected}
            {itemBase}
            {itemIdle}
            {itemActive}
            {searchQuery}
        />
    </nav>
{:else if !searchQuery}
    <div class="px-4 py-8 text-center">
        <Globe size={32} class="mx-auto text-indigo-500/30 mb-4" />
        <p class="text-xs text-gray-500 leading-relaxed">
            Search above to find music from the <span
                class="text-indigo-400 font-medium">Monochrome</span
            > network
        </p>
    </div>
{/if}
