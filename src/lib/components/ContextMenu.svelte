<script lang="ts">
    import { contextMenuState } from "$lib/state/context_menu.svelte";
    import { fade } from "svelte/transition";

    function handleOutsideClick(e: MouseEvent) {
        if (contextMenuState.visible) {
            contextMenuState.close();
        }
    }

    function handleKeydown(e: KeyboardEvent) {
        if (e.key === "Escape" && contextMenuState.visible) {
            contextMenuState.close();
        }
    }
</script>

<svelte:window onmousedown={handleOutsideClick} onkeydown={handleKeydown} />

{#if contextMenuState.visible}
    <div
        id="global-context-menu"
        transition:fade={{ duration: 100 }}
        class="fixed z-[9999] min-w-[180px] bg-black/98 border border-surface-border rounded-xl shadow-2xl p-1.5 backdrop-blur-xl"
        style="left: {contextMenuState.x}px; top: {contextMenuState.y}px;"
        onmousedown={(e) => e.stopPropagation()}
        oncontextmenu={(e) => e.preventDefault()}
        role="menu"
        tabindex={-1}
    >
        <div class="flex flex-col gap-0.5">
            {#each contextMenuState.items as item}
                <button
                    class="flex items-center gap-3 w-full px-2.5 py-2 rounded-lg text-sm transition-colors text-left
                    {item.variant === 'danger'
                        ? 'text-red-400 hover:bg-red-500/10 hover:text-red-300'
                        : 'text-text-secondary hover:bg-surface-hover hover:text-text-primary'}"
                    onclick={() => {
                        item.onclick();
                        contextMenuState.close();
                    }}
                >
                    {#if item.icon}
                        <item.icon size={16} />
                    {/if}
                    <span class="flex-1 truncate">{item.label}</span>
                </button>
            {/each}
        </div>
    </div>
{/if}
