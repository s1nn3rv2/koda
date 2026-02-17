<script lang="ts">
    import { Maximize2, Minimize2 } from "@lucide/svelte";
    import { playerState, uiState } from "$lib/state/player.svelte";
    import PlayerCoverArt from "./PlayerCoverArt.svelte";

    const displayTitle = $derived(
        playerState.currentTrack?.title || playerState.trackName,
    );
    const displayArtist = $derived(
        playerState.currentTrack?.artists || playerState.artistName,
    );
    const trackId = $derived(playerState.currentTrack?.id);
    const ExpandIcon = $derived(uiState.isExpanded ? Minimize2 : Maximize2);
</script>

<div class="min-w-0 flex items-center gap-3">
    <div
        class="relative shrink-0 rounded-md ring-1 ring-white/15 overflow-hidden group transition-all duration-300 {uiState.isExpanded
            ? 'h-24 w-24'
            : 'h-12 w-12'}"
    >
        {#if trackId}
            {#key trackId}
                <PlayerCoverArt
                    {trackId}
                    alt={displayTitle}
                    class="rounded-md {uiState.isExpanded
                        ? 'h-24 w-24'
                        : 'h-12 w-12'}"
                    isExpanded={uiState.isExpanded}
                />
                <button
                    onclick={() => uiState.toggleExpanded()}
                    class="absolute right-1 top-1 flex items-center justify-center bg-black/75 opacity-0 group-hover:opacity-100 transition-opacity p-0.5 rounded-md cursor-pointer"
                    title="Expand"
                >
                    <ExpandIcon size={16} class="text-white" />
                </button>
            {/key}
        {:else}
            <div class="h-full w-full bg-white/10"></div>
        {/if}
    </div>
    <div class="min-w-0 {uiState.isExpanded ? 'mt-4' : ''}">
        <h1
            class="truncate font-semibold text-white {uiState.isExpanded
                ? 'text-2xl'
                : 'text-sm'}"
        >
            {displayTitle}
        </h1>
        <p
            class="truncate text-white/60 {uiState.isExpanded
                ? 'text-lg mt-1'
                : 'text-xs'}"
        >
            {displayArtist}
        </p>
    </div>
</div>
