# cosmic-rss

> ⚠️ **DISCLAIMER: This project is a learning exercise and is under development. It is not production-ready and may have bugs, missing features, or break between updates.**

A simple RSS reader built with [libcosmic](https://github.com/pop-os/libcosmic) and Rust, designed for the COSMIC desktop environment on Linux.

![Status](https://img.shields.io/badge/status-under%20development-orange)
![Platform](https://img.shields.io/badge/platform-Linux-blue)

---

## What it does

- Fetches articles from a hardcoded list of RSS feeds (news, tech, science, open source)
- Stores them locally in a SQLite database so they persist between launches
- Displays feeds in a collapsible sidebar — click a feed to filter articles
- Lazy-loads articles as you scroll, 50 at a time
- Auto-syncs every 10 minutes in the background
- "Refresh" button in the header to trigger an immediate sync

## What it doesn't do (yet)

- Feeds are hardcoded — no way to add or remove feeds from the UI (you have to edit `sync.rs` directly)
- No article reading view — titles only, no content or summary pane
- No read/unread tracking
- No search
- No notifications
- RSS has no historical pagination, so older articles only accumulate over time as you leave the app running

---

## Requirements

- Fedora Linux (tested on Fedora 40+)
- Rust stable toolchain via [rustup](https://rustup.rs)

## Building

### 1. Install system dependencies

```bash
sudo dnf install -y \
    cargo \
    cmake \
    just \
    expat-devel \
    fontconfig-devel \
    freetype-devel \
    libxkbcommon-devel \
    mesa-libGL-devel \
    mesa-libEGL-devel \
    wayland-devel \
    pkgconf-pkg-config \
    gcc \
    g++
```

### 2. Install Rust (if not already installed)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### 3. Clone and build

```bash
git clone <your-repo-url>
cd cosmic_rss

# Check for errors without building
cargo check

# Debug build (unoptimised, good for development)
cargo run

# Release build (optimised, recommended for actual use)
cargo build --release
./target/release/cosmic-rss
```

On first run, `rss.db` will be created in the current directory. Delete it to start fresh:

```bash
rm rss.db && ./target/release/cosmic-rss
```

Or do everything at once with this command:

```bash
cargo build --release && rm rss.db && ./target/release/cosmic-rss
```

---

## Project structure

```
src/
├── main.rs    # App struct, UI layout, nav sidebar, message handling
├── feed.rs    # Entry and Channel data types
├── sync.rs    # RSS fetching, background sync loop
└── db.rs      # SQLite setup, read/write helpers
```

## Adding feeds

Open `src/sync.rs` and add URLs to the `URLS` array at the top of the file:

```rust
const URLS: &[&str] = &[
    "https://example.com/feed.rss",
    // ...
];
```

Then rebuild. The new feeds will be fetched on the next sync.

---

## Dependencies

| Crate | Purpose |
|---|---|
| `libcosmic` | Core UI framework (COSMIC desktop) |
| `tokio` | For Async runtime |
| `reqwest` | HTTP client for fetching feeds |
| `rss` | For parsing RSS feed |
| `rusqlite` | SQLite database (bundled) |
| `chrono` | For date/time parsing and formatting |
| `serde` | Serialisation |
| `anyhow` | Error handling |
| `futures` | Streaming utilities |

---

## Contributing

This is primarily a personal learning project for exploring libcosmic and Rust GUI development. Issues and PRs are welcome but response time may be slow.
