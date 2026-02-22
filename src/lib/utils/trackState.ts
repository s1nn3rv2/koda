import { playbackState } from "$lib/state/playback.svelte";
import { queueState } from "$lib/state/queue.svelte";

export function isTrackActive(
    trackId: string,
    isQueueView: boolean,
    originalIndex?: number,
): boolean {
    if (isQueueView && originalIndex !== undefined) {
        return originalIndex === queueState.queueIndex && playbackState.currentTrackId !== null;
    }
    return playbackState.currentTrackId === trackId;
}

export function isTrackPlaying(
    trackId: string,
    isQueueView: boolean,
    originalIndex?: number,
): boolean {
    return isTrackActive(trackId, isQueueView, originalIndex) && playbackState.isActuallyPlaying;
}
