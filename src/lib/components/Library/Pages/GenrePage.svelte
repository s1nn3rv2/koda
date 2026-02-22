<script lang="ts">
    import type {
        Track,
        GenreWithCount,
        SortColumn,
        SortDirection,
    } from "$lib/types";
    import { TRACK_SORT_OPTIONS } from "$lib/constants/sortOptions";
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
    {onartistclick}
    {onalbumclick}
    layout="grid"
/>
