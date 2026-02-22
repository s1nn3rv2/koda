<script lang="ts">
    import { downloadState } from "$lib/state/download.svelte";
    import { settingsState } from "$lib/state/settings.svelte";
    import {
        X,
        Check,
        Save,
        Folder,
        Music,
        User,
        Disc,
        Calendar,
        Hash,
        Bookmark,
        ChevronRight,
        ChevronDown,
    } from "@lucide/svelte";
    import { fade, scale } from "svelte/transition";
    import { invoke } from "@tauri-apps/api/core";
    import { libraryState } from "$lib/state/library.svelte";

    let dl = $derived(downloadState.trackToImport);

    let title = $state("");
    let artists = $state("");
    let album = $state("");
    let albumArtist = $state("");
    let genre = $state("");
    let trackNumber = $state<number | null>(null);
    let discNumber = $state<number | null>(null);
    let releaseDate = $state("");
    let targetFolder = $state("");
    let rootPath = $state("");
    let treeData = $state<Record<string, string[]>>({});
    let expandedPaths = $state(new Set<string>());
    let isImporting = $state(false);
    let error = $state("");

    const inputClass =
        "w-full px-4 py-2.5 bg-white/[0.03] border border-white/10 rounded-xl text-sm text-white focus:outline-none focus:border-indigo-500/50 transition-all";
    const labelClass =
        "flex items-center gap-2 text-[10px] font-bold text-gray-500 uppercase tracking-widest px-1";

    async function toggleFolder(path: string) {
        if (expandedPaths.has(path)) {
            expandedPaths.delete(path);
            expandedPaths = new Set(expandedPaths);
        } else {
            expandedPaths.add(path);
            expandedPaths = new Set(expandedPaths);
            if (!treeData[path]) {
                try {
                    treeData[path] = await invoke<string[]>(
                        "get_subdirectories",
                        { path },
                    );
                } catch (e) {
                    console.error("Failed to load subfolders", e);
                }
            }
        }
    }

    $effect(() => {
        if (settingsState.musicPaths.length > 0 && !rootPath) {
            const lastFolder = settingsState.lastImportFolder;
            const rootMatch = settingsState.musicPaths.find((p) =>
                lastFolder.startsWith(p),
            );
            if (rootMatch && lastFolder) {
                rootPath = rootMatch;
                targetFolder = lastFolder;
                const parts = lastFolder
                    .replace(rootMatch, "")
                    .split("/")
                    .filter(Boolean);
                let current = rootMatch;
                void toggleFolder(current);
                for (const part of parts) {
                    current = `${current}/${part}`;
                    void toggleFolder(current);
                }
            } else {
                rootPath = settingsState.musicPaths[0];
                targetFolder = rootPath;
                void toggleFolder(rootPath);
            }
        }
    });

    $effect(() => {
        if (dl) {
            title = dl.track.title || "";
            artists = dl.track.artists || "";
            album = dl.track.album || "";
            albumArtist = dl.track.album_artist || "";
            genre = settingsState.lastImportGenre || dl.track.genre || "";
            trackNumber = dl.track.track_number || null;
            discNumber = dl.track.disc_number || null;
            releaseDate = dl.track.release_date || "";
        }
    });

    async function handleImport() {
        if (!dl || !targetFolder) return;
        isImporting = true;
        error = "";
        try {
            await invoke("import_downloaded_track", {
                tempPath: dl.tempPath,
                targetFolder,
                metadata: {
                    title,
                    artists,
                    album,
                    album_artist: albumArtist,
                    genre,
                    track_number: trackNumber,
                    disc_number: discNumber,
                    release_date: releaseDate,
                    cover_url: dl.coverUrl,
                },
            });
            settingsState.lastImportGenre = genre;
            settingsState.lastImportFolder = targetFolder;
            downloadState.clearFinished(dl.id);
            downloadState.trackToImport = null;
            libraryState.refresh();
        } catch (e) {
            error = String(e);
        } finally {
            isImporting = false;
        }
    }

    function cancel() {
        downloadState.trackToImport = null;
    }
</script>

