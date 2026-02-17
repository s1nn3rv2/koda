<script>
    import { playerState, uiState } from "$lib/state/player.svelte";
    import TrackInfo from "./TrackInfo.svelte";
    import Controls from "./Controls.svelte";
    import Volume from "./Volume.svelte";
</script>

<footer
    class="fixed bottom-0 left-0 right-0 z-50
           border-t border-white/10
           bg-gradient-to-r from-slate-900/90 via-indigo-950/80 to-purple-950/80
           shadow-2xl transition-all duration-300"
>
    <div
        class="mx-auto flex max-w-7xl items-center gap-2 px-2 py-3 sm:gap-4 sm:px-4 md:px-6 transition-all duration-300 {uiState.isExpanded
            ? 'h-40 flex-col sm:flex-row'
            : 'h-20 flex-row'}"
    >
        <div
            class="flex items-center gap-3 min-w-0 transition-all duration-300 w-full sm:w-auto sm:flex-1 sm:max-w-xs lg:max-w-sm"
        >
            <TrackInfo />
        </div>

        <div
            class="flex items-center justify-center transition-all duration-300 w-full sm:flex-[2] order-first sm:order-none {uiState.isExpanded
                ? 'mb-2 sm:mb-0'
                : ''}"
        >
            <Controls />
        </div>

        <div
            class="hidden sm:flex items-center justify-end flex-shrink-0 transition-all duration-300 sm:w-auto sm:flex-1 sm:max-w-xs"
        >
            <Volume />
        </div>
    </div>
</footer>
