<script lang="ts">
    let volume = $state(70);
</script>

<div class="hidden items-center gap-3 md:flex">
    <span class="text-xs text-white/60">Vol</span>
    <input
        type="range"
        min="0"
        max="100"
        bind:value={volume}
        class="h-1 w-28 cursor-pointer appearance-none rounded-lg bg-white/20 accent-indigo-300"
    />
</div>