{#snippet field(
    Icon: any,
    label: string,
    value: string,
    onInput: (v: string) => void,
    type?: string,
)}
    <div class="space-y-2">
        <label class={labelClass}>
            <Icon size={12} />
            {label}
        </label>
        <input
            {type}
            {value}
            oninput={(e) => onInput((e.target as HTMLInputElement).value)}
            class={inputClass}
        />
    </div>
{/snippet}

{#snippet numberField(
    Icon: any,
    label: string,
    value: number | null,
    onInput: (v: number | null) => void,
)}
    <div class="space-y-2">
        <label class={labelClass}>
            <Icon size={12} />
            {label}
        </label>
        <input
            type="number"
            value={value ?? ""}
            oninput={(e) => {
                const v = (e.target as HTMLInputElement).value;
                onInput(v ? Number(v) : null);
            }}
            class={inputClass}
        />
    </div>
{/snippet}

{#if dl}
    <div
        class="fixed inset-0 z-[20000] flex items-center justify-center p-4"
        transition:fade={{ duration: 200 }}
        role="dialog"
    >
        <button
            class="absolute inset-0 bg-black/80 backdrop-blur-md w-full h-full border-none cursor-default"
            onclick={cancel}
            aria-label="Close modal"
        ></button>

        <div
            class="relative w-full max-w-2xl bg-[#0c0c0e] border border-white/10 rounded-3xl shadow-2xl overflow-hidden flex flex-col max-h-[90vh]"
            transition:scale={{ duration: 300, start: 0.95 }}
        >
            <div
                class="px-8 py-6 border-b border-white/5 flex items-center justify-between bg-white/[0.02]"
            >
                <div class="flex items-center gap-4">
                    <div
                        class="w-12 h-12 bg-green-500/10 rounded-2xl flex items-center justify-center border border-green-500/20 text-green-400"
                    >
                        <Save size={24} />
                    </div>
                    <div>
                        <h2 class="text-xl font-bold text-white leading-none">
                            Import Track
                        </h2>
                        <p class="text-sm text-gray-500 mt-1.5">
                            Review metadata before adding to library
                        </p>
                    </div>
                </div>
                <button
                    onclick={cancel}
                    class="p-2 text-gray-400 hover:text-white hover:bg-white/5 rounded-xl transition-all"
                >
                    <X size={24} />
                </button>
            </div>

            <div class="p-8 overflow-y-auto custom-scrollbar space-y-6">
                {#if error}
                    <div
                        class="p-4 bg-red-500/10 border border-red-500/20 rounded-2xl text-red-400 text-sm"
                    >
                        {error}
                    </div>
                {/if}

                <div class="space-y-3">
                    <label class={labelClass}
                        ><Folder size={12} /> Save to Folder</label
                    >
                    <div class="space-y-2">
                        <select
                            value={rootPath}
                            onchange={(e) => {
                                rootPath = (e.target as HTMLSelectElement)
                                    .value;
                                targetFolder = rootPath;
                                expandedPaths.clear();
                                expandedPaths = new Set(expandedPaths);
                                void toggleFolder(rootPath);
                            }}
                            class="w-full px-4 py-3 bg-white/[0.03] border border-white/10 rounded-2xl text-sm text-white focus:outline-none focus:border-indigo-500/50 transition-all appearance-none cursor-pointer"
                        >
                            {#each settingsState.musicPaths as path}
                                <option value={path}>{path}</option>
                            {/each}
                        </select>

                        <div
                            class="bg-white/[0.02] border border-white/5 rounded-2xl overflow-hidden"
                        >
                            <div
                                class="px-4 py-2 bg-white/[0.03] border-b border-white/5 items-center justify-between hidden md:flex"
                            >
                                <span
                                    class="text-[10px] font-mono text-gray-500 truncate mr-2"
                                >
                                    Target: {targetFolder.replace(
                                        rootPath,
                                        "~",
                                    )}
                                </span>
                            </div>
                            <div
                                class="p-2 max-h-60 overflow-y-auto custom-scrollbar space-y-1"
                            >
                                {@render folderNode(rootPath, rootPath, 0)}
                            </div>
                        </div>
                    </div>
                </div>

                {#snippet folderNode(path: string, name: string, depth: number)}
                    {@const isExpanded = expandedPaths.has(path)}
                    {@const isSelected = targetFolder === path}
                    <div class="space-y-1">
                        <div
                            class="flex items-center gap-1 group"
                            style="padding-left: {depth * 12}px"
                        >
                            <button
                                onclick={() => toggleFolder(path)}
                                class="p-1 hover:bg-white/5 rounded-md transition-colors text-gray-500 hover:text-gray-300"
                                aria-label={isExpanded ? "Collapse" : "Expand"}
                            >
                                {#if isExpanded}<ChevronDown
                                        size={14}
                                    />{:else}<ChevronRight size={14} />{/if}
                            </button>
                            <button
                                onclick={() => (targetFolder = path)}
                                class="flex-1 flex items-center gap-2 px-3 py-1.5 rounded-xl text-left text-[13px] transition-all {isSelected
                                    ? 'bg-indigo-500/15 text-indigo-400 border border-indigo-500/20'
                                    : 'text-gray-400 hover:bg-white/[0.04] hover:text-white border border-transparent'}"
                            >
                                <Folder
                                    size={15}
                                    class={isSelected
                                        ? "text-indigo-400"
                                        : "text-gray-600"}
                                />
                                <span class="truncate font-medium"
                                    >{depth === 0 ? "Root Library" : name}</span
                                >
                                {#if isSelected}
                                    <div
                                        class="ml-auto w-1.5 h-1.5 bg-indigo-500 rounded-full shadow-[0_0_8px_rgba(99,102,241,0.5)]"
                                    ></div>
                                {/if}
                            </button>
                        </div>
                        {#if isExpanded && treeData[path]}
                            <div class="space-y-0.5">
                                {#each treeData[path] as sub}
                                    {@render folderNode(
                                        `${path}/${sub}`,
                                        sub,
                                        depth + 1,
                                    )}
                                {:else}
                                    <div
                                        class="text-[10px] text-gray-600/60 font-medium py-1"
                                        style="padding-left: {(depth + 1) * 12 +
                                            28}px"
                                    >
                                        (empty folder)
                                    </div>
                                {/each}
                            </div>
                        {/if}
                    </div>
                {/snippet}

                <div class="h-px bg-white/5"></div>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    {@render field(Music, "Title", title, (v) => (title = v))}
                    {@render field(
                        User,
                        "Artists (semicolon separated)",
                        artists,
                        (v) => (artists = v),
                    )}
                    {@render field(Disc, "Album", album, (v) => (album = v))}
                    {@render field(
                        User,
                        "Album Artist",
                        albumArtist,
                        (v) => (albumArtist = v),
                    )}
                    {@render field(
                        Bookmark,
                        "Genre",
                        genre,
                        (v) => (genre = v),
                    )}
                    {@render field(
                        Calendar,
                        "Release Date",
                        releaseDate,
                        (v) => (releaseDate = v),
                    )}
                    <div class="grid grid-cols-2 gap-4">
                        {@render numberField(
                            Hash,
                            "Track #",
                            trackNumber,
                            (v) => (trackNumber = v),
                        )}
                        {@render numberField(
                            Hash,
                            "Disc #",
                            discNumber,
                            (v) => (discNumber = v),
                        )}
                    </div>
                </div>
            </div>

            <div
                class="px-8 py-5 border-t border-white/5 bg-white/[0.01] flex items-center justify-end gap-4"
            >
                <button
                    onclick={cancel}
                    class="px-6 py-2.5 text-gray-400 hover:text-white text-sm font-bold transition-all"
                    >Cancel</button
                >
                <button
                    onclick={handleImport}
                    disabled={isImporting || !targetFolder}
                    class="flex items-center gap-2 px-8 py-2.5 bg-green-500 hover:bg-green-400 disabled:opacity-50 text-black text-sm font-bold rounded-xl transition-all shadow-lg shadow-green-500/20 active:scale-95"
                >
                    {#if isImporting}
                        <div
                            class="w-4 h-4 border-2 border-black/30 border-t-black rounded-full animate-spin"
                        ></div>
                        Importing...
                    {:else}
                        <Check size={18} /> Add to Library
                    {/if}
                </button>
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
        background: rgba(255, 255, 255, 0.05);
        border-radius: 10px;
    }
    .custom-scrollbar::-webkit-scrollbar-thumb:hover {
        background: rgba(255, 255, 255, 0.1);
    }
</style>
