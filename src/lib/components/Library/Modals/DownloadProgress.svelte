<script lang="ts">
    import { X, RotateCw } from "@lucide/svelte";
    import { downloadState } from "$lib/state/download.svelte";

    const formatMB = (bytes: number) => (bytes / (1024 * 1024)).toFixed(1);
</script>

{#if downloadState.downloads.length > 0}
    <div class="space-y-2 mb-4">
        {#each downloadState.downloads as dl}
            {@const progress =
                dl.total > 0
                    ? Math.min(100, Math.round((dl.current / dl.total) * 100))
                    : 0}
            {@const isFinished =
                dl.status === "finished" ||
                (dl.status !== "error" &&
                    dl.total > 0 &&
                    dl.current >= dl.total)}
            <div
                class="p-3 bg-white/3 border border-white/10 rounded-xl text-gray-300 text-sm"
            >
                <div class="flex justify-between items-center mb-1.5">
                    <div class="flex flex-col gap-0.5 max-w-[70%]">
                        <span class="font-bold truncate" title={dl.error}>
                            {#if dl.status === "finished"}Ready to Import:
                            {:else if dl.status === "error"}Error:
                            {:else if dl.status === "resolving"}Resolving:
                            {:else}Downloading:
                            {/if}
                            {dl.track.title}
                        </span>
                        <div
                            class="flex items-center gap-2 text-[10px] text-gray-400 font-medium"
                        >
                            <span
                                class="bg-indigo-500/10 px-1.5 py-0.5 rounded border border-indigo-500/20 text-indigo-400"
                                >{dl.quality}</span
                            >
                            <span
                                class="bg-white/5 px-1.5 py-0.5 rounded border border-white/5"
                                >{dl.fileType}</span
                            >
                            {#if dl.total > 0}
                                <span
                                    >{formatMB(dl.current)} / {formatMB(
                                        dl.total,
                                    )} MB</span
                                >
                            {:else}
                                <span>{formatMB(dl.current)} MB</span>
                            {/if}
                        </div>
                    </div>
                    <div class="flex items-center gap-3 shrink-0">
                        <span class="text-xs font-mono text-gray-400">
                            {dl.status === "finished"
                                ? "100%"
                                : dl.status === "error"
                                  ? "Failed"
                                  : dl.total > 0
                                    ? progress + "%"
                                    : "..."}
                        </span>
                        {#if dl.status === "error"}
                            <button
                                onclick={() =>
                                    downloadState.retryDownload(dl.id)}
                                class="p-2 hover:bg-white/10 rounded-xl text-indigo-500 hover:text-indigo-400 transition-colors"
                                title="Retry"><RotateCw size={18} /></button
                            >
                            <button
                                onclick={() =>
                                    downloadState.clearFinished(dl.id)}
                                class="p-2 hover:bg-white/10 rounded-xl text-gray-500 hover:text-gray-300 transition-colors"
                                title="Dismiss"><X size={18} /></button
                            >
                        {:else if isFinished}
                            <button
                                onclick={() =>
                                    (downloadState.trackToImport = dl)}
                                class="px-5 py-2 bg-indigo-500/20 hover:bg-indigo-500/30 text-indigo-400 border border-indigo-500/30 text-[11px] font-black rounded-xl transition-all active:scale-95 whitespace-nowrap"
                                >IMPORT</button
                            >
                            <button
                                onclick={() =>
                                    downloadState.clearFinished(dl.id)}
                                class="p-2 hover:bg-white/10 rounded-xl text-gray-500 hover:text-gray-300 transition-colors"
                                title="Dismiss"><X size={18} /></button
                            >
                        {:else}
                            <button
                                onclick={() =>
                                    downloadState.retryDownload(dl.id)}
                                class="p-2 hover:bg-white/10 rounded-xl text-gray-200/70 hover:text-gray-100 transition-colors"
                                title="Retry"><RotateCw size={18} /></button
                            >
                            <button
                                onclick={() =>
                                    downloadState.cancelDownload(dl.id)}
                                class="p-2 hover:bg-white/10 rounded-xl text-red-500/70 hover:text-red-400 transition-colors"
                                title="Cancel"><X size={18} /></button
                            >
                        {/if}
                    </div>
                </div>
                {#if dl.total > 0}
                    <div
                        class="mt-2.5 w-full bg-white/5 rounded-full h-1.5 overflow-hidden"
                    >
                        <div
                            class="bg-indigo-500 h-full rounded-full transition-all duration-25"
                            style="width: {progress}%"
                        ></div>
                    </div>
                {/if}
            </div>
        {/each}
    </div>
{/if}
