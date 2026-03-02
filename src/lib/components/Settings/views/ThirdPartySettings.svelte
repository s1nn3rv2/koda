<script lang="ts">
    import { settingsState } from "$lib/state/settings.svelte";
    import { Cloud } from "@lucide/svelte";
</script>

<div class="space-y-6">
    <div class="flex items-center gap-2 mb-2">
        <Cloud size={16} class="text-indigo-400" />
        <h3 class="text-sm font-bold text-white uppercase tracking-widest">
            3rd Party Sources
        </h3>
    </div>

    <div class="grid gap-2">
        <p class="text-xs text-gray-400 mb-2">
            Manage automated fetching of missing metadata like artist profiles
            and album covers.
        </p>

        <div
            class="bg-white/5 border border-white/5 rounded-2xl p-4 flex items-center justify-between"
        >
            <div class="flex-1 pr-6">
                <h4 class="text-sm font-medium text-white mb-1">MusicBrainz</h4>
                <p class="text-xs text-gray-400">
                    Automatically fetch cover art for music and embed it into
                    your file metadata.
                </p>
            </div>

            <label
                class="relative inline-flex items-center cursor-pointer shrink-0"
            >
                <input
                    type="checkbox"
                    bind:checked={settingsState.enableMusicBrainz}
                    class="sr-only peer"
                />
                <div
                    class="w-11 h-6 bg-white/10 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:left-0.5 after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-indigo-500"
                ></div>
            </label>
        </div>

        <div
            class="bg-white/5 border border-white/5 rounded-2xl p-4 flex items-center justify-between"
        >
            <div class="flex-1 pr-6">
                <h4 class="text-sm font-medium text-white mb-1">iTunes</h4>
                <p class="text-xs text-gray-400">
                    Fetch artist and music cover art from the iTunes library.
                </p>
            </div>

            <label
                class="relative inline-flex items-center cursor-pointer shrink-0"
            >
                <input
                    type="checkbox"
                    bind:checked={settingsState.enableITunes}
                    class="sr-only peer"
                />
                <div
                    class="w-11 h-6 bg-white/10 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:left-0.5 after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-indigo-500"
                ></div>
            </label>
        </div>

        <div
            class="bg-white/5 border border-white/5 rounded-2xl p-4 flex items-center justify-between"
        >
            <div class="flex-1 pr-6">
                <h4 class="text-sm font-medium text-white mb-1">LRCLIB</h4>
                <p class="text-xs text-gray-400">
                    Automatically look for time-synced lyrics and embed them
                    into your file metadata.
                </p>
            </div>

            <label
                class="relative inline-flex items-center cursor-pointer shrink-0"
            >
                <input
                    type="checkbox"
                    bind:checked={settingsState.enableLrclib}
                    class="sr-only peer"
                />
                <div
                    class="w-11 h-6 bg-white/10 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:left-0.5 after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-indigo-500"
                ></div>
            </label>
        </div>
    </div>
</div>
