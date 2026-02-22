<script lang="ts">
    import { onMount } from "svelte";
    import { uiState } from "$lib/state/player.svelte";

    let { children, class: className = "", speed = 30 } = $props();
    let container = $state<HTMLDivElement>();
    let content = $state<HTMLDivElement>();
    let inner = $state<HTMLDivElement>();
    let isOverflowing = $state(false);
    let pendingStop = $state(false);
    let duration = $state(0);

    function checkOverflow() {
        if (container && inner) {
            const hasOverflow = inner.scrollWidth > container.clientWidth + 1;

            if (hasOverflow) {
                isOverflowing = true;
                pendingStop = false;
            } else if (isOverflowing) {
                pendingStop = true;

                if (content) {
                    const style = window.getComputedStyle(content);
                    if (
                        style.transform === "none" ||
                        style.transform === "matrix(1, 0, 0, 1, 0, 0)"
                    ) {
                        isOverflowing = false;
                        pendingStop = false;
                    }
                }
            }

            if (isOverflowing && content) {
                duration = content.scrollWidth / 2 / speed;
            }
        }
    }

    function handleIteration() {
        if (pendingStop) {
            isOverflowing = false;
            pendingStop = false;
        }
    }

    $effect(() => {
        children;
        uiState.isExpanded;
        const timer = setTimeout(checkOverflow, 150);
        return () => clearTimeout(timer);
    });

    onMount(() => {
        checkOverflow();

        // ResizeObserver handles changes to the container size
        const resizeObserver = new ResizeObserver(() => {
            checkOverflow();
        });
        if (container) resizeObserver.observe(container);

        // Fallback for general window resizes
        window.addEventListener("resize", checkOverflow);

        return () => {
            resizeObserver.disconnect();
            window.removeEventListener("resize", checkOverflow);
        };
    });

    const mask = $derived(
        isOverflowing
            ? "linear-gradient(to right, transparent 0%, black 20px, black calc(100% - 20px), transparent 100%)"
            : "none",
    );
</script>

<div
    bind:this={container}
    class="relative overflow-hidden whitespace-nowrap min-w-0 w-full {className}"
    style:mask-image={mask}
    style:-webkit-mask-image={mask}
>
    <div
        bind:this={content}
        class="inline-block whitespace-nowrap"
        class:animate-marquee={isOverflowing}
        onanimationiteration={handleIteration}
        style:animation-duration="{duration}s"
        style:animation-delay="2s"
    >
        <div bind:this={inner} class="inline-block">
            {@render children()}
        </div>
        {#if isOverflowing}
            <div class="inline-block ml-12">
                {@render children()}
            </div>
        {/if}
    </div>
</div>

<style>
    .animate-marquee {
        display: inline-block;
        animation: marquee linear infinite;
    }

    @keyframes marquee {
        0% {
            transform: translateX(0);
        }
        100% {
            transform: translateX(calc(-50% - 1.5rem));
        }
    }
</style>
