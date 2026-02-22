<script lang="ts">
    import type { AlbumWithCount } from "$lib/types";
    import { pluralize } from "$lib/utils/format";
    import CoverArt from "$lib/components/CoverArt.svelte";
    import { Play } from "@lucide/svelte";
    import {
        contextMenuState,
        type ContextMenuItem,
    } from "$lib/state/context_menu.svelte";

    interface Props {
        title: string;
        subtitle?: string | null;
        subtitleSnippet?: import("svelte").Snippet;
        totalTracks: number;
        contentAlbum: AlbumWithCount | null;
        imageHash?: string | null;
        roundImage?: boolean;
        isLoading?: boolean;
        coverMenuItems?: ContextMenuItem[];
        showCoverArt?: boolean;
        releaseDate?: string | null;
        onplay?: () => void;
        actions?: import("svelte").Snippet;
    }

    let {
        title,
        subtitle,
        subtitleSnippet,
        totalTracks,
        contentAlbum,
        imageHash,
        roundImage = false,
        isLoading = false,
        coverMenuItems = [],
        showCoverArt = true,
        releaseDate,
        onplay,
        actions,
    }: Props = $props();

    function handleContextMenu(e: MouseEvent) {
        if (coverMenuItems.length > 0) {
            e.preventDefault();
            contextMenuState.open(e.clientX, e.clientY, coverMenuItems);
        }
    }
</script>

<div class="mb-8 flex items-center gap-6">
    {#if showCoverArt}
        {#if isLoading}
            <div
                class="h-64 w-64 bg-white/5 rounded-xl flex flex-col items-center justify-center border border-white/10 shrink-0 shadow-2xl animate-pulse"
            >
                <div
                    class="w-6 h-6 border-2 border-indigo-500 border-t-transparent rounded-full animate-spin mb-3"
                ></div>
                <span
                    class="text-[9px] uppercase font-bold tracking-widest text-indigo-400/60"
                    >Fetching Art</span
                >
            </div>
        {:else if contentAlbum?.cover_hash}
            <CoverArt
                imageHash={contentAlbum.cover_hash}
                alt={contentAlbum.name}
                class="h-64 w-64 rounded-xl shrink-0 shadow-2xl"
                size={256}
                oncontextmenu={handleContextMenu}
            />
        {:else if imageHash}
            <CoverArt
                {imageHash}
                alt={title}
                class="h-64 w-64 shrink-0 shadow-2xl {roundImage
                    ? 'rounded-full'
                    : 'rounded-xl'}"
                size={256}
                oncontextmenu={handleContextMenu}
            />
        {:else}
            <CoverArt
                imageHash={contentAlbum?.cover_hash || imageHash}
                alt={title}
                class="h-64 w-64 shrink-0 shadow-2xl {roundImage
                    ? 'rounded-full'
                    : 'rounded-xl'}"
                size={256}
                oncontextmenu={handleContextMenu}
                placeholderText="No Cover"
            />
        {/if}
    {/if}
    <div class="flex-1 min-w-0">
        {#if subtitleSnippet}
            <div
                class="text-sm font-bold uppercase tracking-wider text-gray-400 mb-2"
            >
                {@render subtitleSnippet()}
            </div>
        {:else if subtitle}
            <p
                class="text-sm font-bold uppercase tracking-wider text-gray-400 mb-2"
            >
                {subtitle}
            </p>
        {/if}
        <h2
            class="{contentAlbum
                ? 'text-6xl'
                : 'text-4xl'} font-black truncate mb-2 leading-tight"
        >
            {title}
        </h2>
        <p class="text-sm text-gray-500 font-medium">
            {totalTracks}
            {pluralize(totalTracks, "track")}
        </p>
        {#if releaseDate}
            <p class="text-xs text-gray-500 font-medium mt-1">
                Released: {releaseDate}
            </p>
        {/if}

        {#if onplay}
            <button
                class="mt-4 px-6 py-2.5 bg-indigo-500 hover:bg-indigo-400 text-white font-bold rounded-full transition-colors flex items-center gap-2 shadow-lg shadow-indigo-500/20"
                onclick={onplay}
            >
                <Play size={18} fill="currentColor" />
                Play All
            </button>
        {/if}
    </div>

    {#if actions}
        <div class="ml-auto mb-2">
            {@render actions()}
        </div>
    {/if}
</div>
