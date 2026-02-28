<script lang="ts">
    import { uiState, playbackState } from "$lib/state/player.svelte";
    import { lyricsState } from "$lib/state/lyrics.svelte";
    import { X, MicVocal, CircleAlert, LoaderCircle } from "@lucide/svelte";
    import { fade, slide } from "svelte/transition";
    import { onMount } from "svelte";

    let lyricsContainer = $state<HTMLElement>();
    let autoScroll = $state(true);
    let isScroll = false;
    let scrollTimeout: ReturnType<typeof setTimeout>;
    let lastActiveIndex = -1;
    let targetScrollPos = 0;

    $effect(() => {
        if (
            autoScroll &&
            lyricsState.syncedLyrics &&
            playbackState.currentPosition &&
            lyricsContainer
        ) {
            const lines = lyricsContainer.querySelectorAll(".lyric-line");
            const activeIndex = lyricsState.syncedLyrics.findIndex((l, i) => {
                const next = lyricsState.syncedLyrics![i + 1];
                return (
                    playbackState.currentPosition >= l.time &&
                    (!next || playbackState.currentPosition < next.time)
                );
            });

            if (
                activeIndex !== -1 &&
                lines[activeIndex] &&
                activeIndex !== lastActiveIndex
            ) {
                lastActiveIndex = activeIndex;

                const activeEl = lines[activeIndex] as HTMLElement;
                const containerHeight = lyricsContainer.clientHeight;
                targetScrollPos =
                    activeEl.offsetTop -
                    containerHeight / 2 +
                    activeEl.clientHeight / 2;

                isScroll = true;
                lyricsContainer.scrollTo({
                    top: targetScrollPos,
                    behavior: "smooth",
                });

                clearTimeout(scrollTimeout);
                scrollTimeout = setTimeout(() => {
                    isScroll = false;
                }, 800);
            }
        }
    });

    $effect(() => {
        if (uiState.showLyrics) {
            autoScroll = true;
            lastActiveIndex = -1;
        }
    });

    function handleScroll() {
        if (!isScroll && lyricsContainer) {
            const currentScroll = lyricsContainer.scrollTop;

            //snap back to autoscroll
            if (Math.abs(currentScroll - targetScrollPos) < 50) {
                autoScroll = true;
            } else {
                autoScroll = false;
            }
        }
    }

    function seekTo(time: number) {
        playbackState.seek(time);
        autoScroll = true;
        lastActiveIndex = -1;
    }
</script>

