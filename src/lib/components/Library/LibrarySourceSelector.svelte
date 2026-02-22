<script lang="ts">
    import { uiState } from "$lib/state/player.svelte";
    import { HardDrive, Globe, ChevronDown } from "@lucide/svelte";
    import { fade, scale } from "svelte/transition";

    let isOpen = $state(false);

    const options = [
        { id: "local", label: "Local Library", icon: HardDrive },
        { id: "online", label: "Online Library", icon: Globe },
    ] as const;

    const currentOption = $derived(
        options.find((o) => o.id === uiState.libraryMode) || options[0],
    );

    function selectOption(id: "local" | "online") {
        uiState.setLibraryMode(id);
        isOpen = false;
    }

    function handleClickOutside(event: MouseEvent) {
        if (
            isOpen &&
            !(event.target as HTMLElement).closest(".source-selector")
        ) {
            isOpen = false;
        }
    }
</script>

<svelte:window onclick={handleClickOutside} />

<div class="relative source-selector">
    <button
        onclick={() => (isOpen = !isOpen)}
        class="flex items-center gap-2.5 px-4 py-2 bg-white/5 hover:bg-white/10 border border-white/10 rounded-xl transition-all duration-200 group"
    >
        {#key currentOption.id}
            {@const Icon = currentOption.icon}
            <Icon
                size={16}
                class="text-indigo-400 group-hover:text-indigo-300 transition-colors"
            />
        {/key}
        <span
            class="text-sm font-semibold text-gray-200 group-hover:text-white"
        >
            {currentOption.label}
        </span>
        <ChevronDown
            size={14}
            class="text-gray-500 group-hover:text-gray-400 transition-transform duration-200 {isOpen
                ? 'rotate-180'
                : ''}"
        />
    </button>

    {#if isOpen}
        <div
            in:scale={{ duration: 200, start: 0.95, opacity: 0 }}
            out:fade={{ duration: 150 }}
            class="absolute top-full left-0 mt-2 w-56 bg-[#1a1a1a] border border-white/10 rounded-2xl shadow-2xl overflow-hidden z-[100] backdrop-blur-xl"
        >
            <div class="p-1.5 flex flex-col gap-1">
                {#each options as option}
                    {@const OptionIcon = option.icon}
                    <button
                        onclick={() => selectOption(option.id)}
                        class="flex items-center gap-3 w-full px-3 py-2.5 rounded-xl transition-all duration-200 {uiState.libraryMode ===
                        option.id
                            ? 'bg-indigo-500 text-white shadow-lg shadow-indigo-500/20'
                            : 'text-gray-400 hover:text-white hover:bg-white/5'}"
                    >
                        <OptionIcon size={18} />
                        <span class="text-sm font-medium">{option.label}</span>
                    </button>
                {/each}
            </div>
        </div>
    {/if}
</div>

<style>
    .source-selector {
        user-select: none;
    }
</style>
