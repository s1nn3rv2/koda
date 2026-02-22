<script lang="ts">
    import type {
        Track,
        GenreWithCount,
        SortColumn,
        SortDirection,
    } from "$lib/types";
    import TrackList from "../TrackList.svelte";
    import ContentHeader from "../ContentHeader.svelte";
    import SortDropdown from "../SortDropdown.svelte";

    interface Props {
        genre: GenreWithCount;
        tracks: Track[];
        totalTracks: number;
        ondelete: (id: string) => void;
        ondeletefile: (id: string) => void;
        onartistclick: (artistName: string) => void;
        onalbumclick: (albumId: string) => void;
        sortColumn: SortColumn;
        sortDirection: SortDirection;
    }

    let {
        genre,
        tracks,
        totalTracks,
        ondelete,
        ondeletefile,
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
    title={genre.name}
    subtitle="Genre"
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
    {onartistclick}
    {onalbumclick}
    layout="grid"
/>
