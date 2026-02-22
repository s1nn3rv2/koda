<script lang="ts">
    import { settingsState } from "$lib/state/settings.svelte";
    import {
        X,
        Globe,
        Zap,
        Server,
        ShieldCheck,
        Check,
        Music2,
        FolderPlus,
        Trash2,
        HardDrive,
    } from "@lucide/svelte";
    import { open } from "@tauri-apps/plugin-dialog";
    import { fade, scale } from "svelte/transition";

    let customUrl = $state(settingsState.monochromeInstance);

    $effect(() => {
        if (settingsState.isSettingsOpen) {
            settingsState.fetchDynamicInstances();
        }
    });

    function close() {
        settingsState.isSettingsOpen = false;
    }

    function handleKeyDown(e: KeyboardEvent) {
        if (e.key === "Escape") close();
    }

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

{#if settingsState.isSettingsOpen}
    <div
        class="fixed inset-0 z-[10000] flex items-center justify-center p-4"
        transition:fade={{ duration: 200 }}
        onkeydown={handleKeyDown}
        role="dialog"
        aria-modal="true"
        tabindex="-1"
    >
        <div
            class="absolute inset-0 bg-black/60 backdrop-blur-sm"
            onclick={close}
            onkeydown={(e) => e.key === "Enter" && close()}
            role="button"
            aria-label="Close settings"
            tabindex="0"
        ></div>

        <div
            class="relative w-full max-w-2xl bg-[#0c0c0e] border border-white/10 rounded-3xl shadow-2xl overflow-hidden flex flex-col max-h-[85vh]"
            transition:scale={{ duration: 300, start: 0.95 }}
        >
            <div
                class="px-8 py-6 border-b border-white/5 flex items-center justify-between bg-white/[0.02]"
            >
                <div class="flex items-center gap-4">
                    <div
                        class="w-12 h-12 bg-indigo-500/10 rounded-2xl flex items-center justify-center border border-indigo-500/20 text-indigo-400"
                    >
                        <Zap size={24} />
                    </div>
                    <div>
                        <h2 class="text-xl font-bold text-white leading-none">
                            Settings
                        </h2>
                        <p class="text-sm text-gray-500 mt-1.5">
                            Configure Koda and Monochrome instances
                        </p>
                    </div>
                </div>
                <button
                    onclick={close}
                    class="p-2 text-gray-400 hover:text-white hover:bg-white/5 rounded-xl transition-all"
                >
                    <X size={24} />
                </button>
            </div>

            <div class="p-8 overflow-y-auto custom-scrollbar space-y-8">
                <section class="space-y-4">
                    <div class="flex items-center justify-between mb-2">
                        <div class="flex items-center gap-2">
                            <HardDrive size={16} class="text-indigo-400" />
                            <h3
                                class="text-sm font-bold text-white uppercase tracking-widest"
                            >
                                Local Library
                            </h3>
                        </div>
                        <button
                            onclick={addFolder}
                            class="flex items-center gap-2 px-3 py-1.5 bg-indigo-500 hover:bg-indigo-400 text-white text-[10px] font-bold rounded-lg transition-all shadow-lg shadow-indigo-500/10 active:scale-95"
                        >
                            <FolderPlus size={14} />
                            Add Folder
                        </button>
                    </div>

                    <p class="text-xs text-gray-400 mb-2">
                        Koda will scan these folders for audio files. You can
                        add multiple locations.
                    </p>

                    <div class="space-y-2">
                        {#if settingsState.musicPaths.length === 0}
                            <div
                                class="p-8 bg-white/[0.02] border border-white/5 border-dashed rounded-2xl text-center"
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
                                        class="flex items-center justify-between p-3 bg-white/[0.03] border border-white/5 rounded-xl group"
                                    >
                                        <div
                                            class="flex items-center gap-3 overflow-hidden"
                                        >
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
                                            onclick={() =>
                                                settingsState.removeMusicPath(
                                                    path,
                                                )}
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

                <div class="h-px bg-white/5"></div>
                <section class="space-y-4">
                    <div class="flex items-center gap-2 mb-2">
                        <Globe size={16} class="text-indigo-400" />
                        <h3
                            class="text-sm font-bold text-white uppercase tracking-widest"
                        >
                            Monochrome API
                        </h3>
                    </div>

                    <div class="grid gap-3">
                        <p class="text-xs text-gray-400 mb-2">
                            Choose an API instance for the Online Library.
                            Instances are fetched live from status trackers.
                        </p>

                        <div class="space-y-2">
                            <div class="flex items-center justify-between">
                                <span
                                    class="text-[10px] font-bold text-gray-500 uppercase tracking-tighter"
                                >
                                    {settingsState.isLoadingInstances
                                        ? "Discovering Instances..."
                                        : "Live Instances"}
                                </span>
                                {#if settingsState.isLoadingInstances}
                                    <div
                                        class="w-3 h-3 border-2 border-indigo-500/30 border-t-indigo-500 rounded-full animate-spin"
                                    ></div>
                                {/if}
                            </div>

                            {#if settingsState.dynamicInstances.length === 0 && !settingsState.isLoadingInstances}
                                <div
                                    class="p-6 bg-white/[0.02] border border-white/5 rounded-2xl text-center"
                                >
                                    <p class="text-xs text-gray-500">
                                        Failed to fetch live instances. Check
                                        your connection or use a custom URL.
                                    </p>
                                </div>
                            {/if}

                            <div class="grid grid-cols-1 sm:grid-cols-2 gap-2">
                                {#each settingsState.dynamicInstances as instance}
                                    <button
                                        onclick={() =>
                                            settingsState.setInstance(
                                                instance.url,
                                            )}
                                        class="flex items-center justify-between p-3 bg-white/[0.02] border {settingsState.monochromeInstance ===
                                        instance.url
                                            ? 'border-indigo-500/40 bg-indigo-500/5'
                                            : 'border-white/5'} rounded-xl hover:bg-white/[0.05] transition-all text-left"
                                    >
                                        <div
                                            class="flex items-center gap-3 overflow-hidden"
                                        >
                                            <div class="relative">
                                                <Server
                                                    size={16}
                                                    class={settingsState.monochromeInstance ===
                                                    instance.url
                                                        ? "text-indigo-400"
                                                        : "text-gray-600"}
                                                />
                                                {#if instance.isStreaming}
                                                    <div
                                                        class="absolute -top-1 -right-1 w-2 h-2 bg-green-500 rounded-full border border-[#0c0c0e]"
                                                    ></div>
                                                {/if}
                                            </div>
                                            <div class="truncate">
                                                <div
                                                    class="text-xs font-semibold text-white truncate"
                                                >
                                                    {instance.url.replace(
                                                        "https://",
                                                        "",
                                                    )}
                                                </div>
                                                <div
                                                    class="text-[9px] text-gray-500 truncate flex items-center gap-1"
                                                >
                                                    v{instance.version}
                                                    {#if instance.isStreaming}
                                                        • <span
                                                            class="text-green-500/60 font-medium"
                                                            >Streaming OK</span
                                                        >
                                                    {/if}
                                                </div>
                                            </div>
                                        </div>
                                        {#if settingsState.monochromeInstance === instance.url}
                                            <Check
                                                size={14}
                                                class="text-indigo-400 shrink-0"
                                            />
                                        {/if}
                                    </button>
                                {/each}
                            </div>
                        </div>

                        <div class="space-y-2 mt-4">
                            <span
                                class="text-[10px] font-bold text-gray-500 uppercase tracking-tighter"
                                >Custom API URL</span
                            >
                            <div class="flex gap-2">
                                <input
                                    type="text"
                                    bind:value={customUrl}
                                    placeholder="https://your-api-instance.com"
                                    class="flex-1 px-4 py-2.5 bg-white/[0.03] border border-white/10 rounded-xl text-sm placeholder-gray-600 focus:outline-none focus:border-indigo-500/50 transition-all"
                                />
                                <button
                                    onclick={() =>
                                        settingsState.setInstance(customUrl)}
                                    disabled={!customUrl ||
                                        settingsState.monochromeInstance ===
                                            customUrl}
                                    class="px-4 py-2 bg-white/5 hover:bg-white/10 disabled:opacity-30 disabled:hover:bg-white/5 rounded-xl text-xs font-bold text-white transition-all border border-white/5"
                                >
                                    Apply
                                </button>
                            </div>
                        </div>
                    </div>
                </section>

                <section class="space-y-4">
                    <div class="flex items-center gap-2 mb-2">
                        <Music2 size={16} class="text-indigo-400" />
                        <h3
                            class="text-sm font-bold text-white uppercase tracking-widest"
                        >
                            Streaming Quality
                        </h3>
                    </div>

                    <p class="text-xs text-gray-400 mb-2">
                        Higher quality uses more bandwidth and takes longer to
                        load.
                    </p>

                    <div class="grid grid-cols-2 gap-2">
                        {#each [{ value: "LOW", label: "Low", desc: "AAC ~96kbps" }, { value: "HIGH", label: "High", desc: "AAC ~320kbps" }, { value: "LOSSLESS", label: "Lossless", desc: "FLAC 16-bit/44.1kHz" }, { value: "HI_RES_LOSSLESS", label: "Hi-Res", desc: "FLAC 24-bit/96kHz" }] as option}
                            <button
                                onclick={() =>
                                    settingsState.setAudioQuality(
                                        option.value as any,
                                    )}
                                class="flex items-center justify-between p-3 bg-white/[0.03] border {settingsState.audioQuality ===
                                option.value
                                    ? 'border-indigo-500/50 bg-indigo-500/5'
                                    : 'border-white/5'} rounded-xl hover:bg-white/[0.06] transition-all text-left"
                            >
                                <div>
                                    <div
                                        class="text-sm font-semibold text-white"
                                    >
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
                        <h3
                            class="text-sm font-bold text-white uppercase tracking-widest"
                        >
                            Download Quality
                        </h3>
                    </div>

                    <p class="text-xs text-gray-400 mb-2">
                        Select the quality for tracks saved to your local
                        library.
                    </p>

                    <div class="grid grid-cols-2 gap-2">
                        {#each [{ value: "LOW", label: "Low", desc: "AAC ~96kbps" }, { value: "HIGH", label: "High", desc: "AAC ~320kbps" }, { value: "LOSSLESS", label: "Lossless", desc: "FLAC 16-bit/44.1kHz" }, { value: "HI_RES_LOSSLESS", label: "Hi-Res", desc: "FLAC 24-bit/96kHz" }] as option}
                            <button
                                onclick={() =>
                                    settingsState.setDownloadQuality(
                                        option.value as any,
                                    )}
                                class="flex items-center justify-between p-3 bg-white/[0.03] border {settingsState.downloadQuality ===
                                option.value
                                    ? 'border-green-500/40 bg-green-500/5'
                                    : 'border-white/5'} rounded-xl hover:bg-white/[0.06] transition-all text-left"
                            >
                                <div>
                                    <div
                                        class="text-sm font-semibold text-white"
                                    >
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

                <section
                    class="p-6 bg-indigo-500/5 border border-indigo-500/10 rounded-2xl"
                >
                    <div class="flex gap-4">
                        <div
                            class="w-10 h-10 bg-indigo-500/20 rounded-xl flex items-center justify-center shrink-0"
                        >
                            <Music2 size={20} class="text-indigo-400" />
                        </div>
                        <div>
                            <h4 class="text-sm font-bold text-white">
                                Monochrome Integration
                            </h4>
                            <p
                                class="text-[11px] text-gray-400 mt-1 leading-relaxed"
                            >
                                Koda connects to Monochrome/Hi-Fi API instances
                                to provide online music discovery. Note that
                                streaming availability depends on the instance
                                and its current backend health.
                            </p>
                        </div>
                    </div>
                </section>
            </div>

            <div
                class="px-8 py-5 border-t border-white/5 bg-white/[0.01] flex items-center justify-between"
            >
                <div class="flex items-center gap-2">
                    <div
                        class="w-1.5 h-1.5 bg-green-500 rounded-full animate-pulse"
                    ></div>
                    <span class="text-[10px] text-gray-500 font-medium"
                        >Auto-saving enabled</span
                    >
                </div>
                <button
                    onclick={close}
                    class="px-6 py-2.5 bg-indigo-500 hover:bg-indigo-400 text-white text-sm font-bold rounded-xl transition-all shadow-lg shadow-indigo-500/20 active:scale-95"
                >
                    Done
                </button>
            </div>
        </div>
    </div>
{/if}

<style>
    .custom-scrollbar::-webkit-scrollbar {
        width: 6px;
    }
    .custom-scrollbar::-webkit-scrollbar-track {
        background: transparent;
    }
    .custom-scrollbar::-webkit-scrollbar-thumb {
        background: rgba(255, 255, 255, 0.05);
        border-radius: 10px;
    }
    .custom-scrollbar::-webkit-scrollbar-thumb:hover {
        background: rgba(255, 255, 255, 0.1);
    }
</style>
