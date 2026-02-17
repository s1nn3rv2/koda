<script lang="ts">
    import { X } from "@lucide/svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import { onMount } from "svelte";
    import { playerState } from "$lib/state/player.svelte";
    import CoverArt from "$lib/components/CoverArt.svelte";
    import type { Track, ScanProgress, LibraryStats } from "$lib/types";
    import { formatDuration, getFileName, pluralize } from "$lib/utils/format";

    let tracks = $state<Track[]>([]);
    let errorMsg = $state("");
    let successMsg = $state("");
    let isScanning = $state(false);
    let libraryStats = $state<LibraryStats | null>(null);
    let scanProgress = $state<ScanProgress | null>(null);

    const libraryCount = $derived(libraryStats?.total_tracks ?? 0);

    async function loadLibrary() {
        try {
            tracks = await invoke("get_all_tracks");
            libraryStats = await invoke<LibraryStats>("get_library_stats");
            errorMsg = "";
        } catch (e) {
            errorMsg = String(e);
        }
    }

    async function scanMusic() {
        try {
            isScanning = true;
            errorMsg = "";
            successMsg = "";
            scanProgress = null;

            const count: number = await invoke("scan_and_save_library");
            successMsg = `Added ${count} new ${pluralize(count, "track")} to library`;

            await loadLibrary();
        } catch (e) {
            errorMsg = String(e);
        } finally {
            isScanning = false;
            scanProgress = null;
        }
    }

    function playTrack(track: Track) {
        playerState.playTrack(track);
    }

    async function deleteTrack(id: string, event: MouseEvent) {
        event.stopPropagation();

        try {
            await invoke("delete_track", { id });
            await loadLibrary();
            successMsg = "Track removed from library";
        } catch (e) {
            errorMsg = String(e);
        }
    }

    onMount(() => {
        loadLibrary();

        const unlisten = listen<ScanProgress>("scan-progress", (event) => {
            scanProgress = event.payload;
        });

        return () => {
            unlisten.then((fn) => fn());
        };
    });
</script>

<main class="p-8 max-w-4xl mx-auto">
    <header class="mb-12">
        <h1 class="text-5xl font-black mb-4 tracking-tight">Library</h1>
        <div class="flex gap-4 items-center">
            <button
                onclick={scanMusic}
                disabled={isScanning}
                class="bg-white text-black px-6 py-2 rounded-full font-bold hover:bg-gray-200 transition disabled:opacity-50 disabled:cursor-not-allowed"
            >
                {#if isScanning && scanProgress}
                    Scanning ({scanProgress.current}/{scanProgress.total})
                {:else if isScanning}
                    Scanning...
                {:else}
                    Scan Music
                {/if}
            </button>
            {#if libraryCount > 0}
                <p class="text-gray-400 text-sm">
                    {libraryCount} track{libraryCount !== 1 ? "s" : ""} in library
                </p>
            {/if}
        </div>
    </header>

    <div class="grid gap-4">
        {#if errorMsg}
            <div
                class="p-4 bg-red-500/10 border border-red-500/20 rounded-xl text-red-400"
            >
                {errorMsg}
            </div>
        {/if}

        {#if isScanning && scanProgress}
            <div
                class="p-4 bg-blue-500/10 border border-blue-500/20 rounded-xl text-blue-400"
            >
                <div class="flex justify-between items-center mb-2">
                    <span
                        >Scanning: {scanProgress.current} / {scanProgress.total}</span
                    >
                    <span
                        >{Math.round(
                            (scanProgress.current / scanProgress.total) * 100,
                        )}%</span
                    >
                </div>
                <div class="w-full bg-white/10 rounded-full h-2 mb-2">
                    <div
                        class="bg-blue-500 h-2 rounded-full transition-all duration-300"
                        style="width: {(scanProgress.current /
                            scanProgress.total) *
                            100}%"
                    ></div>
                </div>
                <p class="text-xs text-blue-300 truncate">
                    {scanProgress.current_file}
                </p>
            </div>
        {/if}

        {#if successMsg}
            <div
                class="p-4 bg-green-500/10 border border-green-500/20 rounded-xl text-green-400"
            >
                {successMsg}
            </div>
        {/if}

        {#if tracks.length === 0 && !isScanning}
            <div class="text-center py-12">
                <p class="text-gray-400 text-lg mb-4">Your library is empty</p>
                <p class="text-gray-500 text-sm">
                    Click "Scan Music" to add tracks from your Music folder
                </p>
            </div>
        {/if}

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            {#each tracks as track (track.id)}
                <div
                    class="p-4 bg-white/5 border border-white/10 rounded-xl hover:bg-white/10 transition group relative"
                >
                    <button
                        onclick={() => playTrack(track)}
                        class="text-left w-full flex gap-3"
                    >
                        <div class="shrink-0">
                            <CoverArt
                                trackId={track.id}
                                alt={track.title || "Track cover"}
                                class="h-16 w-16 rounded-md"
                                size={128}
                            />
                        </div>
                        <div class="min-w-0 flex-1">
                            <p class="font-bold text-lg truncate">
                                {track.title || getFileName(track.path)}
                            </p>
                            {#if track.artists}
                                <p class="text-sm text-gray-400 truncate">
                                    {track.artists}
                                </p>
                            {/if}
                            <div class="flex justify-between items-center mt-2">
                                {#if track.album}
                                    <p
                                        class="text-xs text-gray-500 truncate flex-1"
                                    >
                                        {track.album}
                                    </p>
                                {/if}
                                {#if track.duration}
                                    <p class="text-xs text-gray-500 ml-auto">
                                        {formatDuration(track.duration)}
                                    </p>
                                {/if}
                            </div>
                        </div>
                    </button>

                    <button
                        onclick={(e) => deleteTrack(track.id, e)}
                        class="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity bg-red-500/20 hover:bg-red-500/30 text-red-400 p-2 rounded-lg text-xs"
                        title="Remove from library"
                    >
                        <X size={16} fill="currentColor" />
                    </button>
                </div>
            {/each}
        </div>
    </div>
</main>
