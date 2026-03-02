<script lang="ts">
    import TrackComponent from "./TrackComponent.svelte";
    import { queueState } from "$lib/state/player.svelte";
    import { createVirtualizer } from "$lib/utils/virtualize.svelte";
    import type { Track } from "$lib/types";

    interface Props {
        tracks: Track[];
        ondelete?: (id: string) => void;
        ondeletefile?: (id: string) => void;
        onloadmore?: () => void;
        hasMore?: boolean;
        isLoadingMore?: boolean;
        onartistclick?: (artistName: string) => void;
        onalbumclick?: (albumId: string) => void;
        isAlbumView?: boolean;
        isQueueView?: boolean;
        onremovefromqueue?: (idx: number) => void;
        onreorder?: (fromIdx: number, toIdx: number) => void;
        onplayqueueitem?: (idx: number) => void;
        layout?: "grid" | "list";
    }

    let {
        tracks,
        ondelete,
        ondeletefile,
        onloadmore,
        hasMore = false,
        isLoadingMore = false,
        onartistclick,
        onalbumclick,
        isAlbumView = false,
        isQueueView = false,
        onremovefromqueue,
        onreorder,
        onplayqueueitem,
        layout = "grid",
    }: Props = $props();

    let containerWidth = $state(0);

    const itemsPerRow = $derived.by(() => {
        if (layout === "list") return 1;
        if (containerWidth === 0) return 4;

        if (containerWidth >= 1500) return 8;
        if (containerWidth >= 1300) return 7;
        if (containerWidth >= 1100) return 6;
        if (containerWidth >= 900) return 5;
        if (containerWidth >= 700) return 4;
        if (containerWidth >= 500) return 3;
        return 2;
    });

    function playTrack(track: Track) {
        queueState.playTrack(track, tracks);
    }

    function handleDelete(id: string) {
        ondelete?.(id);
    }

    function handleDeleteFile(id: string) {
        ondeletefile?.(id);
    }

    function handleArtistClick(
        artistName: string,
        event: MouseEvent | KeyboardEvent,
    ) {
        event.stopPropagation();
        event.preventDefault();
        onartistclick?.(artistName.trim());
    }

    function handleAlbumClick(
        albumId: string,
        event: MouseEvent | KeyboardEvent,
    ) {
        event.stopPropagation();
        event.preventDefault();
        onalbumclick?.(albumId);
    }

    function setupIntersectionObserver(el: HTMLElement) {
        const observer = new IntersectionObserver((entries) => {
            if (entries[0].isIntersecting) {
                onloadmore?.();
            }
        });
        observer.observe(el);
        return {
            destroy() {
                observer.disconnect();
            },
        };
    }
    let draggedIndex = $state<number | null>(null);
    let dragOverIndex = $state<number | null>(null);

    function handleDragStart(e: DragEvent, index: number) {
        if (!isQueueView) {
            e.preventDefault();
            return;
        }
        draggedIndex = index;
        if (e.dataTransfer) {
            e.dataTransfer.effectAllowed = "move";
            e.dataTransfer.setData("text/plain", index.toString());
        }
    }

    function handleDragOver(e: DragEvent, index: number) {
        e.preventDefault();
        if (!isQueueView || draggedIndex === null) return;

        if (dragOverIndex !== index) {
            dragOverIndex = index;
        }

        if (e.dataTransfer) {
            e.dataTransfer.dropEffect = "move";
        }
    }

    function handleDragLeave(e: DragEvent, index: number) {
        if (dragOverIndex === index) {
            dragOverIndex = null;
        }
    }

    function handleDrop(e: DragEvent, index: number) {
        e.preventDefault();
        if (draggedIndex !== null && draggedIndex !== index) {
            onreorder?.(draggedIndex, index);
        }
        draggedIndex = null;
        dragOverIndex = null;
    }

    function handleDragEnd() {
        draggedIndex = null;
        dragOverIndex = null;
    }

    const rowHeight = $derived.by(() => {
        if (layout === "list") return 60; // 56px intrinsic height + 4px gap

        const totalGapWidth = (itemsPerRow - 1) * 16;
        const itemWidth = (containerWidth - totalGapWidth) / itemsPerRow;

        return itemWidth + 66 + 16; // image width + text height + 16px row gap
    });

    const virtualizer = createVirtualizer({
        itemsCount: () => tracks.length,
        itemsPerRow: () => itemsPerRow,
        rowHeight: () => rowHeight,
    });

    const visibleRange = $derived(virtualizer.visibleRange);
    const visibleTracksSlice = $derived(
        tracks.slice(visibleRange.start, visibleRange.end),
    );

    $effect(() => {
        if (hasMore && !isLoadingMore && virtualizer.isAtBottom) {
            onloadmore?.();
        }
    });
</script>

{#if tracks.length === 0 && !isLoadingMore}
    <div class="text-center py-12">
        <p class="text-gray-400 text-lg">No tracks found</p>
    </div>
{:else}
    <div
        class="h-full w-full"
        bind:this={virtualizer.containerElement}
        bind:clientWidth={containerWidth}
    >
        {#if tracks.length > 0}
            <div
                class="relative w-full"
                style="height: {virtualizer.totalHeight}px;"
            >
                <div
                    class={layout === "grid"
                        ? "grid gap-4 w-full"
                        : "flex flex-col gap-1 w-full"}
                    style="position: absolute; top: 0; left: 0; right: 0; transform: translateY({virtualizer.translateY}px); {layout ===
                    'grid'
                        ? `grid-template-columns: repeat(${itemsPerRow}, minmax(0, 1fr));`
                        : ''}"
                >
                    {#each visibleTracksSlice as track, i (visibleRange.start + i)}
                        {@const index = visibleRange.start + i}
                        <div
                            draggable={isQueueView}
                            ondragstart={(e) => handleDragStart(e, index)}
                            ondragover={(e) => handleDragOver(e, index)}
                            ondragleave={(e) => handleDragLeave(e, index)}
                            ondrop={(e) => handleDrop(e, index)}
                            ondragend={handleDragEnd}
                            class="relative h-full {isQueueView && index === 0
                                ? 'opacity-100'
                                : ''} {draggedIndex === index
                                ? 'opacity-30 scale-95'
                                : ''}"
                            role="listitem"
                        >
                            {#if dragOverIndex === index}
                                <div
                                    class="absolute inset-x-0 {draggedIndex !==
                                        null && draggedIndex < index
                                        ? '-bottom-0.5'
                                        : '-top-0.5'} h-1 bg-indigo-500 z-10 rounded-full shadow-[0_0_8px_rgba(99,102,241,0.8)] pointer-events-none"
                                ></div>
                            {/if}
                            <TrackComponent
                                {track}
                                originalIndex={index}
                                playTrack={(t: Track) => {
                                    if (isQueueView && onplayqueueitem) {
                                        onplayqueueitem(index);
                                    } else {
                                        playTrack(t);
                                    }
                                }}
                                {handleDelete}
                                ondeletefile={handleDeleteFile}
                                {handleArtistClick}
                                {handleAlbumClick}
                                {isAlbumView}
                                {isQueueView}
                                onremovefromqueue={() =>
                                    onremovefromqueue?.(index)}
                                compact={layout === "list"}
                            />
                        </div>
                    {/each}
                </div>
            </div>
        {/if}
    </div>

    {#if isLoadingMore}
        <div class="flex justify-center py-6">
            <p class="text-gray-400 text-sm">Loading more tracks...</p>
        </div>
    {/if}
{/if}
