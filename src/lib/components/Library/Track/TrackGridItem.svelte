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

<div class="relative shrink-0 w-full aspect-square mb-1">
    <CoverArt
        imageHash={track.cover_hash}
        alt={track.title || "Track cover"}
        class="h-full w-full rounded-lg object-cover"
        size={128}
    />

    <div
        class="absolute inset-0 transition-opacity duration-500 flex items-end justify-end gap-2 p-2 rounded-lg will-change-[transform,opacity] transform-gpu pointer-events-none {isActuallyPlaying
            ? 'opacity-100 bg-black/30'
            : 'opacity-0 group-hover:opacity-100 bg-black/20'}"
    >
        <button
            class="w-8 h-8 rounded-full bg-white/20 backdrop-blur-md text-white flex items-center justify-center hover:bg-white/30 hover:scale-110 transition-all pointer-events-auto"
            onclick={(e) => {
                e.stopPropagation();
                queueState.addToQueue(track);
            }}
            title="Add to queue"
        >
            <Plus size={16} />
        </button>
        <PlayPauseButton
            isPlaying={isActuallyPlaying}
            class="w-10 h-10 bg-brand-primary text-text-primary hover:bg-brand-secondary pointer-events-auto"
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
</div>

<div class="min-w-0 flex-1 flex flex-col">
    <div class="flex items-center gap-2 mb-0.5">
        <p
            class="font-bold text-base truncate {isActuallyPlaying
                ? 'text-brand-secondary'
                : 'text-text-primary'} group-hover:text-brand-secondary transition-colors duration-200"
        >
            {track.title || getFileName(track.path)}
        </p>
    </div>
    {#if track.artists}
        <p class="text-xs text-text-secondary truncate">
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

    <div class="flex items-center gap-2 mt-1 min-w-0">
        <div class="flex-1 min-w-0">
            {#if track.album}
                <button
                    type="button"
                    class="text-[11px] text-text-dim truncate hover:text-text-primary hover:underline cursor-pointer text-left focus:outline-none block w-full"
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
        <div class="shrink-0 flex items-center gap-2">
            {#if track.release_date}
                <span class="text-[11px] text-text-secondary font-mono">
                    {formatDate(track.release_date)}
                </span>
            {/if}
            {#if track.duration}
                <p class="text-[11px] text-text-dim font-medium">
                    {formatDuration(track.duration)}
                </p>
            {/if}
        </div>
    </div>
</div>

{#if isQueueView}
    <button
        onclick={(e) => {
            e.stopPropagation();
            onremovefromqueue?.();
        }}
        class="absolute top-2 right-2 opacity-0 group-hover:opacity-100 bg-red-500/10 text-red-400 p-1.5 rounded-lg text-[10px] border border-red-500/20 transition-opacity"
        title="Remove from queue"
    >
        <X size={14} fill="currentColor" />
    </button>
{/if}
