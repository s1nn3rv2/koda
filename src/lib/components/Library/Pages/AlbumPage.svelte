<script lang="ts">
    import { onMount, untrack } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import type { Track, AlbumWithCount } from "$lib/types";
    import type { ContextMenuItem } from "$lib/state/context_menu.svelte";
    import { RefreshCw } from "@lucide/svelte";
    import { pluralize, formatDate } from "$lib/utils/format";
    import TrackList from "../TrackList.svelte";
    import ContentHeader from "../ContentHeader.svelte";
    import { queueState } from "$lib/state/player.svelte";

    interface Props {
        album: AlbumWithCount;
        tracks: Track[];
        totalTracks: number;
        ondelete: (id: string) => void;
        ondeletefile: (id: string) => void;
        onartistclick: (artistName: string) => void;
    }

    let {
        album,
        tracks,
        totalTracks,
        ondelete,
        ondeletefile,
        onartistclick,
    }: Props = $props();

    let fetchedDate = $state<string | null>(null);
    let fetchedHash = $state<string | null>(null);

    let albumMetadata = $derived({
        release_date: fetchedDate ?? album.release_date,
        cover_hash: fetchedHash ?? album.cover_hash,
    });

    const effectiveReleaseDate = $derived(fetchedDate ?? album.release_date);

    let fetching = $state(false);
    let lastFetchedId = "";

    async function fetchMetadata(
        id: string,
        provider: "musicbrainz" | "itunes" | "auto" = "auto",
        force = false,
    ) {
        if (!force && fetching && id === lastFetchedId && provider === "auto") {
            return;
        }

        fetching = true;
        lastFetchedId = id;

        try {
            const result = await invoke<[string | null, string | null]>(
                "fetch_album_metadata",
                {
                    albumId: id,
                    provider,
                    force,
                },
            );

            if (id === album.id) {
                const [date, hash] = result;
                fetchedDate = date;
                fetchedHash = hash;
            }
        } catch (e) {
            console.error("Failed to fetch album metadata:", e);
        } finally {
            if (id === album.id) {
                fetching = false;
            }
        }
    }

    $effect(() => {
        const id = album.id;
        if (!album.release_date || !album.cover_hash) {
            fetchedDate = null;
            fetchedHash = null;
            fetching = true;
            untrack(() => fetchMetadata(id, "auto"));
        } else {
            fetchedDate = null;
            fetchedHash = null;
            fetching = false;
        }
    });

    const coverMenuItems: ContextMenuItem[] = [
        {
            label: "Refetch Metadata",
            icon: RefreshCw,
            onclick: () => fetchMetadata(album.id, "auto", true),
        },
    ];
</script>

{#snippet albumArtists()}
    <div class="flex items-center gap-1.5 flex-wrap">
        <span>Album by</span>
        <div class="flex items-center">
            {#each album.album_artist_names as name, i}
                {#if i > 0}<span class="mx-1 opacity-50">&</span>{/if}
                <button
                    onclick={(e) => {
                        e.stopPropagation();
                        onartistclick(name);
                    }}
                    class="hover:text-white hover:underline transition-colors"
                >
                    {name}
                </button>
            {/each}
        </div>
        {#if album.album_artist_names.length === 0}
            <span class="italic opacity-50">Unknown Artist</span>
        {/if}
    </div>
{/snippet}

<ContentHeader
    title={album.name}
    subtitleSnippet={albumArtists}
    {totalTracks}
    contentAlbum={{ ...album, cover_hash: albumMetadata.cover_hash }}
    isLoading={fetching}
    {coverMenuItems}
    releaseDate={formatDate(effectiveReleaseDate)}
    onplay={() => queueState.playTrack(tracks[0], tracks)}
/>

<TrackList
    {tracks}
    {ondelete}
    {ondeletefile}
    {onartistclick}
    isAlbumView={true}
    layout="list"
/>
