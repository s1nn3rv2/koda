<script lang="ts">
    import { LoaderCircle, Music } from "@lucide/svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    interface Props {
        trackId: string;
        alt?: string;
        class?: string;
        isExpanded: boolean;
    }

    let {
        trackId,
        alt = "Album cover",
        class: className = "",
        isExpanded,
    }: Props = $props();

    let thumbnailUrl = $state<string | null>(null);
    let fullResUrl = $state<string | null>(null);
    let isLoadingThumbnail = $state(false);
    let isLoadingFullRes = $state(false);
    let hasError = $state(false);

    const currentUrl = $derived(
        isExpanded && fullResUrl ? fullResUrl : thumbnailUrl,
    );
    const isLoading = $derived(isLoadingThumbnail || isLoadingFullRes);

    onMount(() => {
        loadThumbnail();
    });

    // load high res when expanded
    $effect(() => {
        if (isExpanded && !fullResUrl && !isLoadingFullRes) {
            loadFullRes();
        }
    });

    async function loadThumbnail() {
        if (isLoadingThumbnail || thumbnailUrl) return;

        isLoadingThumbnail = true;
        hasError = false;

        try {
            const base64Data = await invoke<string | null>("get_cover_art", {
                id: trackId,
                size: 128,
            });

            if (base64Data) {
                thumbnailUrl = `data:image/jpeg;base64,${base64Data}`;
            }
        } catch (e) {
            console.error("Failed to load thumbnail:", e);
            hasError = true;
        } finally {
            isLoadingThumbnail = false;
        }
    }

    async function loadFullRes() {
        if (isLoadingFullRes || fullResUrl) return;

        isLoadingFullRes = true;

        try {
            const base64Data = await invoke<string | null>("get_cover_art", {
                id: trackId,
                size: 1024,
            });

            if (base64Data) {
                fullResUrl = `data:image/jpeg;base64,${base64Data}`;
            }
        } catch (e) {
            console.error("Failed to load full-res cover art:", e);
        } finally {
            isLoadingFullRes = false;
        }
    }
</script>

<div class={`overflow-hidden bg-white/10 ${className}`}>
    {#if currentUrl}
        <img
            src={currentUrl}
            {alt}
            class="h-full w-full object-cover transition-opacity duration-300"
            loading="lazy"
        />
    {:else if isLoading}
        <div
            class="flex h-full w-full items-center justify-center text-white/30"
        >
            <LoaderCircle size={24} class="animate-spin" />
        </div>
    {:else if hasError}
        <div
            class="flex h-full w-full items-center justify-center text-white/30"
        >
            <Music />
        </div>
    {:else}
        <Music />
    {/if}
</div>
