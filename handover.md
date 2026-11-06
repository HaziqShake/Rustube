# Rustube — Project Handover Document

**Version:** 0.1.0 (Phase 1 Complete)  
**Date:** November 2026  
**Repository:** [HaziqShake/Rustube](https://github.com/HaziqShake/Rustube)  
**Branch:** `main`

---

## 1. Executive Summary

**Rustube** is a native Linux desktop YouTube client built from scratch in **Rust**, utilizing **Iced 0.13** for the declarative graphical user interface and an embedded **MPV** instance for hardware-accelerated video decoding. 

The primary goals of this project are:
- **Zero Web-Bloat**: Eliminate the memory, CPU, and latency overhead of Electron and Chromium-based wrappers.
- **Hardware Acceleration**: Out-of-the-box VA-API GPU decoding targeting 1080p60 on lightweight hardware (tested on Intel UHD Graphics 620).
- **1:1 YouTube Desktop Fidelity**: Pixel-accurate recreation of `youtube.com` dark theme, including masthead search pills, category chips, responsive 16:9 card reflow, skeleton loaders, and watch page layout.

---

## 2. Architecture & Crate Breakdown

The workspace is organized into a clean multi-crate architecture:

```
Rustube/
├── Cargo.toml                      # Workspace root configuration
├── handover.md                     # This project handover document
├── assets/
│   ├── fonts/                      # Bundled Roboto TTF weights (Regular, Medium, Bold)
│   └── icons/                      # Standardized SVG icons matching YouTube desktop
├── crates/
│   ├── rustube-core/               # Business logic, networking, player orchestration
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── player/
│   │       │   ├── controller.rs   # MPV process lifecycle, IPC socket, X11 subwindow
│   │       │   ├── mod.rs
│   │       │   └── sponsorblock.rs # SponsorBlock API client integration
│   │       └── youtube/
│   │           ├── client.rs       # YouTube data extraction & search
│   │           ├── mod.rs
│   │           └── models.rs       # VideoItem, VideoDetails, Channel, Stream structs
│   └── rustube-ui/                 # Native Iced 0.13 GUI application
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs             # Application entrypoint & font registry
│           ├── app.rs              # State machine, tasks, subscriptions, layout root
│           ├── message.rs          # Unified UI event/action enum
│           ├── router.rs           # Navigation routing (Home, Watch, Search, etc.)
│           ├── theme.rs            # YouTube dark theme design tokens & widget styles
│           ├── pages/
│           │   ├── home.rs         # Responsive home feed & category filter bar
│           │   ├── search.rs       # Search result rows with channel metadata
│           │   ├── watch.rs        # Player stage, action pills, description, up next
│           │   └── mod.rs
│           ├── widgets/
│           │   ├── top_bar.rs      # Masthead, search pill, voice mic, user avatar
│           │   ├── sidebar.rs      # Collapsible navigation drawer (expanded & mini)
│           │   ├── video_card.rs   # Grid cards, compact cards, skeleton loaders
│           │   └── mod.rs
│           └── utils/
│               ├── format.rs       # View counts (1 decimal place), relative dates, title sanitizing
│               ├── icons.rs        # SVG icon loader
│               └── images.rs       # 16:9 auto-cropping & anti-aliased corner rounding
```

---

## 3. Phase One Deliverables & Visual Fidelity

### A. Masthead (`widgets/top_bar.rs`)
- **Centered Search Pill**: 40px height, centered using flex portions.
  - Left input field: `#121212` background, `1px solid #303030` border with 20px rounded left corners. Focused state dynamically highlights `#3ea6ff`.
  - Right search submit button: `#222222` background, `1px solid #303030` border with 20px rounded right corners, 20px magnifying glass icon.
  - Voice mic: 40px circular button (`#181818`), separated with standard 12px margin.
- **Brand Lockup**: Hamburger menu with hover circle, official YouTube badge (`assets/icons/logo.svg`), "Rustube" wordmark, and country badge (`IN`).
- **Profile Actions**: 40px notification bell button and 32px circular user avatar.

### B. Navigation Sidebar (`widgets/sidebar.rs`)
- **Expanded (240px)** and **Mini (72px)** modes toggled via the masthead hamburger icon.
- Items are 40px tall with 22px icons and 14px labels, vertically centered using `.align_y(Alignment::Center)`.
- Grouped into distinct sections separated by `#272727` 1px dividers:
  - Navigation: *Home*, *Shorts*, *Subscriptions*.
  - User: *History*, *Playlists*, *Downloads*.
  - Explore: *Trending*, *Music*, *Gaming*, *News*.
  - Utility: *Settings*.
- Entire column is wrapped in a vertical `scrollable` to prevent clipping on low-resolution displays.

### C. Category Chips Bar (`pages/home.rs`)
- Horizontally scrollable filter bar with hidden scrollbar.
- 32px tall pills with 8px corner radius.
- Padding balanced (`[0, 12]`) for centered glyph rendering.
- Inactive pills use `#272727` (hover `#3d3d3d`), active pill uses `#f1f1f1` background with black medium text.

### D. Video Grid & Card System (`widgets/video_card.rs`)
- **Responsive Reflow**: Column count dynamically calculates from window width (clamped 1 to 4 columns, ~330px minimum card width).
- **Uniform Row Alignment**: Title container is clamped to a fixed 42px height (2 lines × 21px line-height). Avatars, channel names, and views remain aligned across all cards in a row regardless of title length.
- **Last-Row Balance**: Remainder slots are padded with `Length::FillPortion(1)` spacers so incomplete rows do not stretch cards across the screen.
- **Duration Badges**: Stacked bottom-right over the thumbnail (4px margin, 4px radius, 80% black background, white medium 12px font).

### E. YouTube Skeleton Loaders (`widgets/video_card.rs`, `pages/home.rs`)
- High-fidelity skeleton placeholder cards rendered during network loading:
  - 16:9 placeholder box (`#212121`, 12px radius).
  - 36px circular avatar placeholder (`#272727`).
  - Two-line title placeholder bars (Line 1: 90% width, Line 2: 60% width, 4px radius).
  - Channel name and metadata placeholder bars.
- Compact skeleton variant for the watch page "Up next" recommendation column.

### F. Watch Page & Embedded Player (`pages/watch.rs`, `player/controller.rs`)
- **Embedded X11 Player Subwindow**: Spawns MPV into an X11 child subwindow mapped to exact 16:9 stage bounds (`(24, 80)`).
- **Non-Blocking Startup**: Dispatched asynchronously via `tokio::task::spawn_blocking` so clicking video cards navigates instantly with 0ms UI delay.
- **Clean Stage**: Removed distracting overlay placeholder text; uses a pure black canvas while MPV renders.
- **Interactive Controls Bar**:
  - Custom red rail scrubber slider synced to MPV playback position via IPC.
  - Play/Pause, -10s seek, +10s seek, volume slider, time display (`0:00 / 3:33`).
  - Subtitles (CC), Settings, Miniplayer (PIP), and Fullscreen buttons.
- **Metadata & Action Bar**:
  - 40px circular channel avatar, channel name, subscriber count, white "Subscribe" pill button.
  - Segmented like/dislike pill (`#272727`, 36px height, 1px divider).
  - Share and Download pills (`#272727`, 36px height).
  - Description box (`#272727`, 12px radius, views • relative date, clean formatted description).
- **"Up Next" Sidebar**: Populated with compact recommendation cards (168×94px thumbnail with duration badge, title, channel name, views).

---

## 4. Key Bug Fixes & Technical Learnings

| Issue | Root Cause | Solution Applied |
|---|---|---|
| **Feeds not scrollable** | Parent `Row` in `app.rs` lacked `.height(Length::Fill)`, defaulting to `Shrink`. In Iced 0.13, this gave the child `scrollable` infinite available height, causing `viewport == content` and zero overflow. | Added `.height(Length::Fill)` to `row![sidebar, main_view]` and bounded sidebar containers. |
| **Thumbnail corners having "weird squares"** | `apply_rounded_mask` scanned a full `(2r+1) × (2r+1)` box around each corner center. Pixels in the corner quadrant facing the interior of the image had `dist > r`, carving circular bites out of the image. | Rewrote rounding to strictly test outer quadrants (`0..r`, `w-r..w`, `0..r`, `h-r..h`) with subpixel sampling `(x+0.5, y+0.5)`. |
| **Button delay & UI freeze** | `find_rustube_window` slept up to 3,000 ms synchronously on the UI thread, and `query_state` made 3 blocking Unix socket queries with 300 ms timeouts every 500 ms. | Moved player launch to background async tasks, cached window ID via `AtomicU32`, and clamped IPC socket timeouts to 15 ms. |
| **Missing glyph boxes (`[]`)** | Bundled Roboto font lacks emoji glyphs; video titles containing emojis rendered fallback missing-character boxes. | Implemented `format::clean_text` to filter unrenderable emoji code points and normalize whitespace. |
| **View counts precision** | Formatted with 2 decimal places (e.g. `124.49M`). | Updated `format_count` to strictly format with 1 decimal place (e.g. `124.5M`, `320.1K`). |
| **4:3 Black Letterbox Thumbnails** | YouTube API often returns 4:3 `hqdefault` thumbnails (480×360) with baked-in black bars. | Implemented automatic 16:9 aspect ratio center-cropping in `images.rs` before corner rounding. |

---

## 5. Development & Build Guide

### Prerequisites
- **Rust Toolchain**: 1.80+ (`rustup default stable`)
- **System Libraries**:
  - `mpv` (installed and available in `PATH`)
  - `yt-dlp` (required by MPV for YouTube stream resolution)
  - `libx11`, `libxcb`, `fontconfig` development packages

### Build Commands

```bash
# Verify workspace compilation
cargo check --workspace

# Build debug binary
cargo build --bin rustube

# Run application
cargo run --bin rustube

# Run application directly opening a video by ID or URL
cargo run --bin rustube -- "dQw4w9WgXcQ"
cargo run --bin rustube -- "https://www.youtube.com/watch?v=dQw4w9WgXcQ"

# Build optimized release binary
cargo build --release --bin rustube
```

---

## 6. Skills & Tooling Applied

The development, reverse engineering, visual redesign, and debugging of Rustube leveraged the following specialized skills, plugins, and tooling:

1. **`figma-create-new-file`**:
   - Used to programmatically initialize and link the official Figma design file under the authenticated team workspace (`Hi Hello's team`), creating the persistent canvas: [Rustube Figma Design](https://www.figma.com/design/yI5vC6ppPW3LPwCspvV1TG).

2. **`figma-code-workflows`**:
   - Packaged and imported the complete interactive HTML/CSS YouTube desktop clone (tokens, layout, masthead, sidebar, chips, grid, and watch page) into Figma via `send_code_to_figma`.

3. **`figma-use` / Figma MCP Suite**:
   - Used Figma MCP server integration (`whoami`, `create_new_file`, `send_code_to_figma`) to bridge UI design specifications with native code implementations.

4. **Web Reverse-Engineering & Headless Snapshot Analysis (`reverse-engineer-anything`)**:
   - Executed headless browser automation (`firefox --headless --screenshot`) against live `youtube.com` to analyze DOM wireframes, layout reflow ratios (3-to-4 columns), typography scales (14px, 16px, 18px, 20px), design tokens (`#0f0f0f`, `#121212`, `#222222`, `#272727`, `#303030`, `#3ea6ff`), and exact spacing hierarchies.

5. **Multi-Agent Concurrent Auditing (`subagents`)**:
   - Spawned multiple specialized background subagents to perform simultaneous codebase audits:
     - **Runtime & Performance Auditor**: Deeply analyzed asynchronous execution paths, identified UI-thread blocking sleeps in `find_rustube_window`, IPC socket timeouts in `query_state`, and diagnosed the unconstrained flex height bug preventing scrolling in Iced.
     - **Visual Design & UI Polish Auditor**: Diagnosed the mathematical quadrant scanning bug in `apply_rounded_mask` that caused corner square artifacts, standardized view counts to 1 decimal place, centered glyph alignments, and designed the YouTube skeleton card component.

6. **Desktop Display & X11 Automation (`x11rb`, `xprop`, `import`, `spectacle`)**:
   - Utilized X11 protocol introspection (`_NET_CLIENT_LIST`, `_NET_WM_PID`, `WM_NAME`) for embedding child subwindows, capturing runtime visual proofs, and verifying pixel-level layout correctness.

---

## 7. Phase Two Roadmap & Next Steps

1. **Comments Section**:
   - Integrate YouTube comment thread fetching under the watch page description box.
   - Channel avatars, author names, relative timestamps, like counts, and reply threads.
2. **SponsorBlock Integration**:
   - Hook `rustube-core::player::sponsorblock` into MPV IPC timeline markers to automatically skip sponsor segments.
3. **Downloads & Offline Library**:
   - Implement downloading queue using `rusty_pipe` stream endpoints.
   - Dedicated Downloads page (`Page::Downloads`) showing progress bars and saved local files.
4. **Subscriptions & History**:
   - Local JSON/SQLite storage for subscription feeds and watch history.
5. **Wayland Native Subsurfaces**:
   - Add Wayland subsurface integration alongside X11 subwindows for native Wayland environments without XWayland.

---

*Handover document authored and verified for Rustube v0.1.0.*
