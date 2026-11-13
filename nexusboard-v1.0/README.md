# NexusBoard v1.0 — Human × AI Agent Collaboration Kanban Platform

**Symmetric Dual-Path Authority • Multi-Tab Pure YAML Engine • 4-Channel Agent Access (`MCP` / `REST` / `Skill` / `CLI`) • Local, VPS & Cloudflare Workers/Pages**

## Quick Start (Zero Dependencies Required for Local CLI & Test Suite)

```bash
# 1. List all Project Tabs from .nexusboard/workspace.yaml
node packages/cli/src/nxb.mjs tab ls

# 2. List tasks in Core Platform tab (compact / yaml / json)
node packages/cli/src/nxb.mjs ls --tab core-platform

# 3. Add a task via CLI as Pi Agent or Claude Code
node packages/cli/src/nxb.mjs add --tab core-platform --title "Add WebSocket delta compression" --priority P0 --agent pi-agent

# 4. Move a task with Full Dual-Path Authority (Human or Agent)
node packages/cli/src/nxb.mjs mv CORE-101 done --tab core-platform --by claude-code --reason "All CAS concurrency tests passing"

# 5. Run 4-Stage Autonomous AI Board Maintenance
node packages/cli/src/nxb.mjs maintain --tab core-platform --all

# 6. Generate Regular Executive Summary Digest (YAML + Markdown)
node packages/cli/src/nxb.mjs summary --cadence daily --locale en

# 7. Run 3-Way Task-Level CAS Concurrency & Self-Healing Quarantine Test Suite
node packages/core-yaml/src/test-cas.mjs
```
