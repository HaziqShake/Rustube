# Rustube

A fast, lightweight, native YouTube desktop client written 100% in Rust for Linux and Windows.

Built with **Iced** GUI and hardware-accelerated playback via **MPV**, designed to run smoothly even on resource-constrained systems (4 GB RAM).

![Rustube Screenshot](https://raw.githubusercontent.com/HaziqShake/Rustube/main/assets/icons/logo.svg)

## Features

- **Pure Native Rust UI**: Built with [Iced](https://github.com/iced-rs/iced) (`wgpu`), with zero Electron bloat.
- **YouTube Dark Theme**: Pixel-perfect replication of YouTube's desktop web UI:
  - Official vector SVG icons.
  - Inter sans-serif typography.
  - YouTube search pill with dedicated search and voice input buttons.
  - Multi-section sidebar navigation (Home, Shorts, Subscriptions, Library, History, Downloads, Explore, Settings).
  - Category filter pills (*All*, *Gaming*, *Music*, *News*, *Podcasts*, *Live*, *Coding*, *Recently uploaded*, *Watched*).
  - 16:9 responsive video card grid with rounded corners and duration timestamp badges.
- **Embedded In-App Video Playback**:
  - Direct X11 child subwindow embedding right into the watch page stage.
  - YouTube Red (`#ff0000`) seek scrubber progress line.
  - Full player controls (play/pause, seek relative $\pm 10$s, volume control, time display, CC, fullscreen).
  - Hardware-accelerated decoding (VA-API / VP9 / H.264) for smooth 60 FPS playback without AV1 CPU throttling.
  - Inter-Process Communication (IPC) over Unix domain sockets for controls.
- **Watch Page Layout**:
  - Left column: Embedded player stage, title, channel avatar with subscriber count, Subscribe button, segmented Like/Dislike pills, Share, and Download buttons, and expandable description box.
  - Right column: "Up next" recommendation sidebar with compact horizontal video cards.

## Architecture

```
Rustube/
├── crates/
│   ├── rustube-core/    # YouTube API client, SQLite database, MPV player controller & X11 embedding
│   └── rustube-ui/      # Iced GUI application, router, widgets (top bar, sidebar, cards), watch page
├── assets/
│   ├── icons/           # YouTube official SVG icons
│   └── fonts/           # Inter sans-serif font family
└── Cargo.toml           # Cargo workspace configuration
```

## Prerequisites

- **Rust toolchain** (1.80+ or latest stable)
- **MPV** (`sudo dnf install mpv` / `sudo apt install mpv`)
- **yt-dlp** (bundled or in PATH for stream extraction)

## Getting Started

### Clone and Run

```bash
git clone https://github.com/HaziqShake/Rustube.git
cd Rustube
cargo run --bin rustube
```

### Direct Video Playback

Launch directly into any video by passing a YouTube video ID or URL:

```bash
cargo run --bin rustube -- dQw4w9WgXcQ
```

## Implementation Plan & Roadmap

For full architectural blueprints, data schemas, design tokens, and phased feature roadmaps, see [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md).

## License

MIT OR Apache-2.0
