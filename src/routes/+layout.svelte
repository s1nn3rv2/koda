<script>
    import MusicPlayer from "$lib/components/MusicPlayer/MusicPlayer.svelte";
    import ContextMenu from "$lib/components/ContextMenu.svelte";
    import ConfirmDialog from "$lib/components/ConfirmDialog.svelte";
    import TitleBar from "$lib/components/TitleBar.svelte";
    import SettingsModal from "$lib/components/Settings/SettingsModal.svelte";
    import "../app.css";

    let { children } = $props();
</script>

<div class="flex flex-col h-screen bg-[#0a0a0f] text-white overflow-hidden">
    <TitleBar />
    <main class="flex-1 relative overflow-auto">
        {@render children()}
    </main>
    <MusicPlayer />
    <ContextMenu />
    <ConfirmDialog />
    <SettingsModal />
</div>
