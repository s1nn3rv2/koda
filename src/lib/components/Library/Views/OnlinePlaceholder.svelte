<script lang="ts">
    import { Globe } from "@lucide/svelte";
</script>

<div
    class="flex flex-col items-center justify-center flex-1 h-full text-center p-12"
>
    <div
        class="w-20 h-20 bg-indigo-500/10 rounded-3xl flex items-center justify-center mb-6 border border-indigo-500/20"
    >
        <Globe size={40} class="text-indigo-400" />
    </div>
    <h2 class="text-2xl font-bold text-white mb-2">Online Library</h2>
    <p class="text-gray-400 max-w-sm">
        Search for tracks, artists, or albums to browse the Monochrome database.
    </p>
</div>
