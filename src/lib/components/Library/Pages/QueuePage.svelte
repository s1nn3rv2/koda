<script lang="ts">
    import { Trash2 } from "@lucide/svelte";
    import { queueState } from "$lib/state/player.svelte";
    import TrackList from "../TrackList.svelte";

    interface Props {
        ondelete: (id: string) => void;
        onartistclick: (artistName: string) => void;
        onalbumclick: (albumId: string) => void;
    }

    let { ondelete, onartistclick, onalbumclick }: Props = $props();

    let tracks = $derived(queueState.queue);
</script>

<div class="mb-4">
    <div class="flex items-center justify-between mb-6">
        <div>
            <h1 class="text-3xl font-bold mb-2">Play Queue</h1>
            <p class="text-sm text-text-secondary">
                {tracks.length} track{tracks.length === 1 ? "" : "s"} in manual queue
            </p>
        </div>
        {#if tracks.length > 1}
            <button
                class="px-4 py-2 bg-white/5 hover:bg-red-500/20 hover:text-red-400 text-sm font-medium rounded-lg transition-colors flex items-center gap-2 group"
                onclick={() => queueState.clearQueue()}
            >
                <Trash2
                    size={16}
                    class="text-gray-400 group-hover:text-red-400 transition-colors"
                />
                Clear Queue
            </button>
        {/if}
    </div>

    {#if tracks.length === 0}
        <div class="text-center py-12">
            <p class="text-gray-400 text-lg mb-4">Your queue is empty</p>
            <p class="text-gray-500 text-sm">
                Play a track or album to add it to the queue
            </p>
        </div>
    {:else}
        <TrackList
            {tracks}
            {ondelete}
            {onartistclick}
            {onalbumclick}
            isQueueView={true}
            onremovefromqueue={(idx: number) => queueState.removeFromQueue(idx)}
            onreorder={(fromIdx: number, toIdx: number) =>
                queueState.reorderQueue(fromIdx, toIdx)}
            onplayqueueitem={(idx: number) => queueState.playQueueIndex(idx)}
            layout="list"
        />
    {/if}
</div>
