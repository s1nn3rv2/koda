<script lang="ts">
    import { playbackState, uiState } from "$lib/state/player.svelte";
    import TrackInfo from "./TrackInfo.svelte";
    import Controls from "./Controls.svelte";
    import Volume from "./Volume.svelte";
    import ShuffleControl from "./ShuffleControl.svelte";

    function createBloom(color: string) {
        const rgb = color.match(/\d+/g)?.map(Number);
        if (!rgb || rgb.length < 3) return "";
        const [r, g, b] = rgb;

        return `radial-gradient(circle at 50% 120%, rgba(${r}, ${g}, ${b}, 0.3) 0%, rgba(${r}, ${g}, ${b}, 0.05) 60%, transparent 100%)`;
    }

    const defaultBloom =
        "radial-gradient(circle at 50% 120%, rgba(99, 102, 241, 0.15) 0%, rgba(99, 102, 241, 0.03) 60%, transparent 100%)";

    let layerABloom = $state(defaultBloom);
    let layerBBloom = $state(defaultBloom);
    let activeLayer = $state<"A" | "B">("A");

    $effect(() => {
        const newColor = playbackState.dominantColor;
        const targetBloom = newColor ? createBloom(newColor) : defaultBloom;

        if (activeLayer === "A" && targetBloom !== layerABloom) {
            layerBBloom = targetBloom;
            requestAnimationFrame(() => {
                requestAnimationFrame(() => {
                    activeLayer = "B";
                });
            });
        } else if (activeLayer === "B" && targetBloom !== layerBBloom) {
            layerABloom = targetBloom;
            requestAnimationFrame(() => {
                requestAnimationFrame(() => {
                    activeLayer = "A";
                });
            });
        }
    });
</script>

<footer
    class="fixed bottom-0 left-0 right-0 z-50 border-t border-white/[0.08] overflow-hidden shadow-[0_-8px_40px_rgba(0,0,0,0.6)]"
    style="background: rgba(12, 12, 15, 0.98); -webkit-backdrop-filter: blur(12px); backdrop-filter: blur(12px); tra"
>
    <div class="absolute inset-0 z-0 pointer-events-none">
        <!-- bloom 1 -->
        <div
            class="absolute inset-0 transition-opacity duration-1000 ease-in-out"
            style="background: {layerABloom}; opacity: {activeLayer === 'A'
                ? 1
                : 0};"
        ></div>

        <!-- bloom 2 -->
        <div
            class="absolute inset-0 transition-opacity duration-1000 ease-in-out"
            style="background: {layerBBloom}; opacity: {activeLayer === 'B'
                ? 1
                : 0};"
        ></div>

        <div
            class="absolute inset-0 opacity-[0.03] z-10"
            style="background-image: url('data:image/svg+xml,%3Csvg viewBox=%220 0 200 200%22 xmlns=%22http://www.w3.org/2000/svg%22%3E%3Cfilter id=%22noise%22%3E%3CfeTurbulence type=%22fractalNoise%22 baseFrequency=%220.65%22 numOctaves=%223%22 stitchTiles=%22stitch%22/%3E%3C/filter%3E%3Crect width=%22100%25%22 height=%22100%25%22 filter=%22url(%23noise)%22/%3E%3C/svg%3E');"
        ></div>

        <div
            class="absolute inset-0 bg-gradient-to-t from-black/20 via-transparent to-white/5 opacity-50"
        ></div>
    </div>

    <div
        class="relative z-20 flex items-center gap-2 px-2 py-3 sm:gap-4 sm:px-4 md:px-6 transition-all duration-300 {uiState.isExpanded
            ? 'min-h-[160px] flex-col sm:flex-row'
            : 'min-h-[80px] flex-row'}"
    >
        <div
            class="flex items-center gap-3 min-w-0 transition-all duration-300 w-full sm:w-auto sm:flex-1"
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
            class="hidden sm:flex items-center justify-end flex-shrink-0 transition-all duration-300 sm:w-auto sm:flex-1 gap-4"
        >
            <ShuffleControl />
            <Volume />
        </div>
    </div>
</footer>
