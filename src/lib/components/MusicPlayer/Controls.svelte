<script lang="ts">
    import {
        playbackState,
        queueState,
        uiState,
    } from "$lib/state/player.svelte";
    import { SkipBack, SkipForward } from "@lucide/svelte";
    import PlayPauseButton from "./PlayPauseButton.svelte";
    import WaveformSeekbar from "$lib/components/Waveform/WaveformSeekbar.svelte";

    function handleSeek(time: number) {
        playbackState.seek(time);
    }
</script>

<div
    class="flex w-full max-w-4xl mx-auto transition-all duration-300 {uiState.isExpanded
        ? 'flex-col sm:flex-row items-center gap-2 sm:gap-6'
        : 'items-center gap-3'}"
>
    <div
        class="flex items-center gap-2 transition-all duration-300 {uiState.isExpanded
            ? 'justify-center sm:flex-shrink-0'
            : 'flex-shrink-0'}"
    >
        <button
            class="rounded-full p-1.5 text-text-secondary transition hover:bg-surface-hover hover:text-text-primary"
            onclick={() => queueState.prevTrack()}
            aria-label="Previous track"
        >
            <SkipBack size={16} fill="currentColor" />
        </button>

        <PlayPauseButton
            isPlaying={playbackState.isActuallyPlaying}
            onclick={() => playbackState.togglePlayPause()}
        />

        <button
            class="rounded-full p-1.5 text-text-secondary transition hover:bg-surface-hover hover:text-text-primary"
            onclick={() => queueState.nextTrack()}
            aria-label="Next track"
        >
            <SkipForward size={16} fill="currentColor" />
        </button>
    </div>

    <div
        class="flex items-center transition-all duration-300 {uiState.isExpanded
            ? 'w-full sm:flex-1'
            : 'flex-1'}"
    >
        <WaveformSeekbar
            trackId={playbackState.currentTrackId}
            currentTime={playbackState.currentPosition}
            duration={playbackState.currentTrack?.duration || 0}
            onSeek={handleSeek}
        />
    </div>

    {#if playbackState.errorMsg}
        <p class="absolute mt-14 text-sm text-red-400">
            {playbackState.errorMsg}
        </p>
    {/if}
</div>
