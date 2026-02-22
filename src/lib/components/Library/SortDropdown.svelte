<script lang="ts">
    import {
        ChevronDown,
        ArrowUp,
        ArrowDown,
        ListFilter,
    } from "@lucide/svelte";

    export type SortColumn =
        | "added_at"
        | "release_date"
        | "title"
        | "duration"
        | "track_count"
        | "name"
        | "default";
    export type SortDirection = "asc" | "desc";

    interface SortOption {
        label: string;
        column: SortColumn;
        defaultDir: SortDirection;
    }

    let {
        options = [],
        column = $bindable("default"),
        direction = $bindable("desc"),
    }: {
        options: SortOption[];
        column: SortColumn;
        direction: SortDirection;
    } = $props();

    let isOpen = $state(false);

    function selectOption(opt: SortOption) {
        if (column === opt.column) {
            direction = direction === "asc" ? "desc" : "asc";
        } else {
            column = opt.column;
            direction = opt.defaultDir;
        }
        isOpen = false;
    }

    const activeOption = $derived(
        options.find((o) => o.column === column) || options[0],
    );
</script>

<div class="relative block z-[40]">
    <button
        class="flex items-center gap-2 text-sm text-gray-400 hover:text-white transition-colors py-1.5 px-3 rounded-md hover:bg-white/5"
        onclick={() => (isOpen = !isOpen)}
    >
        <ListFilter size={14} class="mr-1 opacity-70" />
        {activeOption?.label || "Sort By"}
        {#if column !== "default"}
            {#if direction === "asc"}
                <ArrowUp size={14} class="ml-1" />
            {:else}
                <ArrowDown size={14} class="ml-1" />
            {/if}
        {/if}
        <ChevronDown size={14} class="opacity-50" />
    </button>

    {#if isOpen}
        <div
            class="absolute right-0 mt-2 w-48 bg-gray-900 border border-white/10 rounded-lg shadow-xl overflow-hidden py-1 z-50 text-sm"
        >
            {#each options as opt}
                <button
                    class="w-full text-left px-4 py-2 hover:bg-white/10 transition-colors flex items-center justify-between {column ===
                    opt.column
                        ? 'text-white bg-white/5'
                        : 'text-gray-400'}"
                    onclick={() => selectOption(opt)}
                >
                    {opt.label}
                    {#if column === opt.column}
                        {#if direction === "asc"}
                            <ArrowUp size={14} />
                        {:else}
                            <ArrowDown size={14} />
                        {/if}
                    {/if}
                </button>
            {/each}
        </div>

        <button
            class="fixed inset-0 w-full h-full cursor-default bg-transparent z-40"
            onclick={() => (isOpen = false)}
            tabindex="-1"
            aria-label="Close sorting menu"
        ></button>
    {/if}
</div>
