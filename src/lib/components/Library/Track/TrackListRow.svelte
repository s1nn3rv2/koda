<script lang="ts">
    import { X, Plus } from "@lucide/svelte";
    import CoverArt from "$lib/components/CoverArt.svelte";
    import PlayPauseButton from "$lib/components/MusicPlayer/PlayPauseButton.svelte";
    import { formatDuration, getFileName, formatDate } from "$lib/utils/format";
    import { isTrackActive, isTrackPlaying } from "$lib/utils/trackState";
    import { playbackState } from "$lib/state/playback.svelte";
    import { queueState } from "$lib/state/queue.svelte";
    import type { Track } from "$lib/types";

    interface TrackProps {
        track: Track;
        originalIndex?: number;
        isAlbumView?: boolean;
        isQueueView?: boolean;
        artistsList: string[];
        handleArtistClick: (artist: string, e: MouseEvent) => void;
        handleAlbumClick: (albumId: string, e: MouseEvent) => void;
        playTrack: (track: Track) => void;
        onremovefromqueue?: () => void;
    }

    let {
        track,
        originalIndex,
        isAlbumView = false,
        isQueueView = false,
        artistsList,
        handleArtistClick,
        handleAlbumClick,
        playTrack,
        onremovefromqueue,
    }: TrackProps = $props();

    const isActive = $derived(
        isTrackActive(track.id, isQueueView, originalIndex),
    );
    const isActuallyPlaying = $derived(
        isTrackPlaying(track.id, isQueueView, originalIndex),
    );
</script>

<div class="flex items-center gap-4 min-w-0 flex-1">
    <div class="w-8 h-8 flex items-center justify-center relative shrink-0">
        {#if isActuallyPlaying}
            <div
                class="absolute inset-0 flex items-center justify-center text-brand-secondary"
            >
                <PlayPauseButton
                    isPlaying={true}
                    class="w-8 h-8 bg-transparent text-brand-secondary"
                    onclick={(e) => {
                        e.stopPropagation();
                        playbackState.togglePlayPause();
                    }}
                />
            </div>
        {:else}
            <span
                class="absolute inset-0 flex items-center justify-center text-text-dim font-mono text-xs group-hover:hidden"
            >
                {#if !isQueueView}
                    {#if track.disc_number}
                        {track.disc_number}.{track.track_number || "--"}
                    {:else if track.track_number}
                        {track.track_number}
                    {:else}
                        --
                    {/if}
                {/if}
            </span>
            <div
                class="hidden group-hover:flex absolute inset-0 items-center justify-center"
            >
                <PlayPauseButton
                    isPlaying={false}
                    class="w-8 h-8 bg-transparent text-white"
                    onclick={(e) => {
                        e.stopPropagation();
                        if (isActive) {
                            playbackState.togglePlayPause();
                        } else {
                            playTrack(track);
                        }
                    }}
                />
            </div>
        {/if}
    </div>

    <div class="h-10 w-10 shrink-0">
        <CoverArt
            imageHash={track.cover_hash}
            alt={track.title || "Track cover"}
            class="h-full w-full rounded object-cover"
            size={128}
        />
    </div>

    <div class="min-w-0 flex-1">
        <div class="flex items-center gap-2">
            <p
                class="text-sm font-medium truncate {isActuallyPlaying
                    ? 'text-brand-secondary'
                    : 'text-text-primary'} group-hover:text-brand-secondary transition-colors duration-200"
            >
                {track.title || getFileName(track.path)}
            </p>
        </div>
        {#if track.artists}
            <p class="text-xs text-text-dim truncate">
                {#each artistsList as artist, i}
                    {#if i > 0}<span class="text-text-dim">, </span>{/if}
                    <button
                        type="button"
                        class="hover:text-text-primary hover:underline cursor-pointer text-left focus:outline-none"
                        onclick={(e) => {
                            e.stopPropagation();
                            handleArtistClick(artist, e);
                        }}>{artist}</button
                    >
                {/each}
            </p>
        {/if}
    </div>
</div>

<div class="hidden md:flex items-center gap-8 shrink-0">
    <div class="w-48 shrink-0">
        {#if !isAlbumView && track.album}
            <button
                type="button"
                class="text-xs text-text-dim truncate hover:text-text-primary hover:underline cursor-pointer text-left focus:outline-none w-full"
                onclick={(e) => {
                    if (track.album_id) {
                        e.stopPropagation();
                        handleAlbumClick(track.album_id, e);
                    }
                }}
            >
                {track.album}
            </button>
        {/if}
    </div>
    <div class="w-24 shrink-0 text-right">
        {#if track.release_date}
            <span class="text-xs text-text-secondary truncate">
                {formatDate(track.release_date)}
            </span>
        {/if}
    </div>
    <div class="w-12 shrink-0 text-right">
        {#if track.duration}
            <p class="text-xs text-text-dim font-medium">
                {formatDuration(track.duration)}
            </p>
        {/if}
    </div>
</div>

<div class="flex items-center gap-2 ml-4">
    {#if isQueueView}
        <button
            onclick={(e) => {
                e.stopPropagation();
                onremovefromqueue?.();
            }}
            class="opacity-0 group-hover:opacity-100 text-gray-500 hover:text-red-400 p-1.5 transition-all"
            title="Remove from queue"
        >
            <X size={16} />
        </button>
    {:else}
        <button
            class="opacity-0 group-hover:opacity-100 p-1.5 text-gray-400 hover:text-white transition-all"
            onclick={(e) => {
                e.stopPropagation();
                queueState.addToQueue(track);
            }}
            title="Add to queue"
        >
            <Plus size={16} />
        </button>
    {/if}
</div>
