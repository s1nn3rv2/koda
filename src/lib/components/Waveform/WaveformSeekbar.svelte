<script lang="ts">
    import { uiState } from "$lib/state/player.svelte";
    import { formatTime } from "$lib/utils/format";
    import { useWaveformLoader } from "./useWaveformLoader.svelte";
    import { fade } from "svelte/transition";
    import LoadingState from "./LoadingState.svelte";
    import FallbackSeekbar from "./FallbackSeekbar.svelte";
    import WaveformVisualization from "./WaveformVisualization.svelte";

    interface Props {
        trackId: string | null | undefined;
        currentTime?: number;
        duration?: number;
        onSeek?: (time: number) => void;
    }

    let { trackId, currentTime = 0, duration = 0, onSeek }: Props = $props();

    const loader = useWaveformLoader(() => trackId);

    let containerElement = $state<HTMLDivElement>();
    let isHovering = $state(false);
    let hoverPosition = $state(0);

    const progress = $derived(
        duration > 0 ? (currentTime / duration) * 100 : 0,
    );

    function handleClick(event: MouseEvent) {
        if (!containerElement || !duration) return;

        const rect = containerElement.getBoundingClientRect();
        const x = event.clientX - rect.left;
        const percentage = x / rect.width;
        const seekTime = percentage * duration;

        if (onSeek) {
            onSeek(seekTime);
        }
    }

    function handleMouseMove(event: MouseEvent) {
        if (!containerElement) return;

        const rect = containerElement.getBoundingClientRect();
        const x = event.clientX - rect.left;
        hoverPosition = (x / rect.width) * 100;
    }

    function handleKeyDown(event: KeyboardEvent) {
        if (!containerElement || !duration) return;

        let seekTime = currentTime;
        const step = duration * 0.05; // 5% of duration per keystroke

        switch (event.key) {
            case "ArrowLeft":
                seekTime = Math.max(0, currentTime - step);
                event.preventDefault();
                break;
            case "ArrowRight":
                seekTime = Math.min(duration, currentTime + step);
                event.preventDefault();
                break;
            case "Home":
                seekTime = 0;
                event.preventDefault();
                break;
            case "End":
                seekTime = duration;
                event.preventDefault();
                break;
            default:
                return;
        }

        if (onSeek) {
            onSeek(seekTime);
        }
    }
</script>

<div class="flex justify-center w-full">
    <div class="relative transition-all duration-300 w-full">
        <div
            class="transition-all duration-300 {uiState.isExpanded
                ? 'h-12'
                : 'h-6'}"
        >
            <div
                bind:this={containerElement}
                class="relative h-full w-full cursor-pointer select-none overflow-hidden rounded bg-white/5 transition-all duration-300"
                onclick={handleClick}
                onkeydown={handleKeyDown}
                onmouseenter={() => (isHovering = true)}
                onmouseleave={() => (isHovering = false)}
                onmousemove={handleMouseMove}
                role="slider"
                tabindex="0"
                aria-label="Seek bar"
                aria-valuemin={0}
                aria-valuemax={duration}
                aria-valuenow={currentTime}
            >
                {#if loader.isLoading}
                    <!-- Fallback seekbar doubles as loading state with a subtle pulse, no need for separate loader -->
                    <div out:fade={{ duration: 400 }} class="h-full w-full">
                        <FallbackSeekbar {progress} />
                    </div>
                {:else if !loader.waveformData}
                    <div
                        in:fade={{ duration: 400 }}
                        out:fade={{ duration: 400 }}
                        class="h-full w-full"
                    >
                        <FallbackSeekbar {progress} />
                    </div>
                {:else}
                    <div
                        in:fade={{ duration: 600, delay: 100 }}
                        class="h-full w-full"
                    >
                        <WaveformVisualization
                            waveformData={loader.waveformData}
                            {progress}
                            {isHovering}
                            {hoverPosition}
                            {duration}
                        />
                    </div>
                {/if}
            </div>
        </div>

        <div
            class="absolute top-full mt-1 w-full flex justify-between text-[10px] text-white/40"
        >
            <span>{formatTime(currentTime)}</span>
            <span>{formatTime(duration)}</span>
        </div>
    </div>
</div>
