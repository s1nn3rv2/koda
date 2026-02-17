<script lang="ts">
    import { goto } from "$app/navigation";
</script>

<main class="flex items-center justify-center min-h-screen p-8">
    <div class="text-center max-w-2xl">
        <h1 class="text-7xl font-black mb-6">Koda</h1>
        <p class="text-xl text-gray-400 mb-12">Your music player</p>

        <div class="flex gap-4 justify-center">
            <button
                onclick={() => goto("/library")}
                class="bg-white text-black px-8 py-4 rounded-full font-bold text-lg hover:bg-gray-200 transition"
            >
                Open Library
            </button>
        </div>

        <div class="mt-16 text-sm text-gray-500">
            <p>Press the button above to browse your music library</p>
        </div>
    </div>
</main>
