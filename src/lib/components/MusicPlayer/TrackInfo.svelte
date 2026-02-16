<script lang="ts">
    import { playerState } from "$lib/state/player.svelte";
</script>

<div class="min-w-0 flex items-center gap-3">
    <div
        class="h-12 w-12 shrink-0 rounded-md bg-white/10 ring-1 ring-white/15"
    ></div>
    <div class="min-w-0">
        <h1 class="truncate text-sm font-semibold text-white">
            {playerState.trackName}
        </h1>
        <p class="truncate text-xs text-white/60">{playerState.artistName}</p>
    </div>
</div>
