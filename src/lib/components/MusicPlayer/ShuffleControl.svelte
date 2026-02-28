<script lang="ts">
    import { Shuffle } from "@lucide/svelte";
    import { queueState, uiState } from "$lib/state/player.svelte";
    import Tooltip from "../Tooltip.svelte";

    const isSmart = $derived(queueState.shuffleMode === "smart");
    const isCategory = $derived(queueState.shuffleMode === "category");
    const isRandom = $derived(queueState.shuffleMode === "random");

    const modeText = $derived.by(() => {
        if (isSmart) return "Smart (Sequential)";
        if (isCategory) return "Category (Artist/Album)";
        return "Global (All Tracks)";
    });

    const activeClass = $derived.by(() => {
        if (isSmart)
            return "text-text-secondary hover:bg-surface-hover hover:text-text-primary";
        if (isCategory) return "text-amber-400 bg-amber-400/10";
        return "text-brand-secondary bg-brand-secondary/10";
    });

    const dotClass = $derived.by(() => {
        if (isCategory)
            return "bg-amber-400 shadow-[0_0_8px_rgba(251,191,36,0.5)]";
        return "bg-brand-secondary shadow-[0_0_8px_rgba(var(--brand-secondary-rgb),0.5)]";
    });
</script>

<div class="flex items-center gap-1">
    <button
        class="group relative flex items-center justify-center rounded-full p-1.5 transition-all duration-300 {activeClass}"
        onclick={() => queueState.toggleShuffleMode()}
        aria-label="Toggle Shuffle Mode"
    >
        <Shuffle size={16} />

        {#if !isSmart}
            <span
                class="absolute -top-0.5 -right-0.5 w-1.5 h-1.5 rounded-full {dotClass}"
            >
                {#if isRandom}
                    <span
                        class="absolute inset-0 animate-ping bg-brand-secondary opacity-75 rounded-full"
                    ></span>
                {/if}
            </span>
        {/if}

        <Tooltip text={`Shuffle: ${modeText}`} />
    </button>
</div>
