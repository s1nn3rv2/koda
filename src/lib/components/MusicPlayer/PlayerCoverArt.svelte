<script lang="ts">
    import { LoaderCircle, Music } from "@lucide/svelte";
    import { TauriService } from "$lib/utils/tauri";
    import { onMount } from "svelte";
    import { getDominantColor } from "$lib/utils/color";
    import { playbackState } from "$lib/state/player.svelte";
    import { monochromeService } from "$lib/services/monochrome";

    interface Props {
        trackId: string;
        imageHash?: string | null;
        alt?: string;
        class?: string;
        isExpanded: boolean;
    }

    let {
        trackId,
        imageHash = null,
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
            if (imageHash) {
                let url: string;
                if (imageHash.startsWith("online-cover:")) {
                    const onlineId = parseInt(imageHash.split(":")[1]);
                    const fetchedUrl = await monochromeService.getCoverUrl(
                        onlineId,
                        128,
                    );
                    if (fetchedUrl) {
                        url = fetchedUrl;
                    } else {
                        hasError = true;
                        isLoadingThumbnail = false;
                        return;
                    }
                } else {
                    url = imageHash.startsWith("http")
                        ? imageHash
                        : `library-asset://localhost/${imageHash}/128`;
                }

                thumbnailUrl = url;

                let colorUrl = url;
                if (
                    !imageHash.startsWith("online-cover:") &&
                    !imageHash.startsWith("http")
                ) {
                    const base64Data = await TauriService.getImageByHash(imageHash, 128);
                    if (base64Data) {
                        colorUrl = `data:image/jpeg;base64,${base64Data}`;
                    }
                } else if (url.startsWith("http")) {
                    try {
                        const base64Data = await TauriService.getImageFromUrl(url);
                        if (base64Data) {
                            colorUrl = `data:image/jpeg;base64,${base64Data}`;
                        }
                    } catch (e) {
                        console.error(
                            "Failed to proxy online image for color extraction:",
                            e,
                        );
                    }
                }

                getDominantColor(colorUrl).then((color) => {
                    if (playbackState.currentTrackId === trackId) {
                        playbackState.dominantColor = color;
                    }
                });
            } else {
                const base64Data = await TauriService.getCoverArt(trackId, 128);

                if (base64Data) {
                    const url = `data:image/jpeg;base64,${base64Data}`;
                    thumbnailUrl = url;

                    getDominantColor(url).then((color) => {
                        if (playbackState.currentTrackId === trackId) {
                            playbackState.dominantColor = color;
                        }
                    });
                } else {
                    if (playbackState.currentTrackId === trackId) {
                        playbackState.dominantColor = null;
                    }
                }
            }
        } catch (e) {
            console.error("Failed to load thumbnail:", e);
            hasError = true;
            if (playbackState.currentTrackId === trackId) {
                playbackState.dominantColor = null;
            }
        } finally {
            isLoadingThumbnail = false;
        }
    }

    async function loadFullRes() {
        if (isLoadingFullRes || fullResUrl) return;

        isLoadingFullRes = true;

        try {
            if (imageHash) {
                if (imageHash.startsWith("online-cover:")) {
                    const onlineId = parseInt(imageHash.split(":")[1]);
                    const fetchedUrl = await monochromeService.getCoverUrl(
                        onlineId,
                        1024,
                    );
                    if (fetchedUrl) {
                        fullResUrl = fetchedUrl;
                    } else {
                        isLoadingFullRes = false;
                        return;
                    }
                } else {
                    fullResUrl = imageHash.startsWith("http")
                        ? imageHash
                        : `library-asset://localhost/${imageHash}/1024`;
                }
            } else {
                const base64Data = await TauriService.getCoverArt(trackId, 1024);

                if (base64Data) {
                    fullResUrl = `data:image/jpeg;base64,${base64Data}`;
                }
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
