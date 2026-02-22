<script lang="ts">
    import { ChevronRight, ChevronDown, Music, Disc3 } from "@lucide/svelte";
    import type { GenreWithCount, AlbumWithCount, Selection } from "$lib/types";

    interface Props {
        genres: GenreWithCount[];
        expanded: boolean;
        onToggle: () => void;
        expandedGenreId: string | null;
        genreAlbums: Map<string, AlbumWithCount[]>;
        onToggleGenre: (genre: GenreWithCount) => void;
        onSelectGenre: (genre: GenreWithCount) => void;
        onSelectAlbum: (genre: GenreWithCount, album: AlbumWithCount) => void;
        isSelected: (check: Selection) => boolean;
        itemBase: string;
        itemIdle: string;
        itemActive: string;
        searchQuery?: string;
    }

    let {
        genres,
        expanded,
        onToggle,
        expandedGenreId,
        genreAlbums,
        onToggleGenre,
        onSelectGenre,
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

        // If the search query matches any of the ALBUM artists, it's NOT a feature view
        const isMainArtist = album.album_artist_names.some((name) =>
            name.toLowerCase().includes(q),
        );
        if (isMainArtist) return false;

        // If it's not a main artist but matches the track artists, it's a feature
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
        Genres
        <span class="ml-auto text-gray-600 font-normal normal-case">
            {genres.length}
        </span>
    </button>

    {#if expanded}
        <div class="ml-2 flex flex-col gap-0.5 mt-0.5">
            {#each genres as genre (genre.id)}
                <div>
                    <div class="flex items-center">
                        <button
                            onclick={() => onToggleGenre(genre)}
                            class="p-1 text-gray-600 hover:text-gray-400 transition shrink-0 disabled:opacity-20 disabled:cursor-not-allowed"
                            title={genre.album_count > 0
                                ? "Show albums"
                                : "No albums"}
                            disabled={genre.album_count === 0}
                        >
                            {#if expandedGenreId === genre.id}
                                <ChevronDown size={12} />
                            {:else}
                                <ChevronRight size={12} />
                            {/if}
                        </button>
                        <button
                            onclick={() => onSelectGenre(genre)}
                            class="{itemBase} {isSelected({
                                type: 'genre',
                                genre,
                            })
                                ? itemActive
                                : itemIdle}"
                        >
                            <Music size={13} class="shrink-0" />
                            <span class="truncate">{genre.name}</span>
                            <span
                                class="ml-auto text-xs text-gray-600 shrink-0 flex gap-0.5"
                            >
                                <Music size={12} />{genre.track_count}
                                <Disc3 size={12} />{genre.album_count}
                            </span>
                        </button>
                    </div>

                    {#if expandedGenreId === genre.id}
                        <div class="ml-6 flex flex-col gap-0.5 mt-0.5">
                            {#each genreAlbums.get(genre.id) ?? [] as album (album.id)}
                                <button
                                    onclick={() => onSelectAlbum(genre, album)}
                                    class="{itemBase} {isSelected({
                                        type: 'genre-album',
                                        genre,
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
                                        {#if isFeatured(album)}
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
                            {#if (genreAlbums.get(genre.id) ?? []).length === 0}
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
