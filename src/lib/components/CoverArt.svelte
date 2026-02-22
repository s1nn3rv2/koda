<script lang="ts" module>
    let sharedObserver: IntersectionObserver | null = null;
    const pendingLoads = new Map<HTMLElement, () => void>();

    const coverCache = new Map<string, string>();

    function getObserver() {
        if (!sharedObserver && typeof window !== "undefined") {
            sharedObserver = new IntersectionObserver(
                (entries) => {
                    entries.forEach((entry) => {
                        if (entry.isIntersecting) {
                            const loadFn = pendingLoads.get(
                                entry.target as HTMLElement,
                            );
                            if (loadFn) {
                                loadFn();
                                pendingLoads.delete(
                                    entry.target as HTMLElement,
                                );
                                sharedObserver?.unobserve(entry.target);
                            }
                        }
                    });
                },
                { rootMargin: "100px", threshold: 0.01 },
            );
        }
        return sharedObserver;
    }
</script>

<script lang="ts">
    import { LoaderCircle, Music } from "@lucide/svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount, onDestroy } from "svelte";
    import { monochromeService } from "$lib/services/monochrome";

    interface Props {
        trackId?: string;
        imageHash?: string | null;
        alt?: string;
        class?: string;
        size?: number;
        oncontextmenu?: (e: MouseEvent) => void;
        placeholderText?: string | null;
    }

    let {
        trackId,
        imageHash = null,
        alt = "Album cover",
        class: className = "",
        size = 128,
        oncontextmenu,
        placeholderText = null,
    }: Props = $props();

    let coverUrl = $state<string | null>(null);
    let isLoading = $state(false);
    let hasError = $state(false);
    let imgElement = $state<HTMLDivElement>();
    let hasLoaded = $state(false);
    let loadTimer: any = null;

    $effect(() => {
        trackId;
        imageHash;
        coverUrl = null;
        isLoading = false;
        hasError = false;
        hasLoaded = false;

        const cacheKey = imageHash
            ? `hash-${imageHash}-${size}`
            : `${trackId}-${size}`;
        if (coverCache.has(cacheKey)) {
            coverUrl = coverCache.get(cacheKey)!;
            hasLoaded = true;
            return;
        }

        if (imgElement) {
            const observer = getObserver();
            if (observer) {
                pendingLoads.set(imgElement, () => {
                    loadCoverArt();
                });
                observer.observe(imgElement);
            } else {
                loadCoverArt();
            }
        }
    });

    onDestroy(() => {
        if (imgElement) {
            pendingLoads.delete(imgElement);
            sharedObserver?.unobserve(imgElement);
        }
    });

    async function loadCoverArt() {
        if (isLoading || hasLoaded) return;
        if (!trackId && !imageHash) return;

        const cacheKey = imageHash
            ? `hash-${imageHash}-${size}`
            : `${trackId}-${size}`;
        if (coverCache.has(cacheKey)) {
            coverUrl = coverCache.get(cacheKey)!;
            hasLoaded = true;
            return;
        }

        isLoading = true;
        hasLoaded = true;
        hasError = false;

        try {
            if (imageHash) {
                let url;
                if (imageHash.startsWith("online-cover:")) {
                    const onlineId = parseInt(imageHash.split(":")[1]);
                    const fetchedUrl = await monochromeService.getCoverUrl(
                        onlineId,
                        size,
                    );
                    if (fetchedUrl) {
                        url = fetchedUrl;
                    } else {
                        hasError = true;
                        isLoading = false;
                        return;
                    }
                } else {
                    url = imageHash.startsWith("http")
                        ? imageHash
                        : `library-asset://localhost/${imageHash}/${size}`;
                }

                coverUrl = url;
                if (coverCache.size < 100) {
                    coverCache.set(cacheKey, url);
                }
            } else if (trackId) {
                const base64Data = await invoke<string | null>(
                    "get_cover_art",
                    {
                        id: trackId,
                        size,
                    },
                );
                if (base64Data) {
                    const dataUrl = `data:image/jpeg;base64,${base64Data}`;
                    coverUrl = dataUrl;
                    if (coverCache.size < 100) {
                        coverCache.set(cacheKey, dataUrl);
                    }
                } else {
                    hasError = true;
                }
            }
        } catch (e) {
            console.error("Failed to load cover art:", e);
            hasError = true;
        } finally {
            isLoading = false;
        }
    }
</script>

<div
    bind:this={imgElement}
    {oncontextmenu}
    role="img"
    class={`overflow-hidden bg-surface-base ${className} transition-colors duration-300`}
>
    {#if coverUrl}
        <img
            src={coverUrl}
            {alt}
            class="h-full w-full object-cover animate-in fade-in duration-500"
            loading="lazy"
        />
    {:else if isLoading}
        <div
            class="flex h-full w-full items-center justify-center text-text-dim/20"
        >
            <LoaderCircle size={18} class="animate-spin" />
        </div>
    {:else if placeholderText}
        <div
            class="flex h-full w-full items-center justify-center text-text-dim/40"
        >
            <span class="text-xs uppercase font-bold tracking-widest"
                >{placeholderText}</span
            >
        </div>
    {:else}
        <div
            class="flex h-full w-full items-center justify-center text-text-dim/10"
        >
            <Music size={size > 128 ? 48 : 18} />
        </div>
    {/if}
</div>
