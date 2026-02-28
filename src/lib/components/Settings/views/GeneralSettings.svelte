<script lang="ts">
    import { settingsState } from "$lib/state/settings.svelte";
    import {
        FolderPlus,
        Trash2,
        HardDrive,
        Music2,
        Check,
    } from "@lucide/svelte";
    import { open } from "@tauri-apps/plugin-dialog";

    async function addFolder() {
        try {
            const selected = await open({
                directory: true,
                multiple: false,
                title: "Select Music Folder",
            });

            if (selected && typeof selected === "string") {
                settingsState.addMusicPath(selected);
            }
        } catch (e) {
            console.error("Failed to select folder:", e);
        }
    }
</script>

<section class="space-y-4">
    <div class="flex items-center justify-between mb-2">
        <div class="flex items-center gap-2">
            <HardDrive size={16} class="text-indigo-400" />
            <h3 class="text-sm font-bold text-white uppercase tracking-widest">
                Local Library
            </h3>
        </div>
        <button
            onclick={addFolder}
            class="flex items-center gap-2 px-3 py-1.5 bg-indigo-500 hover:bg-indigo-400 text-white text-[10px] font-bold rounded-lg transition-all active:scale-95"
        >
            <FolderPlus size={14} />
            Add Folder
        </button>
    </div>

    <p class="text-xs text-gray-400 mb-2">
        Koda will scan these folders for audio files. You can add multiple
        locations.
    </p>

    <div class="space-y-2">
        {#if settingsState.musicPaths.length === 0}
            <div
                class="p-8 bg-white/2 border border-white/5 border-dashed rounded-2xl text-center"
            >
                <p class="text-[11px] text-gray-500">
                    No music folders added yet.
                </p>
                <button
                    onclick={addFolder}
                    class="mt-3 text-indigo-400 text-xs font-semibold hover:text-indigo-300 transition-colors"
                >
                    Select your first folder
                </button>
            </div>
        {:else}
            <div class="grid gap-2">
                {#each settingsState.musicPaths as path}
                    <div
                        class="flex items-center justify-between p-3 bg-white/3 border border-white/5 rounded-xl group"
                    >
                        <div class="flex items-center gap-3 overflow-hidden">
                            <HardDrive
                                size={14}
                                class="text-gray-500 shrink-0"
                            />
                            <span
                                class="text-xs text-gray-300 truncate font-mono"
                                >{path}</span
                            >
                        </div>
                        <button
                            onclick={() => settingsState.removeMusicPath(path)}
                            class="p-1.5 text-gray-500 hover:text-red-400 hover:bg-red-400/10 rounded-lg transition-all opacity-0 group-hover:opacity-100"
                            title="Remove path"
                        >
                            <Trash2 size={14} />
                        </button>
                    </div>
                {/each}
            </div>
        {/if}
    </div>
</section>

<section class="space-y-4">
    <div class="flex items-center gap-2 mb-2">
        <Music2 size={16} class="text-indigo-400" />
        <h3 class="text-sm font-bold text-white uppercase tracking-widest">
            Streaming Quality
        </h3>
    </div>

    <p class="text-xs text-gray-400 mb-2">
        Higher quality uses more bandwidth and takes longer to load.
    </p>

    <div class="grid grid-cols-2 gap-2">
        {#each [{ value: "LOW", label: "Low", desc: "AAC ~96kbps" }, { value: "HIGH", label: "High", desc: "AAC ~320kbps" }, { value: "LOSSLESS", label: "Lossless", desc: "FLAC 16-bit/44.1kHz" }, { value: "HI_RES_LOSSLESS", label: "Hi-Res", desc: "FLAC 24-bit/96kHz" }] as option}
            <button
                onclick={() =>
                    settingsState.setAudioQuality(option.value as any)}
                class="flex items-center justify-between p-3 bg-white/3 border {settingsState.audioQuality ===
                option.value
                    ? 'border-indigo-500/50 bg-indigo-500/5'
                    : 'border-white/5'} rounded-xl hover:bg-white/6 transition-all text-left"
            >
                <div>
                    <div class="text-sm font-semibold text-white">
                        {option.label}
                    </div>
                    <div class="text-[10px] text-gray-500">
                        {option.desc}
                    </div>
                </div>
                {#if settingsState.audioQuality === option.value}
                    <div
                        class="w-5 h-5 bg-indigo-500 rounded-full flex items-center justify-center text-white"
                    >
                        <Check size={12} />
                    </div>
                {/if}
            </button>
        {/each}
    </div>
</section>

<div class="h-px bg-white/5"></div>

<section class="space-y-4">
    <div class="flex items-center gap-2 mb-2">
        <Music2 size={16} class="text-green-400" />
        <h3 class="text-sm font-bold text-white uppercase tracking-widest">
            Download Quality
        </h3>
    </div>

    <p class="text-xs text-gray-400 mb-2">
        Select the quality for tracks saved to your local library.
    </p>

    <div class="grid grid-cols-2 gap-2">
        {#each [{ value: "LOW", label: "Low", desc: "AAC ~96kbps" }, { value: "HIGH", label: "High", desc: "AAC ~320kbps" }, { value: "LOSSLESS", label: "Lossless", desc: "FLAC 16-bit/44.1kHz" }, { value: "HI_RES_LOSSLESS", label: "Hi-Res", desc: "FLAC 24-bit/96kHz" }] as option}
            <button
                onclick={() =>
                    settingsState.setDownloadQuality(option.value as any)}
                class="flex items-center justify-between p-3 bg-white/3 border {settingsState.downloadQuality ===
                option.value
                    ? 'border-green-500/40 bg-green-500/5'
                    : 'border-white/5'} rounded-xl hover:bg-white/6 transition-all text-left"
            >
                <div>
                    <div class="text-sm font-semibold text-white">
                        {option.label}
                    </div>
                    <div class="text-[10px] text-gray-500">
                        {option.desc}
                    </div>
                </div>
                {#if settingsState.downloadQuality === option.value}
                    <div
                        class="w-5 h-5 bg-green-500 rounded-full flex items-center justify-center text-black"
                    >
                        <Check size={12} />
                    </div>
                {/if}
            </button>
        {/each}
    </div>
</section>
