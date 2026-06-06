# 🖥️ Maestri Linux — Infinite Canvas for AI Coding Agents

<p align="center">
  <strong>BUILD. CONNECT. AUTOMATE.</strong>
</p>

<p align="center">
  <a href="#"><img src="https://img.shields.io/github/v/release/eluvju/maestri-linux?include_prereleases&style=for-the-badge" alt="GitHub release"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge" alt="MIT License"></a>
</p>

**Maestri Linux** is an _infinite canvas_ desktop app where AI coding agents run in terminal nodes connected via PTY pipes. Drop terminals, wire them together, and watch agents collaborate in real time — all locally on your Linux machine.

Inspired by [Maestri.app](https://maestri.app) for macOS. Built from scratch for Linux with **Tauri**, **Rust**, **Svelte 5**, and **xterm.js**.

[GitHub](https://github.com/eluvju/maestri-linux)

## Install

### AppImage (recommended)

```bash
# Download the latest AppImage from releases
chmod +x maestri-linux-x86_64.AppImage
./maestri-linux-x86_64.AppImage
```

### From source

```bash
git clone https://github.com/eluvju/maestri-linux.git
cd maestri-linux

npm install
npm run tauri build

# Binary at src-tauri/target/release/maestri-linux
```

## Quick start

```bash
./maestri-linux
```

- **Double-click** the canvas to create a terminal node.
- **Drag** nodes to arrange them on the infinite canvas.
- **Ctrl+Click** a port to connect nodes (terminal → terminal pipes).
- **Right-click** a connection to remove it.
- **Delete** button removes the active node.

Each terminal spawns a real PTY (pseudo-terminal) running your default shell. Type commands in any terminal — output streams to connected nodes.

## Highlights

- **Infinite canvas** — pan/zoom with DOM-based transforms, no limits.
- **Real PTY terminals** — each node runs a real shell via `portable-pty`.
- **PTY pipes** — wire stdout/stderr of one terminal into stdin of another.
- **xterm.js** — battle-tested terminal emulator (VS Code engine).
- **Local-first** — no telemetry, no accounts, fully offline.
- **Tiny binary** — ~10MB Rust + ~80MB AppImage, minimal dependencies.

## Architecture

```
┌─────────────────────────────────────────────┐
│  Tauri v2 (Rust backend)                     │
│  ┌──────────┐  ┌──────────┐  ┌───────────┐  │
│  │  state   │  │   pty    │  │   pipe    │  │
│  │  (graph) │  │ (PTY mgr)│  │(pipe eng) │  │
│  └──────────┘  └──────────┘  └───────────┘  │
│         │            │              │        │
│         └──── IPC Events (Tauri) ────┘        │
├───────────────────┬─────────────────────────┤
│  Svelte 5 (frontend)                         │
│  ┌──────────┐ ┌────────┐ ┌──────────────┐   │
│  │  Canvas  │ │Terminal│ │ Connections  │   │
│  │ (pan/zoom)│ │(xterm) │ │ (SVG bezier) │   │
│  └──────────┘ └────────┘ └──────────────┘   │
└─────────────────────────────────────────────┘
```

## Development

```bash
npm run tauri dev    # Dev mode with hot reload
npm run tauri build  # Release build
```

Prerequisites: Rust, Node.js 20+, and [Tauri system deps](https://v2.tauri.app/start/prerequisites/).

## License

MIT
