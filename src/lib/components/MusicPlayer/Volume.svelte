<script lang="ts">
    import { VolumeX, Volume, Volume1, Volume2 } from "@lucide/svelte";
    import { playbackState, uiState } from "$lib/state/player.svelte";

    $effect(() => {
        playbackState.volume;
        playbackState.syncVolume();
    });

    const VolumeIcon = $derived.by(() => {
        if (playbackState.isMuted || playbackState.volume === 0) return VolumeX;
        else if (playbackState.volume < 0.3) return Volume;
        else if (playbackState.volume < 0.7) return Volume1;
        else return Volume2;
    });

    function toggleMute() {
        playbackState.toggleMute();
    }
</script>

<div
    class="flex {uiState.isExpanded
        ? 'flex-col-reverse'
        : 'flex-row'} items-center gap-2 h-full"
>
    <button
        class="shrink-0 flex items-center justify-center {uiState.isExpanded
            ? 'h-8'
            : ''} hover:bg-surface-hover rounded-full p-1 transition-colors cursor-pointer"
        onclick={toggleMute}
        aria-label={playbackState.isMuted ? "Unmute" : "Mute"}
    >
        <VolumeIcon size={16} class="text-text-secondary" />
    </button>
    <div
        class="flex items-center justify-center {uiState.isExpanded
            ? 'h-32'
            : 'h-6'}"
    >
        <input
            type="range"
            min="0"
            max="1"
            step="0.01"
            bind:value={playbackState.volume}
            oninput={() => {
                if (playbackState.isMuted && playbackState.volume > 0) {
                    playbackState.isMuted = false;
                }
                playbackState.syncVolume();
            }}
            class="cursor-pointer appearance-none rounded-lg bg-white/10 accent-brand-secondary transition-all duration-300
            {uiState.isExpanded ? 'w-1 h-32' : 'h-1 w-20'}"
            style={uiState.isExpanded
                ? "-webkit-appearance: slider-vertical; width: 4px;"
                : "-webkit-appearance: slider-horizontal; height: 4px;"}
        />
    </div>
</div>
