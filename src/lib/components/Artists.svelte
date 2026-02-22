<script lang="ts">
    import { splitArtists } from "$lib/utils/split_artists";

    let {
        artists,
        handleArtistClick,
    }: {
        artists: string;
        handleArtistClick: (
            artist: string,
            event: MouseEvent | KeyboardEvent,
        ) => void;
    } = $props();
</script>

{#each splitArtists(artists) as artist, i}
    {#if i > 0}<span class="text-gray-600">,{" "}</span>{/if}
    <span
        class="hover:text-white hover:underline cursor-pointer transition-colors"
        onclick={(e) => handleArtistClick(artist, e)}
        onkeydown={(e) => {
            if (e.key === "Enter") handleArtistClick(artist, e);
        }}
        role="link"
        tabindex="0">{artist}</span
    >
{/each}
