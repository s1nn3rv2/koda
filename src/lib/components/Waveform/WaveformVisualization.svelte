<script lang="ts">
    import type { WaveformData } from "$lib/types";
    import { formatTime } from "$lib/utils/format";

    interface Props {
        waveformData: WaveformData;
        progress: number;
        isHovering: boolean;
        hoverPosition: number;
        duration: number;
    }

    let { waveformData, progress, isHovering, hoverPosition, duration }: Props =
        $props();
</script>

<svg
    class="h-full w-full"
    viewBox="0 0 {waveformData.samples.length} 100"
    preserveAspectRatio="none"
>
    {#each waveformData.samples as sample, i}
        {@const barProgress = (i / waveformData.samples.length) * 100}
        {@const isHovered = isHovering && barProgress <= hoverPosition}
        <rect
            x={i}
            y={50 - sample * 45}
            width="1"
            height={sample * 90}
            class={isHovered
                ? "fill-white/30 transition-colors"
                : "fill-white/20 transition-colors"}
        />
    {/each}

    <!-- played waveform -->
    {#each waveformData.samples as sample, i}
        {#if (i / waveformData.samples.length) * 100 <= progress}
            <rect
                x={i}
                y={50 - sample * 45}
                width="1"
                height={sample * 90}
                class="fill-indigo-400"
            />
        {/if}
    {/each}
</svg>

<!-- progress -->
<div
    class="pointer-events-none absolute top-0 h-full w-0.5 bg-white shadow-lg transition-all"
    style="left: {progress}%"
></div>

<!-- on hover -->
{#if isHovering}
    <div
        class="pointer-events-none absolute top-0 h-full w-0.5 bg-white/50"
        style="left: {hoverPosition}%"
    ></div>
    <div
        class="pointer-events-none absolute -top-8 rounded bg-black/80 px-2 py-1 text-xs text-white"
        style="left: {hoverPosition}%; transform: translateX(-50%)"
    >
        {formatTime((hoverPosition / 100) * duration)}
    </div>
{/if}
