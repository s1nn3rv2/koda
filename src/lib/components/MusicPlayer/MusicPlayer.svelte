<script lang="ts">
    import { playbackState, uiState } from "$lib/state/player.svelte";
    import { lyricsState } from "$lib/state/lyrics.svelte";
    import TrackInfo from "./TrackInfo.svelte";
    import Controls from "./Controls.svelte";
    import Volume from "./Volume.svelte";
    import ShuffleControl from "./ShuffleControl.svelte";
    import LyricsView from "./LyricsView.svelte";
    import Tooltip from "../Tooltip.svelte";
    import { MicVocal } from "@lucide/svelte";

    const defaultColor = "99, 102, 241";

    const extractRgb = (colorString: string | null) => {
        if (!colorString) return defaultColor;
        const rgb = colorString.match(/\d+/g)?.map(Number);
        if (!rgb || rgb.length < 3) return defaultColor;
        return `${rgb[0]}, ${rgb[1]}, ${rgb[2]}`;
    };

    const currentColor = $derived(extractRgb(playbackState.dominantColor));
</script>

<footer
    class="fixed bottom-0 left-0 right-0 z-50 border-t border-white/8 overflow-hidden shadow-[0_-8px_40px_rgba(0,0,0,0.6)]"
    style="background: rgba(12, 12, 15, 0.98); -webkit-backdrop-filter: blur(12px); backdrop-filter: blur(12px); tra"
>
    <div
        class="absolute inset-0 z-0 pointer-events-none transition-colors duration-1000 ease-in-out"
        style="background-color: rgba({currentColor}, 0.15);"
    >
        <div
            class="absolute w-[150%] h-[150%] -left-[25%] -top-[25%] transition-colors duration-1000 ease-in-out"
            style="background-color: rgba({currentColor}, 0.35); mask-image: radial-gradient(circle at 50% 80%, black 0%, transparent 60%); -webkit-mask-image: radial-gradient(circle at 50% 80%, black 0%, transparent 60%); mix-blend-mode: screen;"
        ></div>

        <div
            class="absolute inset-0 opacity-3 z-10"
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
            <div class="flex items-center gap-1">
                <button
                    class="group relative flex items-center justify-center rounded-full p-2 transition-all duration-300
                    {uiState.showLyrics
                        ? 'text-indigo-400 bg-indigo-500/10'
                        : 'text-text-secondary hover:bg-surface-hover hover:text-text-primary'}"
                    onclick={() => uiState.toggleLyrics()}
                    aria-label="Toggle Lyrics"
                >
                    <MicVocal size={16} />

                    {#if lyricsState.syncedLyrics}
                        <span
                            class="absolute top-0 right-0 w-1.5 h-1.5 rounded-full bg-indigo-400 shadow-[0_0_8px_rgba(129,140,248,0.5)]"
                        ></span>
                    {/if}

                    <Tooltip
                        text={uiState.showLyrics
                            ? "Hide Lyrics"
                            : "Show Lyrics"}
                    />
                </button>
            </div>
            <Volume />
        </div>
    </div>
</footer>

<LyricsView />
