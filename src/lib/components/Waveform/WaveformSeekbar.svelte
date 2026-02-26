<script lang="ts">
    // TODO: fix this function mess
    import { playbackState, uiState } from "$lib/state/player.svelte";
    import { formatTime } from "$lib/utils/format";
    import { useWaveformLoader } from "./useWaveformLoader.svelte";
    import { fade } from "svelte/transition";
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

    let isDragging = $state(false);
    let isScrubbing = $state(false);
    let scrubTime = $state(0);
    let scrubTimeout: ReturnType<typeof setTimeout>;

    let isLocked = $state(false);
    let uiLockTimeout: ReturnType<typeof setTimeout>;

    const displayTime = $derived(
        isDragging || isScrubbing || isLocked ? scrubTime : currentTime,
    );

    function startSeeking() {
        if (!isDragging && !isScrubbing && !isLocked) {
            scrubTime = currentTime;
        }
        isLocked = true;
        clearTimeout(uiLockTimeout);
    }

    function stopSeeking(finalTime: number) {
        if (onSeek) onSeek(finalTime);

        isDragging = false;

        clearTimeout(uiLockTimeout);
        uiLockTimeout = setTimeout(() => {
            isLocked = false;
            isScrubbing = false;
        }, 500);
    }

    function applyScrubChange(newTime: number) {
        startSeeking();
        isScrubbing = true;
        scrubTime = Math.max(0, Math.min(duration, newTime));

        clearTimeout(scrubTimeout);
        scrubTimeout = setTimeout(() => {
            stopSeeking(scrubTime);
        }, 150);
    }

    const progress = $derived(
        duration > 0 ? (displayTime / duration) * 100 : 0,
    );

    function handlePointerDown(event: PointerEvent) {
        if (!containerElement || !duration || event.button !== 0) return;

        containerElement.setPointerCapture(event.pointerId);
        startSeeking();
        isDragging = true;

        if (scrubTimeout) {
            clearTimeout(scrubTimeout);
            isScrubbing = false;
        }

        const rect = containerElement.getBoundingClientRect();
        const percentage = Math.max(
            0,
            Math.min(1, (event.clientX - rect.left) / rect.width),
        );
        scrubTime = percentage * duration;
    }

    function handlePointerMove(event: PointerEvent) {
        if (!containerElement) return;

        const rect = containerElement.getBoundingClientRect();
        const x = event.clientX - rect.left;
        const percentage = Math.max(0, Math.min(1, x / rect.width));

        hoverPosition = percentage * 100;

        if (isDragging && duration) {
            scrubTime = percentage * duration;
        }
    }

    function handlePointerUp(event: PointerEvent) {
        if (!isDragging) return;

        containerElement?.releasePointerCapture(event.pointerId);
        stopSeeking(scrubTime);
    }

    function handleWheel(event: WheelEvent) {
        event.preventDefault();
        if (!duration) return;

        const timeChange = -(event.deltaY * 0.0002 * duration);
        applyScrubChange(
            (isScrubbing || isDragging || isLocked ? scrubTime : currentTime) +
                timeChange,
        );
    }

    function handleKeyDown(event: KeyboardEvent) {
        if (!containerElement || !duration) return;

        const stepMulti = event.repeat ? 0.005 : 0.05;
        const baseTime =
            isScrubbing || isDragging || isLocked ? scrubTime : currentTime;

        switch (event.key) {
            case "ArrowDown":
            case "ArrowLeft":
                event.preventDefault();
                applyScrubChange(baseTime - duration * stepMulti);
                break;
            case "ArrowUp":
            case "ArrowRight":
                event.preventDefault();
                applyScrubChange(baseTime + duration * stepMulti);
                break;
            case "Home":
                event.preventDefault();
                applyScrubChange(0);
                break;
            case "End":
                event.preventDefault();
                applyScrubChange(duration);
                break;
            case " ":
                event.preventDefault();
                playbackState.togglePlayPause();
                break;
            default:
                return;
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
                onpointerdown={handlePointerDown}
                onpointermove={handlePointerMove}
                onpointerup={handlePointerUp}
                onpointercancel={handlePointerUp}
                onkeydown={handleKeyDown}
                onmouseenter={() => (isHovering = true)}
                onmouseleave={() => (isHovering = false)}
                onwheel={handleWheel}
                role="slider"
                tabindex="0"
                aria-label="Seek bar"
                aria-valuemin={0}
                aria-valuemax={duration}
                aria-valuenow={displayTime}
            >
                {#if loader.isLoading}
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
                            isHovering={isHovering || isDragging || isLocked}
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
            <span>{formatTime(displayTime)}</span>
            <span>{formatTime(duration)}</span>
        </div>
    </div>
</div>
