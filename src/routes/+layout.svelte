<script>
    import MusicPlayer from "$lib/components/MusicPlayer/MusicPlayer.svelte";
    import "../app.css";

    let { children } = $props();
</script>

<div class="min-h-screen bg-[#0a0a0f] text-white">
    <div class="pb-24">
        {@render children()}
    </div>
    <MusicPlayer />
</div>
