import { invoke } from "@tauri-apps/api/core";
import type { WaveformData } from "$lib/types";

interface WaveformLoaderState {
  waveformData: WaveformData | null;
  isLoading: boolean;
  hasError: boolean;
}

export function useWaveformLoader(getTrackId: () => string | null | undefined) {
  let state = $state<WaveformLoaderState>({
    waveformData: null,
    isLoading: false,
    hasError: false,
  });

  let lastLoadedTrackId = $state<string | null>(null);
  let waveformCache = $state<Map<string, WaveformData>>(new Map());

  async function loadWaveform(id: string) {
    state.isLoading = true;
    state.hasError = false;

    if (id.startsWith("online:")) {
      state.waveformData = null;
      state.isLoading = false;
      state.hasError = true;
      return;
    }

    try {
      const data: WaveformData = await invoke("get_waveform", { id });

      const currentTrackId = getTrackId();
      if (id === currentTrackId) {
        state.waveformData = data;
        waveformCache.set(id, data);
        state.isLoading = false;
      }
    } catch (e) {
      console.error("Failed to load waveform:", e);
      const currentTrackId = getTrackId();
      if (id === currentTrackId) {
        state.hasError = true;
        state.isLoading = false;
      }
    }
  }

  $effect(() => {
    const trackId = getTrackId();
    if (!trackId) {
      state.waveformData = null;
      state.isLoading = false;
      state.hasError = false;
      lastLoadedTrackId = null;
    } else if (trackId !== lastLoadedTrackId) {
      lastLoadedTrackId = trackId;

      if (waveformCache.has(trackId)) {
        state.waveformData = waveformCache.get(trackId)!;
        state.isLoading = false;
        state.hasError = false;
      } else {
        loadWaveform(trackId);
      }
    }
  });

  return {
    get waveformData() {
      return state.waveformData;
    },
    get isLoading() {
      return state.isLoading;
    },
    get hasError() {
      return state.hasError;
    },
  };
}
