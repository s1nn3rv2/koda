<script lang="ts">
    import { ChevronRight, ChevronDown, Disc3 } from "@lucide/svelte";
    import type { AlbumWithCount, Selection } from "$lib/types";

    interface Props {
        albums: AlbumWithCount[];
        expanded: boolean;
        onToggle: () => void;
        onSelectAlbum: (album: AlbumWithCount) => void;
        isSelected: (check: Selection) => boolean;
        itemBase: string;
        itemIdle: string;
        itemActive: string;
        searchQuery?: string;
    }

    let {
        albums,
        expanded,
        onToggle,
        onSelectAlbum,
        isSelected,
        itemBase,
        itemIdle,
        itemActive,
        searchQuery = "",
    }: Props = $props();

    function isFeatured(album: AlbumWithCount): boolean {
        if (!searchQuery) return false;
        const q = searchQuery.toLowerCase();

        const isMainArtist = album.album_artist_names.some((name) =>
            name.toLowerCase().includes(q),
        );
        if (isMainArtist) return false;

        const matchesTrackArtist = album.artist_name?.toLowerCase().includes(q);
        return !!matchesTrackArtist;
    }
</script>

<div class="mt-3">
    <button
        onclick={onToggle}
        class="flex items-center gap-1 w-full px-2 py-1 text-xs font-semibold uppercase tracking-wider text-gray-500 hover:text-gray-300 transition"
    >
        {#if expanded}
            <ChevronDown size={12} class="shrink-0" />
        {:else}
            <ChevronRight size={12} class="shrink-0" />
        {/if}
        Albums
        <span class="ml-auto text-gray-600 font-normal normal-case">
            {albums.length}
        </span>
    </button>

    {#if expanded}
        <div class="ml-2 flex flex-col gap-0.5 mt-0.5">
            {#each albums as album (album.id)}
                <button
                    onclick={() => onSelectAlbum(album)}
                    class="{itemBase} {isSelected({ type: 'album', album })
                        ? itemActive
                        : itemIdle}"
                >
                    <Disc3 size={13} class="shrink-0" />
                    <span class="truncate flex items-center gap-1.5">
                        {album.name}
                        {#if isFeatured(album)}
                            <span
                                class="text-[10px] px-1 py-0.5 rounded bg-white/5 text-gray-500 uppercase font-bold tracking-tighter shrink-0"
                            >
                                Featured
                            </span>
                        {/if}
                    </span>
                    <span class="ml-auto text-xs text-gray-600 shrink-0">
                        {album.track_count}
                    </span>
                </button>
            {/each}
        </div>
    {/if}
</div>
