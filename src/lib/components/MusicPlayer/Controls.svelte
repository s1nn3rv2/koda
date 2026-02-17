<script lang="ts">
    import { playerState, uiState } from "$lib/state/player.svelte";
    import { SkipBack, SkipForward } from "@lucide/svelte";
    import PlayPauseButton from "./PlayPauseButton.svelte";
    import WaveformSeekbar from "$lib/components/Waveform/WaveformSeekbar.svelte";

    function handleSeek(time: number) {
        playerState.seek(time);
    }
</script>

<div
    class="flex w-full transition-all duration-300 {uiState.isExpanded
        ? 'flex-col items-center gap-2'
        : 'items-center gap-3'}"
>
    <div
        class="flex items-center gap-2 transition-all duration-300 {uiState.isExpanded
            ? 'justify-center'
            : 'flex-shrink-0'}"
    >
        <button
            class="rounded-full p-1.5 text-white/70 transition hover:bg-white/10 hover:text-white"
            onclick={() => playerState.prevTrack()}
            aria-label="Previous track"
        >
            <SkipBack size={16} fill="currentColor" />
        </button>

        <PlayPauseButton
            isPlaying={playerState.isActuallyPlaying}
            onclick={() => playerState.togglePlayPause()}
        />

        <button
            class="rounded-full p-1.5 text-white/70 transition hover:bg-white/10 hover:text-white"
            onclick={() => playerState.nextTrack()}
            aria-label="Next track"
        >
            <SkipForward size={16} fill="currentColor" />
        </button>
    </div>

    <div
        class="flex items-center transition-all duration-300 {uiState.isExpanded
            ? 'w-full'
            : 'flex-1'}"
    >
        <WaveformSeekbar
            trackId={playerState.currentTrackId}
            currentTime={playerState.currentPosition}
            duration={playerState.currentTrack?.duration || 0}
            onSeek={handleSeek}
        />
    </div>

    {#if playerState.errorMsg}
        <p class="absolute mt-14 text-sm text-red-400">
            {playerState.errorMsg}
        </p>
    {/if}
</div>
