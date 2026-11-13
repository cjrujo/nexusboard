# NexusBoard GUI v1.0 (Tauri v2 Desktop App)

**Dual-Surface Human × AI YAML Project Orchestration — Native Desktop Edition**

`nexusboard-gui-v1.0` wraps the exact same NexusBoard v1.0 UI (8-language 3-layer i18n, 4 themes, Time-Machine, Timeline/Gantt, Visual Calendar Date Picker, Smart Paste, Sky·Rain·Umbrella, and `window.NexusBoardAPI`) inside a lightweight **Tauri v2** native desktop shell with direct atomic `.nexusboard/projects/*.yaml` disk persistence and live external-agent file watching.

## Prerequisites (Windows / macOS / Linux)
1. **Node.js 18+** (`node -v`)
2. **Rust & Cargo** (`rustc --version`, install via [rustup.rs](https://rustup.rs/))
3. **Windows**: WebView2 is pre-installed on Windows 10/11. (Build tools: Visual Studio C++ Build Tools if compiling native binaries.)

## Quick Start

### Option A: Windows 1-Click Launch
Double-click **`Launch-NexusBoard-GUI.bat`**.

### Option B: Terminal (Windows / macOS / Linux)
```bash
npm install
npm run dev
```

### Build Native Installer (`.exe` / `.msi` / `.dmg` / `.AppImage`)
```bash
npm run build
```
Output binaries will be placed in `src-tauri/target/release/bundle/`.

## How Desktop YAML Sync Works
- By default, NexusBoard GUI opens `.nexusboard` in your current working directory (or `%USERPROFILE%\.nexusboard` / `~/.nexusboard`).
- Click **`📂 Workspace`** in the top bar to switch to any project folder containing a `.nexusboard` directory using the native OS folder picker.
- Every UI edit writes atomically (`write temp` -> `fsync` -> `rename`) to `.nexusboard/projects/<slug>.yaml`.
- A background Rust file watcher polls `.nexusboard/projects/*.yaml` every 1.2s and emits `nxb://yaml-changed` so edits made by external AI agents (`nxb` CLI, MCP server, Claude Code, Codex) appear automatically.
