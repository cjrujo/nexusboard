# NexusBoard v1.0 — Dual-Surface Human × AI YAML Project Orchestration

**NexusBoard** is a database-free, YAML-native Human × AI collaboration Kanban and project orchestration platform. Both Humans (via the Visual Web UI or Desktop GUI) and external AI agents (`Claude Code`, `OpenAI Codex`, `Pi Agent`, `Cursor`, `nxb` CLI, MCP, REST) manipulate the exact same `.nexusboard/projects/<slug>.yaml` workspace with symmetric read/write/organize authority.

## Repository Structure

- **`nexusboard-v1.0/`** — Web Service, Core YAML Engine (3-Way CAS Merger & 256-bucket sharding), `nxb` CLI, MCP Server, REST API Worker, and Agent `SKILL.md`.
- **`nexusboard-gui-v1.0/`** — Native **Tauri v2** Desktop Application (Windows / macOS / Linux) with atomic disk YAML persistence, background file watcher, system tray, and global Quick-Capture (`Ctrl+Shift+Space` / `Alt+Shift+Space`).

## Key Features

- **Pure YAML Multi-Tab Storage**: Every project tab maps 1-to-1 to `.nexusboard/projects/<slug>.yaml` (supports deleting even the last tab).
- **Custom & Standard Kanban Columns**: Standard **Backlog**, **ToDo**, **In Progress**, **Done** columns with inline add, rename, reorder (`◀`/`▶`), WIP limits, and safe delete.
- **Drag-and-Drop + In-Column Sorting**: Move cards across columns or drop onto another card to set exact manual order.
- **5 Views**: Kanban Board, AI vs Human Swimlanes, Dependency Graph, Spreadsheet Table, and Timeline/Gantt (with exact Start/Due dates & urgency badges) + Cross-Tab Analytics.
- **Keyboard-First Productivity**:
  - `N`: New Task | `Q`: Quick-Capture | `Delete`/`Backspace`: Delete selected or hovered task
  - `Ctrl/Cmd+Z`: Instant Undo (plus `↩️ Undo` toast button) | `Ctrl/Cmd+Enter`: Save modal
  - `Ctrl/Cmd+K`: Command Palette | `E`: Live YAML Drawer | `D`: Card Density | `F`: Focus Mode | `?`: Shortcut Cheat Sheet
- **8-Language 3-Layer i18n & 4 Themes**: `en`, `zh-CN`, `zh-TW`, `ja`, `ko`, `es`, `fr`, `de` across all headers, columns, cards, and modals.
- **Human × AI Collaboration Tools**: YAML Time-Machine, Live Activity Feed & Revert, Sky·Rain·Umbrella (`空·雨·伞`) Radar, Smart Paste, Tab Scaffolder, Context Packets, Handoff Receipts, Claim Leases, Outbound Webhooks, and `.yaml`/`.md`/`.csv` Export & Import.

## Quick Start

### 1. Web Service & CLI (`nexusboard-v1.0`)
```bash
cd nexusboard-v1.0
node packages/cli/src/nxb.mjs serve --port 4300
# Open http://localhost:4300
```

### 2. Desktop App (`nexusboard-gui-v1.0`, Tauri v2)
Requires **Node.js 18+** and **Rust/Cargo**:
```bash
cd nexusboard-gui-v1.0
npm install
npm run dev     # Launch desktop window
npm run build   # Build native installer (.exe / .msi / .dmg / .AppImage)
```
On Windows, you can also double-click `nexusboard-gui-v1.0/Launch-NexusBoard-GUI.bat`.
