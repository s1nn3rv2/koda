import { browser } from "$app/environment";

export type AudioQuality = "HI_RES_LOSSLESS" | "LOSSLESS" | "HIGH" | "LOW";

export interface MonochromeInstance {
  url: string;
  version: string;
  isStreaming?: boolean;
}

class SettingsState {
  isSettingsOpen = $state(false);
  monochromeInstance = $state(
    browser
      ? localStorage.getItem("monochrome_instance") ||
          "https://api.monochrome.tf"
      : "https://api.monochrome.tf",
  );
  audioQuality = $state<AudioQuality>(
    browser
      ? (localStorage.getItem("audio_quality") as AudioQuality) || "HIGH"
      : "HIGH",
  );
  downloadQuality = $state<AudioQuality>(
    browser
      ? (localStorage.getItem("download_quality") as AudioQuality) ||
          "HI_RES_LOSSLESS"
      : "HI_RES_LOSSLESS",
  );
  musicPaths = $state<string[]>(
    browser ? JSON.parse(localStorage.getItem("music_paths") || "[]") : [],
  );
  lastImportGenre = $state(
    browser ? localStorage.getItem("last_import_genre") || "" : "",
  );
  lastImportFolder = $state(
    browser ? localStorage.getItem("last_import_folder") || "" : "",
  );

  dynamicInstances = $state<MonochromeInstance[]>([]);
  isLoadingInstances = $state(false);

  constructor() {
    $effect.root(() => {
      $effect(() => {
        if (browser) {
          localStorage.setItem("monochrome_instance", this.monochromeInstance);
          localStorage.setItem("audio_quality", this.audioQuality);
          localStorage.setItem("download_quality", this.downloadQuality);
          localStorage.setItem("music_paths", JSON.stringify(this.musicPaths));
          localStorage.setItem("last_import_genre", this.lastImportGenre);
          localStorage.setItem("last_import_folder", this.lastImportFolder);
        }
      });
    });
  }

  toggleSettings() {
    this.isSettingsOpen = !this.isSettingsOpen;
  }

  setInstance(url: string) {
    this.monochromeInstance = url;
  }

  setAudioQuality(quality: AudioQuality) {
    this.audioQuality = quality;
  }

  setDownloadQuality(quality: AudioQuality) {
    this.downloadQuality = quality;
  }

  addMusicPath(path: string) {
    if (!this.musicPaths.includes(path)) {
      this.musicPaths.push(path);
    }
  }

  removeMusicPath(path: string) {
    this.musicPaths = this.musicPaths.filter((p) => p !== path);
  }

  async fetchDynamicInstances() {
    if (this.dynamicInstances.length > 0 && !this.isLoadingInstances) return;

    this.isLoadingInstances = true;
    try {
      const trackers = [
        "https://tidal-uptime.jiffy-puffs-1j.workers.dev/",
        "https://tidal-uptime.props-76styles.workers.dev/",
      ];

      let data;
      for (const url of trackers) {
        try {
          const res = await fetch(url);
          if (res.ok) {
            data = await res.json();
            break;
          }
        } catch (e) {
          console.warn(`[Settings] Tracker ${url} failed`, e);
        }
      }

      if (data) {
        const api = data.api || [];
        const streaming = new Set(
          (data.streaming || []).map((s: any) => s.url),
        );

        this.dynamicInstances = api.map((item: any) => ({
          url: item.url,
          version: item.version,
          isStreaming: streaming.has(item.url),
        }));
        console.log(
          `[Settings] Fetched ${this.dynamicInstances.length} dynamic instances`,
        );
      }
    } catch (e) {
      console.error("[Settings] Failed to fetch instances", e);
    } finally {
      this.isLoadingInstances = false;
    }
  }
}

export const settingsState = new SettingsState();
