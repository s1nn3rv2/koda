<script lang="ts">
    import { X } from "@lucide/svelte";
    import { downloadState } from "$lib/state/download.svelte";

    const formatMB = (bytes: number) => (bytes / (1024 * 1024)).toFixed(1);
</script>

{#if downloadState.downloads.length > 0}
    <div class="space-y-2 mb-4">
        {#each downloadState.downloads as dl}
            {@const progress =
                dl.total > 0 ? Math.round((dl.current / dl.total) * 100) : 0}
            {@const isFinished =
                dl.status === "finished" ||
                (dl.total > 0 && dl.current >= dl.total)}
            <div
                class="p-3 bg-green-500/10 border border-green-500/20 rounded-xl text-green-400 text-sm"
            >
                <div class="flex justify-between items-center mb-1.5">
                    <div class="flex flex-col gap-0.5 max-w-[70%]">
                        <span class="font-bold truncate">
                            {#if dl.status === "finished"}Ready to Import:
                            {:else if dl.status === "resolving"}Resolving:
                            {:else}Downloading:
                            {/if}
                            {dl.track.title}
                        </span>
                        <div
                            class="flex items-center gap-2 text-[10px] text-green-400/60 font-medium"
                        >
                            <span
                                class="bg-green-500/10 px-1.5 py-0.5 rounded border border-green-500/20"
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
                        <span class="text-xs font-mono text-green-400/70">
                            {dl.status === "finished"
                                ? "100%"
                                : dl.total > 0
                                  ? progress + "%"
                                  : "..."}
                        </span>
                        {#if isFinished}
                            <button
                                onclick={() =>
                                    (downloadState.trackToImport = dl)}
                                class="px-5 py-2 bg-green-500 hover:bg-green-400 text-black text-[11px] font-black rounded-xl transition-all active:scale-95 shadow-lg shadow-green-500/20 whitespace-nowrap"
                                >IMPORT</button
                            >
                            <button
                                onclick={() =>
                                    downloadState.clearFinished(dl.id)}
                                class="p-2 hover:bg-white/10 rounded-xl text-green-400/50 hover:text-green-400 transition-colors"
                                title="Dismiss"><X size={18} /></button
                            >
                        {/if}
                    </div>
                </div>
                {#if dl.status !== "finished" && dl.total > 0}
                    <div
                        class="mt-2.5 w-full bg-white/5 rounded-full h-1.5 overflow-hidden"
                    >
                        <div
                            class="bg-green-500 h-full rounded-full transition-all duration-300 shadow-[0_0_10px_rgba(34,197,94,0.3)]"
                            style="width: {progress}%"
                        ></div>
                    </div>
                {/if}
            </div>
        {/each}
    </div>
{/if}
