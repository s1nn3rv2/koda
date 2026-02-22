<script lang="ts">
    import type { Track, SortColumn, SortDirection } from "$lib/types";
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

    const sortOptions = [
        {
            label: "Default",
            column: "default" as SortColumn,
            defaultDir: "desc" as SortDirection,
        },
        {
            label: "Title",
            column: "title" as SortColumn,
            defaultDir: "asc" as SortDirection,
        },
        {
            label: "Date Added",
            column: "added_at" as SortColumn,
            defaultDir: "desc" as SortDirection,
        },
        {
            label: "Release Date",
            column: "release_date" as SortColumn,
            defaultDir: "desc" as SortDirection,
        },
        {
            label: "Duration",
            column: "duration" as SortColumn,
            defaultDir: "desc" as SortDirection,
        },
    ];
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
            options={sortOptions}
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
