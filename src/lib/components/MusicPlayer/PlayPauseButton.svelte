<script lang="ts">
    import { Tween } from "svelte/motion";
    import { cubicInOut } from "svelte/easing";
    import { interpolate } from "flubber";

    const ANIMATION_DURATION = 250;

    let { isPlaying, onclick, class: className = "bg-white text-slate-900 w-12 h-12 shadow-lg hover:bg-white/90" } = $props<{
        isPlaying: boolean;
        onclick: (e: MouseEvent) => void;
        class?: string;
    }>();

    // copied from god knows even where

    const playLeft = "M 6,4 L 13.25,8.14 L 13.25,15.86 L 6,20 Z";
    const pauseLeft = "M 6,4 L 10,4 L 10,20 L 6,20 Z";

    const pathLeft = new Tween(playLeft, {
        duration: ANIMATION_DURATION,
        easing: cubicInOut,
        interpolate: interpolate,
    });

    const playRight = "M 12.75,7.86 L 20,12 L 12.75,16.14 Z";
    const pauseRight = "M 14,4 L 18,4 L 18,20 L 14,20 Z";

    const pathRight = new Tween(playRight, {
        duration: ANIMATION_DURATION,
        easing: cubicInOut,
        interpolate: interpolate,
    });

    $effect(() => {
        pathLeft.target = isPlaying ? pauseLeft : playLeft;
        pathRight.target = isPlaying ? pauseRight : playRight;
    });
</script>

<button
    class="rounded-full flex items-center justify-center transition hover:scale-105 cursor-pointer {className}"
    {onclick}
    aria-label={isPlaying ? "Pause" : "Play"}
>
    <svg viewBox="0 0 24 24" fill="currentColor" class="w-2/3 h-2/3">
        <path d={pathLeft.current} />
        <path d={pathRight.current} />
    </svg>
</button>
