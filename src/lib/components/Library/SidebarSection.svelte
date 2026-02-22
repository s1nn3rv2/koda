<script lang="ts">
    import { Search, PanelLeft } from "@lucide/svelte";
    import { libraryState } from "$lib/state/library.svelte";
    import { uiState } from "$lib/state/player.svelte";
    import LibrarySidebar from "./LibrarySidebar/LibrarySidebar.svelte";

    interface Props {
        send: any;
        receive: any;
    }

    let { send, receive }: Props = $props();

    function handleSelect(sel: any) {
        libraryState.setSelection(sel);
    }

    function toggleMode() {
        const nextMode = uiState.libraryMode === "local" ? "online" : "local";
        uiState.setLibraryMode(nextMode);
    }
</script>

<aside
    class="w-64 h-full shrink-0 overflow-y-auto rounded-xl bg-white/[0.02] border border-white/5 p-3 pb-24"
>
    <div class="flex gap-2 items-center mb-3">
        {#if !uiState.sidebarHidden}
            <button
                onclick={() => uiState.toggleSidebar()}
                in:receive={{ key: "sidebar-toggle" }}
                out:send={{ key: "sidebar-toggle" }}
                class="p-2 bg-white/5 hover:bg-indigo-500/20 rounded-lg transition-colors text-gray-400 hover:text-indigo-400 shrink-0"
                title="Hide Sidebar"
            >
                <PanelLeft size={18} />
            </button>
        {/if}
        <div class="relative flex-1">
            <Search
                size={14}
                class="absolute left-2.5 top-1/2 -translate-y-1/2 text-gray-500 pointer-events-none"
            />
            <input
                type="text"
                placeholder={uiState.libraryMode === "local"
                    ? "Search..."
                    : "Search online..."}
                bind:value={libraryState.searchQuery}
                class="w-full pl-8 pr-3 py-1.5 bg-white/5 border border-white/10 rounded-lg text-sm text-white placeholder-gray-500 focus:outline-none focus:border-white/20 transition"
            />
        </div>
    </div>

    <LibrarySidebar
        loadKey={libraryState.refreshTrigger}
        selection={libraryState.selection}
        onselect={handleSelect}
        searchQuery={libraryState.searchQuery}
    />
</aside>
