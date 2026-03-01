<p align="center">
  <img src="https://github.com/user-attachments/assets/d595e91b-0aca-49fc-b3e9-ce3a7b975508" width="200" alt="Koda Logo">
  <h1 align="center">KODA</h1>
  <p align="center">A modern desktop music player with both local and online playback.</p>
</p>
Koda is a modern desktop music player with a twist. While it focuses on local playback, it supports online music playback and download to your local library via Monochrome API. 
It also has metadata tagging features, like foobar2000. It's built on Tauri using Svelte and Rust.

## Screenshots

<img width="1749" height="1043" alt="image" src="https://github.com/user-attachments/assets/2a1e4281-ee18-4a85-af6d-f5030c18892b" />

<img width="1749" height="1043" alt="image" src="https://github.com/user-attachments/assets/f73b15da-7112-4a22-a074-8282f188d04e" />

<img width="1749" height="1043" alt="image" src="https://github.com/user-attachments/assets/09fabe41-e234-438b-b79f-7232eae945b4" />

## Features
- Koda allows you to stream tracks from the online library (powered via [**Monochrome**](https://github.com/monochrome-music/monochrome) while also allowing you to download the tracks into your local library.
- Koda has a metadata tagging similar to foobar2000.
- Koda also has lyric support using LRCLIB.

## Quick Start

### Download binary
To get the latest stable version of Koda:
1. Go to the [**Releases**](https://github.com/s1nn3rv2/koda/releases/latest) page.
2. Download the installer for your OS (`.exe` for Windows, `.dmg` for macOS, or `.AppImage/.deb/.rpm` for Linux).
3. You can now install and enjoy!

### Build from source (Linux)
You'll need to install [**tauri dependencies**](https://v2.tauri.app/start/prerequisites/) before building.
I recommend using pnpm, but you can use npm as well.
```bash
git clone https://github.com/s1nn3rv2/koda.git
cd koda
pnpm tauri:build
```
You can run the dev version using ```pnpm tauri:dev```.
