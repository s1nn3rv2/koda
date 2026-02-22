<script lang="ts">
    let { albumId, albumName, handleAlbumClick } = $props();
</script>

<span
    class="text-xs text-gray-500 truncate hover:text-white hover:underline cursor-pointer transition-colors"
    onclick={(e) => handleAlbumClick(albumId, e)}
    onkeydown={(e) => {
        if (e.key === "Enter") handleAlbumClick(albumId, e);
    }}
    role="link"
    tabindex="0"
>
    {albumName}
</span>
