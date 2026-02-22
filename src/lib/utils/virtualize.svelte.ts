export function createVirtualizer(options: {
    itemsCount: () => number;
    itemsPerRow: () => number;
    rowHeight: () => number;
    overscan?: number;
}) {
    let containerElement = $state<HTMLElement>();
    let scrollContainer = $state<HTMLElement | null>(null);
    let scrollTop = $state(0);
    let viewportHeight = $state(0);

    const totalRows = $derived(Math.ceil(options.itemsCount() / options.itemsPerRow()));
    const totalHeight = $derived(totalRows * options.rowHeight());

    const overscan = options.overscan ?? 2;

    const visibleRows = $derived.by(() => {
        const start = Math.max(0, Math.floor(scrollTop / options.rowHeight()) - overscan);
        const end = Math.min(
            totalRows,
            Math.ceil((scrollTop + viewportHeight) / options.rowHeight()) + overscan,
        );
        return { start, end };
    });

    const visibleRange = $derived.by(() => {
        const startIdx = visibleRows.start * options.itemsPerRow();
        const endIdx = Math.min(options.itemsCount(), visibleRows.end * options.itemsPerRow());
        return { start: startIdx, end: endIdx };
    });

    const translateY = $derived(visibleRows.start * options.rowHeight());

    $effect(() => {
        let parent = containerElement?.parentElement;
        while (parent) {
            const overflow = window.getComputedStyle(parent).overflowY;
            if (overflow === "auto" || overflow === "scroll") {
                scrollContainer = parent;
                break;
            }
            parent = parent.parentElement;
        }

        if (scrollContainer) {
            let rafId: number | null = null;
            const handleScroll = () => {
                if (rafId !== null) return;
                rafId = requestAnimationFrame(() => {
                    scrollTop = scrollContainer!.scrollTop;
                    viewportHeight = scrollContainer!.clientHeight;
                    rafId = null;
                });
            };
            scrollContainer.addEventListener("scroll", handleScroll, {
                passive: true,
            });
            handleScroll();

            const resizeObserver = new ResizeObserver(handleScroll);
            resizeObserver.observe(scrollContainer);

            return () => {
                scrollContainer?.removeEventListener("scroll", handleScroll);
                resizeObserver.disconnect();
                if (rafId !== null) cancelAnimationFrame(rafId);
            };
        }
    });

    return {
        get containerElement() { return containerElement; },
        set containerElement(val) { containerElement = val; },
        get totalHeight() { return totalHeight; },
        get visibleRange() { return visibleRange; },
        get translateY() { return translateY; },
        get isAtBottom() { return visibleRows.end >= totalRows - 1; }
    };
}
