import { playbackState } from "./playback.svelte";
import { queueState } from "./queue.svelte";

export { playbackState } from "./playback.svelte";
export { queueState } from "./queue.svelte";

class UIState {
  isExpanded = $state(false);
  sidebarHidden = $state(false);
  libraryMode = $state<'local' | 'online'>(
    (typeof localStorage !== 'undefined' && localStorage.getItem('libraryMode') as 'local' | 'online') || 'local'
  );
  showLyrics = $state(false);

  toggleExpanded() {
    this.isExpanded = !this.isExpanded;
  }

  toggleSidebar() {
    this.sidebarHidden = !this.sidebarHidden;
  }

  setLibraryMode(mode: 'local' | 'online') {
    this.libraryMode = mode;
    localStorage.setItem('libraryMode', mode);
  }

  toggleLyrics() {
    this.showLyrics = !this.showLyrics;
  }
}

export const uiState = new UIState();
