<script lang="ts">
    import { onMount, untrack } from "svelte";
    import { TauriService } from "$lib/utils/tauri";
    import { ChevronDown, ChevronUp, RefreshCw } from "@lucide/svelte";
    import {
        TRACK_SORT_OPTIONS,
        ALBUM_SORT_OPTIONS,
    } from "$lib/constants/sortOptions";
    import type {
        Track,
        ArtistWithCount,
        AlbumWithCount,
        SortColumn,
        SortDirection,
    } from "$lib/types";
    import type { ContextMenuItem } from "$lib/state/context_menu.svelte";
    import TrackList from "../TrackList.svelte";
    import ContentHeader from "../ContentHeader.svelte";
    import AlbumCard from "../AlbumCard.svelte";
    import SortDropdown from "../SortDropdown.svelte";
    import { libraryState } from "$lib/state/library.svelte";

    interface Props {
        artist: ArtistWithCount;
        tracks: Track[];
        totalTracks: number;
        ondelete: (id: string) => void;
        ondeletefile: (id: string) => void;
        onalbumclick: (albumId: string) => void;
        sortColumn: SortColumn;
        sortDirection: SortDirection;
    }

    let {
        artist,
        tracks,
        totalTracks,
        ondelete,
        ondeletefile,
        onalbumclick,
        sortColumn = $bindable(),
        sortDirection = $bindable(),
    }: Props = $props();

    let albumSortColumn = $state<SortColumn>("release_date");
    let albumSortDirection = $state<SortDirection>("desc");

    let albums = $state<AlbumWithCount[]>([]);
    let isLoadingAlbums = $state(false);
    let showAllAlbums = $state(false);

    let fetchedHash = $state<string | null>(null);
    let artistImageHash = $derived(fetchedHash ?? artist.image_hash);

    let isFetchingMetadata = $state(false);
    let lastFetchedId = "";

    const artistMenuItems: ContextMenuItem[] = [
        {
            label: "Refetch Metadata",
            icon: RefreshCw,
            onclick: () => fetchMetadata(artist.id, "auto"),
        },
    ];

    const visibleAlbums = $derived(showAllAlbums ? albums : albums.slice(0, 6));

    async function fetchMetadata(
        id: string,
        provider: "musicbrainz" | "itunes" | "auto" = "auto",
    ) {
        if (isFetchingMetadata && id === lastFetchedId && provider === "auto") {
            return;
        }

        isFetchingMetadata = true;
        lastFetchedId = id;

        try {
            const hash = await TauriService.fetchArtistMetadata(id, provider);
            if (id === artist.id) {
                fetchedHash = hash;
            }
        } catch (e) {
            console.error("Failed to fetch artist metadata:", e);
        } finally {
            if (id === artist.id) {
                isFetchingMetadata = false;
            }
        }
    }

    async function loadAlbums() {
        try {
            isLoadingAlbums = true;
            const rawAlbums = await TauriService.getAlbumsByArtist(
                artist.id,
                null,
                albumSortColumn === "default" ? null : albumSortColumn,
                albumSortDirection
            );

            // we prioritize non-featured albums
            albums = rawAlbums.sort((a, b) => {
                const aIsMain = a.album_artist_ids.includes(artist.id);
                const bIsMain = b.album_artist_ids.includes(artist.id);
                if (aIsMain && !bIsMain) return -1;
                if (!aIsMain && bIsMain) return 1;
                return 0;
            });
        } catch (e) {
            console.error("Failed to load artist albums:", e);
        } finally {
            isLoadingAlbums = false;
        }
    }

    $effect(() => {
        // Trigger reload on sort or artist change
        const _trigger = libraryState.refreshTrigger;
        const _id = artist.id;
        const _sort = { albumSortColumn, albumSortDirection };

        untrack(() => {
            loadAlbums();
        });
    });

    $effect(() => {
        const id = artist.id;
        if (!artist.image_hash) {
            fetchedHash = null;
            isFetchingMetadata = true;
            untrack(() => fetchMetadata(id, "auto"));
        } else {
            fetchedHash = null;
            isFetchingMetadata = false;
        }
    });
</script>

{#snippet artistSubtitle()}
    <div class="flex items-center gap-3">
        <span class="text-sm font-bold uppercase tracking-wider text-gray-400"
            >Artist</span
        >
    </div>
{/snippet}

<ContentHeader
    title={artist.name}
    subtitleSnippet={artistSubtitle}
    {totalTracks}
    contentAlbum={null}
    imageHash={artistImageHash}
    roundImage={false}
    isLoading={isFetchingMetadata}
    coverMenuItems={artistMenuItems}
/>

{#if albums.length > 0}
    <div class="mb-12">
        <div class="flex items-center justify-between mb-4">
            <h2 class="text-xl font-bold">Albums</h2>
            <div class="flex items-center gap-4">
                <SortDropdown
                    options={ALBUM_SORT_OPTIONS}
                    bind:column={albumSortColumn}
                    bind:direction={albumSortDirection}
                />
                {#if albums.length > 6}
                    <button
                        onclick={() => (showAllAlbums = !showAllAlbums)}
                        class="text-xs font-bold uppercase tracking-wider text-gray-500 hover:text-white transition flex items-center gap-1"
                    >
                        {#if showAllAlbums}
                            Show Less <ChevronUp size={14} />
                        {:else}
                            Show All ({albums.length}) <ChevronDown size={14} />
                        {/if}
                    </button>
                {/if}
            </div>
        </div>

        <div
            class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-4"
        >
            {#each visibleAlbums as album (album.id)}
                <AlbumCard
                    {album}
                    onclick={onalbumclick}
                    isFeatured={!album.album_artist_ids.includes(artist.id)}
                />
            {/each}
        </div>
    </div>
{/if}

<div>
    <div class="flex items-center justify-between mb-4">
        <h2 class="text-xl font-bold">All Tracks</h2>
        <SortDropdown
            options={TRACK_SORT_OPTIONS}
            bind:column={sortColumn}
            bind:direction={sortDirection}
        />
    </div>
    <TrackList
        {tracks}
        {ondelete}
        {ondeletefile}
        {onalbumclick}
        layout="grid"
    />
</div>
