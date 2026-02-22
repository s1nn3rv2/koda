<script lang="ts">
    import {
        ChevronRight,
        ChevronDown,
        User,
        Music,
        Disc3,
    } from "@lucide/svelte";
    import type {
        ArtistWithCount,
        AlbumWithCount,
        Selection,
    } from "$lib/types";

    interface Props {
        artists: ArtistWithCount[];
        expanded: boolean;
        onToggle: () => void;
        expandedArtistId: string | null;
        artistAlbums: Map<string, AlbumWithCount[]>;
        onToggleArtist: (artist: ArtistWithCount) => void;
        onSelectArtist: (artist: ArtistWithCount) => void;
        onSelectAlbum: (artist: ArtistWithCount, album: AlbumWithCount) => void;
        isSelected: (check: Selection) => boolean;
        itemBase: string;
        itemIdle: string;
        itemActive: string;
    }

    let {
        artists,
        expanded = $bindable(),
        onToggle,
        expandedArtistId,
        artistAlbums,
        onToggleArtist,
        onSelectArtist,
        onSelectAlbum,
        isSelected,
        itemBase,
        itemIdle,
        itemActive,
    }: Props = $props();
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
        Artists
        <span
            class="ml-auto text-gray-600 font-normal normal-case flex items-center gap-0.5"
        >
            {artists?.length}
        </span>
    </button>

    {#if expanded}
        <div class="ml-2 flex flex-col gap-0.5 mt-0.5">
            {#each artists as artist (artist.id)}
                <div>
                    <div class="flex items-center">
                        <button
                            onclick={() => onToggleArtist(artist)}
                            class="p-1 text-gray-600 hover:text-gray-400 transition shrink-0 disabled:opacity-20 disabled:cursor-not-allowed"
                            title={artist.album_count > 0
                                ? "Show albums"
                                : "No albums"}
                            disabled={artist.album_count === 0}
                        >
                            {#if expandedArtistId === artist.id}
                                <ChevronDown size={12} />
                            {:else}
                                <ChevronRight size={12} />
                            {/if}
                        </button>
                        <button
                            onclick={() => onSelectArtist(artist)}
                            class="{itemBase} {isSelected({
                                type: 'artist',
                                artist,
                            })
                                ? itemActive
                                : itemIdle}"
                        >
                            <User size={14} class="shrink-0" />
                            <span class="truncate">{artist.name}</span>
                            <span
                                class="ml-auto text-xs text-gray-600 shrink-0 flex gap-0.5"
                            >
                                <Music size={12} />{artist.track_count}
                                <Disc3 size={12} />{artist.album_count}
                            </span>
                        </button>
                    </div>

                    {#if expandedArtistId === artist.id}
                        <div class="ml-6 flex flex-col gap-0.5 mt-0.5">
                            {#each artistAlbums.get(artist.id) ?? [] as album (album.id)}
                                <button
                                    onclick={() => onSelectAlbum(artist, album)}
                                    class="{itemBase} {isSelected({
                                        type: 'artist-album',
                                        artist,
                                        album,
                                    })
                                        ? itemActive
                                        : itemIdle}"
                                >
                                    <Disc3 size={13} class="shrink-0" />
                                    <span
                                        class="truncate flex items-center gap-1.5"
                                    >
                                        {album.name}
                                        {#if album.album_artist_ids.length > 0 && !album.album_artist_ids.includes(artist.id)}
                                            <span
                                                class="text-[10px] px-1 py-0.5 rounded bg-white/5 text-gray-500 uppercase font-bold tracking-tighter shrink-0"
                                            >
                                                Featured
                                            </span>
                                        {/if}
                                    </span>
                                    <span
                                        class="ml-auto text-xs text-gray-600 shrink-0"
                                    >
                                        {album.track_count}
                                    </span>
                                </button>
                            {/each}
                            {#if (artistAlbums.get(artist.id) ?? []).length === 0}
                                <p
                                    class="px-2 py-1 text-xs text-gray-600 italic"
                                >
                                    No albums
                                </p>
                            {/if}
                        </div>
                    {/if}
                </div>
            {/each}
        </div>
    {/if}
</div>
