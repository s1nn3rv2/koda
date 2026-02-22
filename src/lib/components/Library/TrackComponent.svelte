<script lang="ts">
    import {
        Play,
        ListMusic,
        User,
        Disc3,
        Edit,
        X,
        Trash2,
    } from "@lucide/svelte";
    import { splitArtists } from "$lib/utils/split_artists";
    import { queueState } from "$lib/state/queue.svelte";
    import { libraryState } from "$lib/state/library.svelte";
    import { downloadState } from "$lib/state/download.svelte";
    import {
        contextMenuState,
        type ContextMenuItem,
    } from "$lib/state/context_menu.svelte";
    import { ask } from "@tauri-apps/plugin-dialog";
    import { TauriService } from "$lib/utils/tauri";
    import TrackListRow from "./TrackListRow.svelte";
    import TrackGridItem from "./TrackGridItem.svelte";

    let {
        track,
        playTrack,
        handleDelete,
        ondeletefile,
        handleArtistClick,

        handleAlbumClick,
        isAlbumView = false,
        isQueueView = false,
        onremovefromqueue,
        originalIndex,
        compact = false,
    } = $props();

    function handleContextMenu(e: MouseEvent) {
        e.preventDefault();

        const isOnline = track.id.startsWith("online:");

        let menuItems: ContextMenuItem[] = [
            {
                label: "Play Now",
                icon: Play,
                onclick: () => playTrack(track),
            },
        ];

        if (isOnline) {
            menuItems.push({
                label: "Download",
                icon: ListMusic,
                onclick: () => {
                    downloadState.startDownload(track);
                },
            });
        } else {
            menuItems.push(
                {
                    label: "Add to Queue",
                    icon: ListMusic,
                    onclick: () => queueState.addToQueue(track),
                },
                {
                    label: "View Artist",
                    icon: User,
                    onclick: () => {
                        const primaryArtist = splitArtists(
                            track.artists || "",
                        )[0];
                        if (primaryArtist) handleArtistClick(primaryArtist, e);
                    },
                },
            );

            if (track.album_id) {
                menuItems.push({
                    label: "View Album",
                    icon: Disc3,
                    onclick: () => {
                        if (track.album_id) handleAlbumClick(track.album_id, e);
                    },
                });
            }

            menuItems.push(
                {
                    label: "Edit Tags",
                    icon: Edit,
                    onclick: () => libraryState.editTrack(track),
                },
                {
                    label: "Delete File",
                    icon: Trash2,
                    variant: "danger",
                    onclick: async () => {
                        const confirmed = await ask(
                            `Are you sure you want to permanently delete "${track.title}" from your disk? This cannot be undone.`,
                            {
                                title: "Delete File",
                                kind: "warning",
                            },
                        );
                        if (confirmed) {
                            ondeletefile?.(track.id);
                        }
                    },
                },

                {
                    label: "Remove from Library",
                    icon: X,
                    onclick: () => handleDelete(track.id, e as any),
                },
            );
        }

        contextMenuState.open(e.clientX, e.clientY, menuItems);
    }

    const artistsList = $derived(
        track.artists ? splitArtists(track.artists) : [],
    );
</script>

<div
    class="{compact
        ? 'py-2 px-4 flex items-center gap-4'
        : 'p-3 flex flex-col gap-1.5'} bg-surface-base border border-transparent rounded-xl group relative hover:bg-surface-hover hover:border-surface-border transition-colors duration-200"
    style="content-visibility: auto; contain-intrinsic-size: auto {compact
        ? '56px'
        : '220px'};"
    oncontextmenu={handleContextMenu}
    role="button"
    tabindex="0"
>
    {#if compact}
        <TrackListRow
            {track}
            {originalIndex}
            {artistsList}
            {handleArtistClick}
            {handleAlbumClick}
            {playTrack}
            {onremovefromqueue}
        />
    {:else}
        <TrackGridItem
            {track}
            {originalIndex}
            {isQueueView}
            {artistsList}
            {handleArtistClick}
            {handleAlbumClick}
            {playTrack}
            {onremovefromqueue}
        />
    {/if}
</div>
