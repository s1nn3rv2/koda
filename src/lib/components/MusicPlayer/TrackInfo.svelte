<script lang="ts">
    import { Maximize2, Minimize2 } from "@lucide/svelte";
    import { goto } from "$app/navigation";
    import { playbackState, uiState } from "$lib/state/player.svelte";
    import { libraryState } from "$lib/state/library.svelte";
    import PlayerCoverArt from "./PlayerCoverArt.svelte";
    import Artists from "../Artists.svelte";
    import Album from "../Album.svelte";
    import ScrollingText from "../ScrollingText.svelte";

    const displayTitle = $derived(playbackState.currentTrack?.title || "");
    const displayArtist = $derived(playbackState.currentTrack?.artists || "");
    const displayAlbum = $derived(playbackState.currentTrack?.album);
    const trackId = $derived(playbackState.currentTrack?.id);
    const ExpandIcon = $derived(uiState.isExpanded ? Minimize2 : Maximize2);

    function handleArtistClick(
        artist: string,
        event: MouseEvent | KeyboardEvent,
    ) {
        event.stopPropagation();
        event.preventDefault();
        libraryState.selectArtist(artist);
        if (uiState.isExpanded) {
            uiState.toggleExpanded();
        }
        goto("/library");
    }

    async function handleAlbumClick(
        albumId: string,
        event: MouseEvent | KeyboardEvent,
    ) {
        event.stopPropagation();
        event.preventDefault();
        await libraryState.selectAlbum(albumId);
        goto("/library");
    }
</script>

<div class="min-w-0 flex items-center gap-3 w-full">
    <div
        class="relative shrink-0 rounded-md ring-1 ring-white/10 overflow-hidden group transition-all duration-300 {uiState.isExpanded
            ? 'h-36 w-36'
            : 'h-12 w-12'}"
    >
        {#if trackId}
            {#key trackId}
                <PlayerCoverArt
                    {trackId}
                    imageHash={playbackState.currentTrack?.cover_hash}
                    alt={displayTitle}
                    class="rounded-md object-cover h-full w-full"
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
            <div class="h-full w-full bg-surface-hover"></div>
        {/if}
    </div>
    <div class="min-w-0 flex flex-col justify-center flex-1 overflow-hidden">
        <div
            class="font-semibold text-text-primary {uiState.isExpanded
                ? 'text-2xl'
                : 'text-sm'}"
        >
            {#key displayTitle + uiState.isExpanded}
                <ScrollingText>
                    {displayTitle}
                </ScrollingText>
            {/key}
        </div>
        <div
            class="text-text-secondary {uiState.isExpanded
                ? 'text-lg mt-1'
                : 'text-xs'}"
        >
            {#if playbackState.currentTrack?.artists}
                {#key displayArtist + uiState.isExpanded}
                    <ScrollingText>
                        <Artists
                            artists={playbackState.currentTrack.artists}
                            {handleArtistClick}
                        />
                    </ScrollingText>
                {/key}
            {/if}
        </div>
        {#if playbackState.currentTrack?.album}
            <div
                class="text-text-secondary {uiState.isExpanded
                    ? 'text-lg'
                    : 'text-xs'}"
            >
                {#if playbackState.currentTrack.album_id}
                    {#key (displayAlbum ?? "") + uiState.isExpanded}
                        <ScrollingText>
                            <Album
                                albumName={playbackState.currentTrack.album}
                                albumId={playbackState.currentTrack.album_id}
                                {handleAlbumClick}
                            />
                        </ScrollingText>
                    {/key}
                {/if}
            </div>
        {/if}
    </div>
</div>
