<script lang="ts">
    import { settingsState } from "$lib/state/settings.svelte";
    import {
        X,
        Settings,
    } from "@lucide/svelte";
    import GeneralSettings from "./views/GeneralSettings.svelte";
    import MonochromeSettings from "./views/MonochromeSettings.svelte";
    import { open } from "@tauri-apps/plugin-dialog";
    import { fade, scale } from "svelte/transition";

    let activeTab = $state<"koda" | "monochrome">("koda");

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
            class="relative w-full max-w-2xl bg-[#0c0c0e] border border-white/10 rounded-3xl shadow-2xl overflow-hidden flex flex-col h-[600px] max-h-[85vh]"
            transition:scale={{ duration: 300, start: 0.95 }}
        >
            <div
                class="px-8 pt-6 pb-0 border-b border-white/5 flex flex-col bg-white/2"
            >
                <div class="flex items-center justify-between mb-6">
                    <div class="flex items-center gap-4">
                        <div
                            class="w-12 h-12 bg-indigo-500/10 rounded-2xl flex items-center justify-center border border-indigo-500/20 text-indigo-400"
                        >
                            <Settings size={24} />
                        </div>
                        <div>
                            <h2
                                class="text-xl font-bold text-white leading-none"
                            >
                                Settings
                            </h2>
                        </div>
                    </div>
                    <button
                        onclick={close}
                        class="p-2 text-gray-400 hover:text-white hover:bg-white/5 rounded-xl transition-all"
                    >
                        <X size={24} />
                    </button>
                </div>

                <div class="flex gap-6">
                    <button
                        class="pb-3 text-sm font-bold transition-colors relative {activeTab ===
                        'koda'
                            ? 'text-white'
                            : 'text-gray-500 hover:text-gray-300'}"
                        onclick={() => (activeTab = "koda")}
                    >
                        General
                        {#if activeTab === "koda"}
                            <div
                                class="absolute bottom-0 left-0 right-0 h-0.5 bg-indigo-500 rounded-t-full"
                            ></div>
                        {/if}
                    </button>
                    <button
                        class="pb-3 text-sm font-bold transition-colors relative {activeTab ===
                        'monochrome'
                            ? 'text-white'
                            : 'text-gray-500 hover:text-gray-300'}"
                        onclick={() => (activeTab = "monochrome")}
                    >
                        Monochrome
                        {#if activeTab === "monochrome"}
                            <div
                                class="absolute bottom-0 left-0 right-0 h-0.5 bg-indigo-500 rounded-t-full"
                            ></div>
                        {/if}
                    </button>
                </div>
            </div>

            <div class="p-8 overflow-y-auto custom-scrollbar space-y-8 flex-1">
                {#if activeTab === "koda"}
                    <GeneralSettings />
                {:else if activeTab === "monochrome"}
                    <MonochromeSettings />
                {/if}
            </div>

            <div
                class="px-8 py-5 border-t border-white/5 bg-white/1 flex items-center justify-end"
            >
                <button
                    onclick={close}
                    class="px-6 py-2.5 bg-indigo-500 hover:bg-indigo-400 text-white text-sm font-bold rounded-xl transition-all active:scale-95"
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
