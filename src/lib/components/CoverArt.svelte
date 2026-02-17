<script lang="ts">
    import { LoaderCircle, Music } from "@lucide/svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    interface Props {
        trackId: string;
        alt?: string;
        class?: string;
        size?: number;
    }

    let {
        trackId,
        alt = "Album cover",
        class: className = "",
        size = 128,
    }: Props = $props();

    let coverUrl = $state<string | null>(null);
    let isLoading = $state(false);
    let hasError = $state(false);
    let imgElement: HTMLDivElement | undefined = $state();
    let hasLoaded = $state(false);

    onMount(() => {
        if (!imgElement) return;

        const observer = new IntersectionObserver(
            (entries) => {
                entries.forEach((entry) => {
                    if (entry.isIntersecting && !hasLoaded) {
                        loadCoverArt();
                    }
                });
            },
            {
                rootMargin: "50px",
                threshold: 0.01,
            },
        );

        observer.observe(imgElement);

        return () => {
            observer.disconnect();
        };
    });

    async function loadCoverArt() {
        if (isLoading || hasLoaded) return;

        isLoading = true;
        hasLoaded = true;
        hasError = false;

        try {
            const base64Data = await invoke<string | null>("get_cover_art", {
                id: trackId,
                size,
            });

            if (base64Data) {
                coverUrl = `data:image/jpeg;base64,${base64Data}`;
            } else {
                hasError = true;
            }
        } catch (e) {
            console.error("Failed to load cover art:", e);
            hasError = true;
        } finally {
            isLoading = false;
        }
    }
</script>

<div bind:this={imgElement} class={`overflow-hidden bg-white/10 ${className}`}>
    {#if coverUrl}
        <img
            src={coverUrl}
            {alt}
            class="h-full w-full object-cover"
            loading="lazy"
        />
    {:else if isLoading}
        <div
            class="flex h-full w-full items-center justify-center text-white/30"
        >
            <LoaderCircle size={24} class="animate-spin" />
        </div>
    {:else}
        <Music />
    {/if}
</div>
