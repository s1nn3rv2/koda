<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { playerState } from "$lib/state/player.svelte";

    let musicFiles = $state<string[]>([]);
    let errorMsg = $state("");

    async function listMusic() {
        try {
            musicFiles = await invoke("list_music_files");
            errorMsg = "";
        } catch (e) {
            errorMsg = String(e);
        }
    }

    async function playMusic(path: string) {
        try {
            playerState.currentTrackPath = path;
            await invoke("play_file", { path });
            playerState.isPlaying = true;
            playerState.isPaused = false;
            errorMsg = "";
        } catch (e) {
            errorMsg = String(e);
        }
    }
</script>

<main class="p-8 max-w-4xl mx-auto">
    <header class="mb-12">
        <h1 class="text-5xl font-black mb-4 tracking-tight">Library</h1>
        <button
            onclick={listMusic}
            class="bg-white text-black px-6 py-2 rounded-full font-bold hover:bg-gray-200 transition"
        >
            Scan Music
        </button>
    </header>

    <div class="grid gap-4">
        {#if errorMsg}
            <div
                class="p-4 bg-red-500/10 border border-red-500/20 rounded-xl text-red-400"
            >
                {errorMsg}
            </div>
        {/if}

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            {#each musicFiles as file}
                <button
                    onclick={() => playMusic(file)}
                    class="p-4 bg-white/5 border border-white/10 rounded-xl hover:bg-white/10 transition"
                >
                    <p class="font-medium truncate">{file}</p>
                </button>
            {/each}
        </div>
    </div>
</main>
