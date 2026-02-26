<script lang="ts">
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import { Minus, Square, X, Copy, Settings } from "@lucide/svelte";
    import { onMount } from "svelte";
    import { settingsState } from "$lib/state/settings.svelte";

    const appWindow = getCurrentWindow();
    let isMaximized = $state(false);

    onMount(() => {
        let unlisten: () => void;

        async function setup() {
            isMaximized = await appWindow.isMaximized();
            unlisten = await appWindow.onResized(async () => {
                isMaximized = await appWindow.isMaximized();
            });
        }

        setup();

        return () => {
            if (unlisten) unlisten();
        };
    });

    const minimize = () => appWindow.minimize();
    const toggleMaximize = () => appWindow.toggleMaximize();
    const close = () => appWindow.close();
</script>

<div
    class="flex items-center justify-between h-8 bg-zinc-950/40 backdrop-blur-xl select-none border-b border-white/3 sticky top-0 z-[100]"
>
    <div
        data-tauri-drag-region
        class="flex-1 h-full flex items-center px-4 gap-2"
    >
        <span
            class="text-[10px] font-bold tracking-[0.2em] text-indigo-400/80 uppercase"
        >
            Koda
        </span>
    </div>

    <div class="flex items-center h-full">
        <button
            onclick={() => settingsState.toggleSettings()}
            class="flex items-center justify-center w-10 h-full hover:bg-white/5 transition-colors text-zinc-400 hover:text-zinc-200"
            title="settings"
        >
            <Settings size={14} />
        </button>
        <div class="w-px h-3 bg-white/10 mx-1"></div>
        <button
            onclick={minimize}
            class="flex items-center justify-center w-10 h-full hover:bg-white/5 transition-colors text-zinc-400 hover:text-zinc-200"
            title="minimize"
        >
            <Minus size={16} />
        </button>
        <button
            onclick={toggleMaximize}
            class="flex items-center justify-center w-10 h-full hover:bg-white/5 transition-colors text-zinc-400 hover:text-zinc-200"
            title={isMaximized ? "restore" : "maximize"}
        >
            {#if isMaximized}
                <Copy size={16} />
            {:else}
                <Square size={16} />
            {/if}
        </button>
        <button
            onclick={close}
            class="flex items-center justify-center w-11 h-full hover:bg-red-500/80 transition-colors text-zinc-400 hover:text-white"
            title="close"
        >
            <X size={20} />
        </button>
    </div>
</div>

<style>
    button {
        -webkit-app-region: no-drag;
    }
</style>
