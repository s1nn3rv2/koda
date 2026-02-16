<script>
    import TrackInfo from "./TrackInfo.svelte";
    import Controls from "./Controls.svelte";
    import Volume from "./Volume.svelte";
</script>

<footer
    class="fixed bottom-0 left-0 right-0 z-50
           border-t border-white/10
           bg-gradient-to-r from-slate-900/90 via-indigo-950/80 to-purple-950/80
           shadow-2xl"
>
    <div
        class="mx-auto flex h-20 max-w-7xl items-center justify-between px-4 md:px-6"
    >
        <TrackInfo />

        <Controls />

        <Volume />
    </div>
</footer>
