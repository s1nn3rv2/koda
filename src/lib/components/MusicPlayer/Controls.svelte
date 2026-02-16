<script lang="ts">
    import { playerState } from "$lib/state/player.svelte";
    import { SkipBack, SkipForward } from "@lucide/svelte";
    import PlayPauseButton from "./PlayPauseButton.svelte";
</script>

<div class="flex flex-col items-center">
    <div class="flex items-center gap-2">
        <button
            class="rounded-full p-2 text-white/70 transition hover:bg-white/10 hover:text-white"
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
            class="rounded-full p-2 text-white/70 transition hover:bg-white/10 hover:text-white"
            onclick={() => playerState.nextTrack()}
            aria-label="Next track"
        >
            <SkipForward size={16} fill="currentColor" />
        </button>
    </div>

    {#if playerState.errorMsg}
        <p class="absolute mt-14 text-sm text-red-400">{playerState.errorMsg}</p>
    {/if}
</div>
