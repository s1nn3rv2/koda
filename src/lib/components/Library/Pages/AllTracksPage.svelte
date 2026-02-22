<script lang="ts">
    import type { Track, SortColumn, SortDirection } from "$lib/types";
    import { TRACK_SORT_OPTIONS } from "$lib/constants/sortOptions";
    import TrackList from "../TrackList.svelte";
    import ContentHeader from "../ContentHeader.svelte";
    import SortDropdown from "../SortDropdown.svelte";

    interface Props {
        tracks: Track[];
        totalTracks: number;
        ondelete: (id: string) => void;
        ondeletefile: (id: string) => void;
        onloadmore: () => void;
        hasMore: boolean;
        isLoadingMore: boolean;
        onartistclick: (artistName: string) => void;
        onalbumclick: (albumId: string) => void;
        sortColumn: SortColumn;
        sortDirection: SortDirection;
    }

    let {
        tracks,
        totalTracks,
        ondelete,
        ondeletefile,
        onloadmore,
        hasMore,
        isLoadingMore,
        onartistclick,
        onalbumclick,
        sortColumn = $bindable(),
        sortDirection = $bindable(),
    }: Props = $props();
</script>

<ContentHeader
    title="All Tracks"
    subtitle={null}
    {totalTracks}
    contentAlbum={null}
    showCoverArt={false}
>
    {#snippet actions()}
        <SortDropdown
            options={TRACK_SORT_OPTIONS}
            bind:column={sortColumn}
            bind:direction={sortDirection}
        />
    {/snippet}
</ContentHeader>

<TrackList
    {tracks}
    {ondelete}
    {ondeletefile}
    {onloadmore}
    {hasMore}
    {isLoadingMore}
    {onartistclick}
    {onalbumclick}
/>
