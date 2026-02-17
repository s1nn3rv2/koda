<script lang="ts">
    import { VolumeX, Volume, Volume1, Volume2 } from "@lucide/svelte";
    import { playerState } from "$lib/state/player.svelte";

    $effect(() => {
        playerState.volume;
        playerState.setVolume();
    });

    const VolumeIcon = $derived.by(() => {
        if (playerState.volume === 0) return VolumeX;
        else if (playerState.volume < 0.3) return Volume;
        else if (playerState.volume < 0.7) return Volume1;
        else return Volume2;
    });
</script>

<div class="flex items-center gap-2">
    <VolumeIcon size={16} class="text-white/60" />
    <input
        type="range"
        min="0"
        max="1"
        step="0.01"
        bind:value={playerState.volume}
        class="h-1 w-20 cursor-pointer appearance-none rounded-lg bg-white/20 accent-indigo-300"
    />
</div>