{#if uiState.showLyrics}
    <div
        class="fixed inset-0 z-40 bg-black/97 backdrop-blur-2xl flex flex-col pt-12 pb-24 md:pb-32 px-4 md:px-12"
        transition:fade={{ duration: 300 }}
    >
        <div class="flex justify-between items-center mb-8 shrink-0">
            <button
                onclick={() => uiState.toggleLyrics()}
                class="w-10 h-10 ml-auto rounded-full bg-white/5 hover:bg-white/10 flex items-center justify-center text-white transition-colors"
            >
                <X size={20} />
            </button>
        </div>

        <div
            class="flex-1 overflow-y-auto no-scrollbar relative w-full h-full pb-32"
            bind:this={lyricsContainer}
            onscroll={handleScroll}
        >
            {#if lyricsState.isLoading}
                <div
                    class="absolute inset-0 flex flex-col items-center justify-center text-gray-400"
                >
                    <LoaderCircle
                        size={32}
                        class="animate-spin mb-4 text-indigo-400"
                    />
                    <p>Fetching lyrics...</p>
                </div>
            {:else if lyricsState.error}
                <div
                    class="absolute inset-0 flex flex-col items-center justify-center text-gray-500"
                >
                    <CircleAlert size={48} class="mb-4 opacity-50" />
                    <p class="text-lg font-medium">{lyricsState.error}</p>
                    <p class="text-sm mt-2 opacity-60">
                        Couldn't find any lyrics for this track on LRCLIB.
                    </p>
                </div>
            {:else if lyricsState.syncedLyrics}
                <div class="max-w-3xl mx-auto space-y-6 pt-12 pb-[50vh]">
                    {#each lyricsState.syncedLyrics as line, i}
                        {@const nextTime =
                            lyricsState.syncedLyrics[i + 1]?.time || Infinity}
                        {@const isActive =
                            playbackState.currentPosition >= line.time &&
                            playbackState.currentPosition < nextTime}
                        {@const isPassed =
                            playbackState.currentPosition >= nextTime}
                        {@const duration = nextTime - line.time}
                        {@const progress = isActive
                            ? Math.max(
                                  0,
                                  Math.min(
                                      1,
                                      (playbackState.currentPosition -
                                          line.time) /
                                          Math.min(duration, 5),
                                  ),
                              )
                            : isPassed
                              ? 1
                              : 0}

                        <button
                            class="lyric-line block w-full text-left transition-all duration-300 transform-gpu cursor-pointer group"
                            onclick={() => seekTo(line.time)}
                        >
                            <span
                                class="
                                inline-block text-2xl sm:text-3xl md:text-5xl font-black leading-tight transition-all duration-300
                                {isActive ? 'text-white' : ''}
                                {isPassed && !isActive ? 'text-white/30' : ''}
                                {!isActive && !isPassed
                                    ? 'text-white/50 group-hover:text-white/80'
                                    : ''}
                                "
                            >
                                {#if line.text}
                                    {#each line.text.split(" ") as word, wordIdx}
                                        {@const wordOffset =
                                            line.text
                                                .split(" ")
                                                .slice(0, wordIdx)
                                                .join(" ").length +
                                            (wordIdx > 0 ? 1 : 0)}
                                        <span
                                            class="inline-block whitespace-nowrap"
                                        >
                                            {#each word.split("") as char, charIdx}
                                                {@const charGlobalIdx =
                                                    wordOffset + charIdx}
                                                <span
                                                    class="inline-block"
                                                    style={isActive
                                                        ? `animation: lyric-wave 2s cubic-bezier(0.4, 0, 0.2, 1) infinite ${charGlobalIdx * 0.05}s; will-change: transform; backface-visibility: hidden; transform: translateZ(0);`
                                                        : ""}
                                                >
                                                    {char}
                                                </span>
                                            {/each}
                                            {#if wordIdx < line.text.split(" ").length - 1}
                                                <span
                                                    class="inline-block w-[0.25em]"
                                                ></span>
                                            {/if}
                                        </span>
                                    {/each}
                                {:else}<br />{/if}
                            </span>
                        </button>
                    {/each}
                </div>
            {:else}
                <div
                    class="absolute inset-0 flex flex-col items-center justify-center text-gray-500"
                >
                    <CircleAlert size={48} class="mb-4 opacity-50" />
                    <p>No lyrics found</p>
                </div>
            {/if}
        </div>

        {#if !autoScroll && lyricsState.syncedLyrics}
            <div
                class="absolute bottom-32 left-1/2 -translate-x-1/2"
                transition:slide={{ duration: 200, axis: "y" }}
            >
                <button
                    onclick={() => (autoScroll = true)}
                    class="px-6 py-2.5 rounded-full bg-indigo-500 hover:bg-indigo-400 backdrop-blur-xl border border-indigo-400/50 text-white font-bold text-sm flex items-center gap-2 shadow-[0_0_20px_rgba(99,102,241,0.3)] transition-all hover:scale-105 active:scale-95"
                >
                    Scroll back
                </button>
            </div>
        {/if}
    </div>
{/if}

<style>
    .no-scrollbar::-webkit-scrollbar {
        display: none;
    }
    .no-scrollbar {
        -ms-overflow-style: none;
        scrollbar-width: none;
    }

    @keyframes -global-lyric-wave {
        0%,
        100% {
            transform: translateY(0);
        }
        50% {
            transform: translateY(-2px);
        }
    }
</style>
