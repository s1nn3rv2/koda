<script lang="ts">
    import { libraryState } from "$lib/state/library.svelte";
    import { uiState } from "$lib/state/player.svelte";
    import LibraryHeader from "$lib/components/Library/LibraryHeader.svelte";
    import SidebarSection from "$lib/components/Library/SidebarSection.svelte";
    import MainSection from "$lib/components/Library/MainSection.svelte";
    import TagEditorModal from "$lib/components/Library/TagEditorModal.svelte";
    import ImportModal from "$lib/components/Library/ImportModal.svelte";
    import { onMount } from "svelte";
    import { PanelLeft } from "@lucide/svelte";
    import { fly, fade, scale, crossfade } from "svelte/transition";
    import { cubicInOut } from "svelte/easing";

    const [send, receive] = crossfade({
        duration: 400,
        easing: cubicInOut,
        fallback(node, params) {
            return scale(node, params as any);
        },
    });

    function handleReload() {
        libraryState.refresh();
    }

    onMount(() => {
        const handleKeyDown = (e: KeyboardEvent) => {
            if ((e.ctrlKey || e.metaKey) && e.key === "b") {
                e.preventDefault();
                uiState.toggleSidebar();
            }
        };

        window.addEventListener("keydown", handleKeyDown);
        return () => window.removeEventListener("keydown", handleKeyDown);
    });
</script>

<main class="flex flex-col h-screen">
    <LibraryHeader onreload={handleReload} {send} {receive} />

    <div class="flex flex-1 min-h-0 px-8 gap-0 overflow-hidden relative">
        <div
            class="flex shrink-0 h-full transition-all duration-400 ease-[cubic-bezier(0.4,0,0.2,1)] overflow-hidden"
            style="width: {uiState.sidebarHidden ? '0' : '256px'}; 
                   margin-right: {uiState.sidebarHidden ? '0' : '24px'};
                   opacity: {uiState.sidebarHidden ? '0' : '1'};
                   pointer-events: {uiState.sidebarHidden ? 'none' : 'auto'};
                   will-change: width, opacity, margin-right;"
        >
            <div class="w-64 h-full">
                <SidebarSection {send} {receive} />
            </div>
        </div>

        <div class="flex-1 flex flex-col min-w-0">
            <MainSection />
        </div>
    </div>

    <TagEditorModal />
    <ImportModal />
</main>
