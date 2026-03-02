<script lang="ts">
    import { TauriService } from "$lib/utils/tauri";
    import { listen } from "@tauri-apps/api/event";
    import { PanelLeft, X } from "@lucide/svelte";
    import { onMount } from "svelte";
    import { slide } from "svelte/transition";
    import { cubicInOut } from "svelte/easing";
    import type { ScanProgress, LibraryStats } from "$lib/types";
    import { pluralize } from "$lib/utils/format";
    import { libraryState } from "$lib/state/library.svelte";
    import { uiState } from "$lib/state/player.svelte";
    import { settingsState } from "$lib/state/settings.svelte";
    import { downloadState } from "$lib/state/download.svelte";
    import LibrarySourceSelector from "./LibrarySourceSelector.svelte";
    import DownloadProgress from "./Modals/DownloadProgress.svelte";

    interface Props {
        onreload: () => void;
        send?: any;
        receive?: any;
    }

    let { onreload, send, receive }: Props = $props();

    let isScanning = $state(false);
    let libraryStats = $state<LibraryStats | null>(null);
    let errorMsg = $state("");
    let successMsg = $state("");

    const libraryCount = $derived(libraryStats?.total_tracks ?? 0);

    async function loadStats() {
        try {
            libraryStats = await TauriService.getLibraryStats();
        } catch (e) {
            console.error("Failed to load stats:", e);
        }
    }

    async function scanMusic() {
        if (settingsState.musicPaths.length === 0) {
            errorMsg =
                "No music folders configured. Please add some in Settings.";
            settingsState.isSettingsOpen = true;
            return;
        }

        try {
            isScanning = true;
            errorMsg = "";
            successMsg = "";

            const count: number = await TauriService.scanAndSaveLibrary(
                $state.snapshot(settingsState.musicPaths),
            );
            successMsg = `Added ${count} new ${pluralize(count, "track")} to library`;

            await loadStats();
            onreload();
        } catch (e) {
            errorMsg = String(e);
        } finally {
            isScanning = false;
        }
    }

    onMount(() => {
        loadStats();
    });
</script>

<header class="px-8 pt-8 pb-4 shrink-0">
    <div class="flex items-center justify-between mb-4">
        <div class="flex items-center">
            <div
                class="flex items-center transition-all duration-400 ease-in-out overflow-hidden"
                style="width: {uiState.sidebarHidden
                    ? '56px'
                    : '0px'}; opacity: {uiState.sidebarHidden ? 1 : 0};"
            >
                {#if uiState.sidebarHidden}
                    <button
                        onclick={() => uiState.toggleSidebar()}
                        in:receive={{ key: "sidebar-toggle" }}
                        out:send={{ key: "sidebar-toggle" }}
                        class="p-2 bg-indigo-500 hover:bg-indigo-600 rounded-lg transition-colors text-white shadow-lg z-50 mr-4"
                        title="Show Sidebar"
                    >
                        <PanelLeft size={24} />
                    </button>
                {/if}
            </div>

            <div class="flex items-center gap-6">
                <LibrarySourceSelector />
                <h1 class="text-4xl font-black tracking-tight">Library</h1>
            </div>
        </div>
        <div class="flex gap-4 items-center">
            {#if uiState.libraryMode === "local"}
                <button
                    onclick={scanMusic}
                    disabled={isScanning}
                    class="bg-white text-black px-5 py-2 rounded-full font-bold text-sm hover:bg-gray-200 transition disabled:opacity-50 disabled:cursor-not-allowed"
                >
                    {#if isScanning && libraryState.scanProgress}
                        Scanning ({libraryState.scanProgress
                            .current}/{libraryState.scanProgress.total})
                    {:else if isScanning}
                        Scanning...
                    {:else}
                        Scan Music
                    {/if}
                </button>
                {#if libraryCount > 0}
                    <p class="text-gray-500 text-xs">
                        {libraryCount}
                        {pluralize(libraryCount, "track")}
                        {#if libraryStats}
                            · {libraryStats.unique_artists}
                            {pluralize(libraryStats.unique_artists, "artist")}
                            · {libraryStats.unique_albums}
                            {pluralize(libraryStats.unique_albums, "album")}
                        {/if}
                    </p>
                {/if}
            {/if}
        </div>
    </div>

    {#if errorMsg}
        <div
            class="mb-3 p-3 bg-red-500/10 border border-red-500/20 rounded-xl text-red-400 text-sm"
        >
            {errorMsg}
        </div>
    {/if}

    {#if libraryState.scanProgress}
        <div
            class="mb-3 p-3 bg-blue-500/10 border border-blue-500/20 rounded-xl text-blue-400 text-sm"
        >
            <div class="flex justify-between items-center mb-1.5">
                <span
                    >Scanning: {libraryState.scanProgress.current} / {libraryState
                        .scanProgress.total}</span
                >
                <span
                    >{Math.round(
                        (libraryState.scanProgress.current /
                            libraryState.scanProgress.total) *
                            100,
                    )}%</span
                >
            </div>
            <div class="w-full bg-white/10 rounded-full h-1.5 mb-1.5">
                <div
                    class="bg-blue-500 h-1.5 rounded-full transition-all duration-25"
                    style="width: {(libraryState.scanProgress.current /
                        libraryState.scanProgress.total) *
                        100}%"
                ></div>
            </div>
            <p class="text-xs text-blue-300 truncate">
                {libraryState.scanProgress.current_file}
            </p>
        </div>
    {/if}

    {#if successMsg && !libraryState.scanProgress}
        <div
            transition:slide={{ duration: 300, easing: cubicInOut }}
            class="mb-3 p-3 bg-green-500/10 border border-green-500/20 rounded-xl text-green-400 text-sm flex items-center justify-between group/msg"
        >
            <span>{successMsg}</span>
            <button
                onclick={() => (successMsg = "")}
                class="p-1.5 hover:bg-white/10 rounded-lg text-green-400/50 hover:text-green-400 transition-colors opacity-0 group-hover/msg:opacity-100"
                title="Dismiss"
            >
                <X size={16} />
            </button>
        </div>
    {/if}
    {#if downloadState.downloads.length > 0}
        <DownloadProgress />
    {/if}
</header>
