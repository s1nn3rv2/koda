<script lang="ts">
    import { LoaderCircle } from "@lucide/svelte";
    import { uiState } from "$lib/state/player.svelte";
</script>

<div
    class="flex h-full w-full flex-col items-center justify-center gap-2 text-xs text-white/40 transition-all duration-300 {uiState.isExpanded
        ? 'px-8 py-4'
        : 'px-4 py-2'}"
>
    <span class="text-center text-xs">Generating waveform...</span>
</div>
