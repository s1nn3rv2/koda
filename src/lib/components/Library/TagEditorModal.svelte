<script lang="ts">
    import { libraryState } from "$lib/state/library.svelte";
    import { TauriService } from "$lib/utils/tauri";
    import {
        X,
        Save,
        Music2,
        User,
        Disc,
        Calendar,
        Hash,
        Layers,
        SquarePen,
    } from "@lucide/svelte";
    import { fade, scale } from "svelte/transition";
    import type { Component } from "svelte";

    let track = $derived(libraryState.trackToEdit);

    let title = $state("");
    let artists = $state("");
    let album = $state("");
    let albumArtist = $state("");
    let trackNumber = $state<number | null>(null);
    let discNumber = $state<number | null>(null);
    let releaseDate = $state("");
    let genre = $state("");
    let isSaving = $state(false);

    const inputClass =
        "w-full px-4 py-2.5 bg-white/5 border border-white/10 rounded-xl text-white placeholder-gray-600 focus:outline-none focus:border-indigo-500/50 focus:ring-1 focus:ring-indigo-500/50 transition-all";
    const labelClass =
        "text-xs font-semibold text-gray-400 uppercase tracking-wider flex items-center gap-2";

    $effect(() => {
        if (track) {
            title = track.title || "";
            artists = track.artists || "";
            album = track.album || "";
            albumArtist = track.album_artist || "";
            trackNumber = track.track_number ?? null;
            discNumber = track.disc_number ?? null;
            releaseDate = track.release_date || "";
            genre = track.genre || "";
        }
    });

    async function handleSave() {
        if (!track) return;
        isSaving = true;
        try {
            await TauriService.updateTrackMetadata(track.id, {
                title: title || null,
                artists: artists || null,
                album: album || null,
                albumArtist: albumArtist || null,
                trackNumber,
                discNumber,
                releaseDate: releaseDate || null,
                genre: genre || null,
            });
            libraryState.refresh();
            close();
        } catch (e) {
            console.error("Failed to save metadata:", e);
        } finally {
            isSaving = false;
        }
    }

    function close() {
        libraryState.editTrack(null as any);
    }

    function handleKeyDown(e: KeyboardEvent) {
        if (e.key === "Escape") close();
        if (e.key === "Enter" && (e.ctrlKey || e.metaKey)) handleSave();
    }
</script>

{#snippet textField(
    Icon: Component,
    label: string,
    value: string,
    onInput: (v: string) => void,
    span2?: boolean,
    placeholder?: string,
)}
    <div class={span2 ? "col-span-2 space-y-2" : "space-y-2"}>
        <label class={labelClass}><Icon size={12} /> {label}</label>
        <input
            type="text"
            {value}
            oninput={(e) => onInput((e.target as HTMLInputElement).value)}
            {placeholder}
            class={inputClass}
        />
    </div>
{/snippet}

{#snippet numField(
    Icon: Component,
    label: string,
    value: number | null,
    onInput: (v: number | null) => void,
    placeholder?: string,
)}
    <div class="space-y-2">
        <label class={labelClass}><Icon size={12} /> {label}</label>
        <input
            type="number"
            value={value ?? ""}
            oninput={(e) => {
                const v = (e.target as HTMLInputElement).value;
                onInput(v ? Number(v) : null);
            }}
            {placeholder}
            class={inputClass}
        />
    </div>
{/snippet}

{#if track}
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
            aria-label="Close modal"
            tabindex="0"
        ></div>

        <div
            class="relative w-full max-w-2xl bg-[#121214] border border-white/10 rounded-2xl shadow-2xl overflow-hidden flex flex-col"
            transition:scale={{ duration: 300, start: 0.95 }}
        >
            <div
                class="px-6 py-4 border-b border-white/5 flex items-center justify-between bg-[#121214]/50 backdrop-blur-md sticky top-0 z-10"
            >
                <div class="flex items-center gap-3">
                    <div
                        class="w-10 h-10 bg-indigo-500/10 rounded-xl flex items-center justify-center border border-indigo-500/20 text-indigo-400"
                    >
                        <SquarePen size={20} />
                    </div>
                    <div>
                        <h2 class="text-lg font-bold text-white leading-none">
                            Edit Metadata
                        </h2>
                        <p
                            class="text-xs text-gray-400 mt-1 truncate max-w-[300px]"
                        >
                            {track.path.split("/").pop()}
                        </p>
                    </div>
                </div>
                <button
                    onclick={close}
                    class="p-2 text-gray-400 hover:text-white hover:bg-white/5 rounded-lg transition-colors"
                >
                    <X size={20} />
                </button>
            </div>

            <div class="p-6 overflow-y-auto max-h-[70vh] custom-scrollbar">
                <div class="grid grid-cols-2 gap-6">
                    {@render textField(
                        Music2,
                        "Track Title",
                        title,
                        (v) => (title = v),
                        true,
                        "Unknown Title",
                    )}
                    {@render textField(
                        User,
                        "Artists (separated with ;)",
                        artists,
                        (v) => (artists = v),
                        true,
                        "Unknown Artist",
                    )}
                    {@render textField(
                        Disc,
                        "Album Name",
                        album,
                        (v) => (album = v),
                        false,
                        "Unknown Album",
                    )}
                    {@render textField(
                        User,
                        "Album Artists",
                        albumArtist,
                        (v) => (albumArtist = v),
                        false,
                        "Unknown Album Artists",
                    )}
                    {@render textField(
                        Calendar,
                        "Release Date (YYYY-MM-DD)",
                        releaseDate,
                        (v) => (releaseDate = v),
                        false,
                        "YYYY-MM-DD",
                    )}
                    {@render textField(
                        Layers,
                        "Genre",
                        genre,
                        (v) => (genre = v),
                        false,
                        "Unknown Genre",
                    )}
                    {@render numField(
                        Hash,
                        "Track #",
                        trackNumber,
                        (v) => (trackNumber = v),
                        "0",
                    )}
                    {@render numField(
                        Layers,
                        "Disc #",
                        discNumber,
                        (v) => (discNumber = v),
                        "1",
                    )}
                </div>
            </div>

            <div
                class="px-6 py-4 bg-white/5 border-t border-white/5 flex items-center justify-between backdrop-blur-md"
            >
                <div class="flex items-center gap-3 ml-auto">
                    <button
                        onclick={close}
                        class="px-5 py-2 text-sm font-medium text-gray-400 hover:text-white transition-colors"
                        >Cancel</button
                    >
                    <button
                        onclick={handleSave}
                        disabled={isSaving}
                        class="px-3 py-2 bg-indigo-500 hover:bg-indigo-400 disabled:opacity-50 disabled:cursor-not-allowed text-white text-sm font-bold rounded-xl transition-all shadow-lg shadow-indigo-500/20 flex items-center gap-2 active:scale-95"
                    >
                        {#if isSaving}
                            <div
                                class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"
                            ></div>
                            Saving...
                        {:else}
                            <Save size={16} /> Save Changes
                        {/if}
                    </button>
                </div>
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
        background: rgba(255, 255, 255, 0.1);
        border-radius: 10px;
    }
    .custom-scrollbar::-webkit-scrollbar-thumb:hover {
        background: rgba(255, 255, 255, 0.2);
    }
</style>
