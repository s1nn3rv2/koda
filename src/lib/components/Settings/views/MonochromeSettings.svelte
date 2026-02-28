<script lang="ts">
    import { settingsState } from "$lib/state/settings.svelte";
    import { Globe, Server, Check } from "@lucide/svelte";

    let customUrl = $state(settingsState.monochromeInstance);
</script>

<section class="space-y-4">
    <div class="flex items-center gap-2 mb-2">
        <Globe size={16} class="text-indigo-400" />
        <h3 class="text-sm font-bold text-white uppercase tracking-widest">
            Monochrome API
        </h3>
    </div>

    <div class="grid gap-3">
        <p class="text-xs text-gray-400 mb-2">
            Choose an API instance for the Online Library. Instances are fetched
            live from status trackers.
        </p>

        <div class="space-y-2">
            <div class="flex items-center justify-between">
                <span class="text-[10px] font-bold text-gray-500 uppercase tracking-tighter">
                    {settingsState.isLoadingInstances ? "Discovering Instances..." : "Live Instances"}
                </span>
                {#if settingsState.isLoadingInstances}
                    <div class="w-3 h-3 border-2 border-indigo-500/30 border-t-indigo-500 rounded-full animate-spin"></div>
                {/if}
            </div>

            {#if settingsState.dynamicInstances.length === 0 && !settingsState.isLoadingInstances}
                <div class="p-6 bg-white/2 border border-white/5 rounded-2xl text-center">
                    <p class="text-xs text-gray-500">
                        Failed to fetch live instances. Check your connection or use
                        a custom URL.
                    </p>
                </div>
            {/if}

            <div class="grid grid-cols-1 sm:grid-cols-2 gap-2">
                {#each settingsState.dynamicInstances as instance}
                    <button
                        onclick={() => settingsState.setInstance(instance.url)}
                        class="flex items-center justify-between p-3 bg-white/2 border {settingsState.monochromeInstance === instance.url ? 'border-indigo-500/40 bg-indigo-500/5' : 'border-white/5'} rounded-xl hover:bg-white/5 transition-all text-left"
                    >
                        <div class="flex items-center gap-3 overflow-hidden">
                            <div class="relative">
                                <Server
                                    size={16}
                                    class={settingsState.monochromeInstance === instance.url ? "text-indigo-400" : "text-gray-600"}
                                />
                                {#if instance.isStreaming}
                                    <div class="absolute -top-1 -right-1 w-2 h-2 bg-green-500 rounded-full border border-[#0c0c0e]"></div>
                                {/if}
                            </div>
                            <div class="truncate">
                                <div class="text-xs font-semibold text-white truncate">
                                    {instance.url.replace("https://", "")}
                                </div>
                                <div class="text-[9px] text-gray-500 truncate flex items-center gap-1">
                                    v{instance.version}
                                    {#if instance.isStreaming}
                                        • <span class="text-green-500/60 font-medium">Streaming OK</span>
                                    {/if}
                                </div>
                            </div>
                        </div>
                        {#if settingsState.monochromeInstance === instance.url}
                            <Check size={14} class="text-indigo-400 shrink-0" />
                        {/if}
                    </button>
                {/each}
            </div>
        </div>

        <div class="space-y-2 mt-4">
            <span class="text-[10px] font-bold text-gray-500 uppercase tracking-tighter">Custom API URL</span>
            <div class="flex gap-2">
                <input
                    type="text"
                    bind:value={customUrl}
                    placeholder="https://your-api-instance.com"
                    class="flex-1 px-4 py-2.5 bg-white/3 border border-white/10 rounded-xl text-sm placeholder-gray-600 focus:outline-none focus:border-indigo-500/50 transition-all"
                />
                <button
                    onclick={() => settingsState.setInstance(customUrl)}
                    disabled={!customUrl || settingsState.monochromeInstance === customUrl}
                    class="px-4 py-2 bg-white/5 hover:bg-white/10 disabled:opacity-30 disabled:hover:bg-white/5 rounded-xl text-xs font-bold text-white transition-all border border-white/5"
                >
                    Apply
                </button>
            </div>
        </div>
    </div>
</section>
