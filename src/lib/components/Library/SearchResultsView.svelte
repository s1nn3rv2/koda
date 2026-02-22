<script lang="ts">
    import { uiState } from "$lib/state/player.svelte";
    import { libraryState } from "$lib/state/library.svelte";
    import { monochromeService } from "$lib/services/monochrome";
    import { pluralize } from "$lib/utils/format";
    import type { Track, SortColumn, SortDirection } from "$lib/types";
    import type { SortOption } from "$lib/constants/sortOptions";
    import TrackList from "./TrackList.svelte";
    import ContentHeader from "./ContentHeader.svelte";
    import SortDropdown from "./SortDropdown.svelte";

    interface Props {
        searchResults: Track[];
        searchTotal: number;
        isSearching: boolean;
        sortOptions: SortOption[];
        sortColumn: SortColumn;
        sortDirection: SortDirection;
        hasMoreOnlineSearch: boolean;
        hasMoreSearch: boolean;
        isOnlineSearchingMore: boolean;
        isSearchingMore: boolean;
        ondelete: (id: string) => void;
        ondeletefile: (id: string) => void;
        onloadmoreonline: () => void;
        onloadmoresearch: () => void;
        onartistclick: (name: string) => void;
        onalbumclick: (id: string) => void;
    }

    let {
        searchResults,
        searchTotal,
        isSearching,
        sortOptions,
        sortColumn = $bindable(),
        sortDirection = $bindable(),
        hasMoreOnlineSearch,
        hasMoreSearch,
        isOnlineSearchingMore,
        isSearchingMore,
        ondelete,
        ondeletefile,
        onloadmoreonline,
        onloadmoresearch,
        onartistclick,
        onalbumclick,
    }: Props = $props();

    const isOnline = $derived(uiState.libraryMode === "online");
    const loading = $derived(isSearching || libraryState.isOnlineSearching);

    const subtitle = $derived.by(() => {
        if (loading) return "Searching...";
        const query = libraryState.searchQuery.trim();
        const count = isOnline ? libraryState.onlineSearchTotal : searchTotal;
        return `${count} ${pluralize(count, "result")} for "${query}"`;
    });

    const tracks = $derived(
        isOnline
            ? libraryState.onlineSearchResults.map((t) =>
                  monochromeService.mapToTrack(t),
              )
            : searchResults,
    );
</script>

<div class="mb-4">
    <ContentHeader
        title="Search Results"
        {subtitle}
        totalTracks={isOnline ? libraryState.onlineSearchTotal : searchTotal}
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
    <div
        class={loading
            ? "opacity-50 pointer-events-none transition-opacity"
            : "transition-opacity"}
    >
        <TrackList
            {tracks}
            {ondelete}
            {ondeletefile}
            onloadmore={isOnline ? onloadmoreonline : onloadmoresearch}
            hasMore={isOnline ? hasMoreOnlineSearch : hasMoreSearch}
            isLoadingMore={isOnline ? isOnlineSearchingMore : isSearchingMore}
            {onartistclick}
            {onalbumclick}
        />
    </div>
</div>
