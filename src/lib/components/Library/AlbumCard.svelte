<script lang="ts">
    import { TauriService } from "$lib/utils/tauri";
    import type { AlbumWithCount } from "$lib/types";
    import CoverArt from "$lib/components/CoverArt.svelte";
    import { formatDate } from "$lib/utils/format";
    import { contextMenuState } from "$lib/state/context_menu.svelte";
    import { Disc, User, RefreshCw } from "@lucide/svelte";
    import { libraryState } from "$lib/state/library.svelte";

    interface Props {
        album: AlbumWithCount;
        onclick: (id: string) => void;
        isFeatured?: boolean;
    }

    let { album, onclick, isFeatured = false }: Props = $props();

    function handleContextMenu(e: MouseEvent) {
        e.preventDefault();
        contextMenuState.open(e.clientX, e.clientY, [
            {
                label: "View Album",
                icon: Disc,
                onclick: () => onclick(album.id),
            },
            {
                label: "View Artist",
                icon: User,
                onclick: () => {
                    if (album.artist_name) {
                        libraryState.selectArtist(
                            album.artist_name.split(";")[0].trim(),
                        );
                    }
                },
            },
            {
                label: "Refetch Metadata",
                icon: RefreshCw,
                onclick: async () => {
                    await TauriService.fetchAlbumMetadata(album.id, "auto", true);
                    libraryState.refresh();
                },
            },
        ]);
    }
</script>

<button
    onclick={() => onclick(album.id)}
    oncontextmenu={handleContextMenu}
    class="flex flex-col gap-2 p-3 rounded-xl bg-surface-base border border-transparent hover:bg-surface-hover hover:border-surface-border transition group text-left relative"
>
    <div class="aspect-square w-full relative overflow-hidden rounded-lg">
        <CoverArt
            imageHash={album.cover_hash}
            alt={album.name}
            class="h-full w-full object-cover group-hover:scale-105 transition-transform duration-500"
            size={256}
            placeholderText="No Cover"
        />

        {#if isFeatured}
            <div class="absolute top-2 right-2">
                <span
                    class="text-[10px] px-1.5 py-0.5 rounded bg-black/60 backdrop-blur-md text-white border border-white/10 uppercase font-bold tracking-tighter"
                >
                    Featured
                </span>
            </div>
        {/if}
    </div>

    <div class="min-w-0">
        <h3
            class="font-bold text-sm truncate text-white group-hover:text-indigo-300 transition-colors"
        >
            {album.name}
        </h3>
        <p class="text-[10px] text-gray-500 mt-0.5 flex items-center gap-1.5">
            <span
                >{album.track_count}
                {album.track_count === 1 ? "track" : "tracks"}</span
            >
            {#if album.release_date}
                <span class="opacity-50">•</span>
                <span>{formatDate(album.release_date)}</span>
            {/if}
        </p>
    </div>
</button>
