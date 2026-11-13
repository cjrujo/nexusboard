---
name: nexusboard
description: Full Dual-Path Authority skill for any authorized AI agent to claim and execute tasks assigned to "AI", organize project tabs (priority sorting, category-grouped archiving of completed tasks, WIP balancing), and maintain NexusBoard YAML files.
---
# NexusBoard Agent Skill (v1.0)

You have **Symmetric Dual-Path Authority** on this NexusBoard workspace alongside human teammates.

## 1. Simplified Assignee Model (`AI` vs `HUMAN`)
- Tasks assigned to **`assignee.type: "AI"`** belong to the universal authorized AI pool—any authorized AI agent connected to the board is in charge of executing, updating, and completing these tasks.
- Tasks assigned to **`assignee.type: "HUMAN"`** specify `human_function` (e.g., `"Architecture"`, `"Product Design"`) and `human_name` (e.g., `"Wesley"`, `"Elena"`).

## 2. AI Smart Organization Responsibilities
Whenever you inspect or maintain a project tab (`.nexusboard/projects/<slug>.yaml`), keep the board clean and organized:
1. **Priority & Critical-Path Ordering**: Sort `backlog`, `ready`, and `in_progress` tasks by `priority` (`P0 -> P1 -> P2 -> P3`), dependency impact, and progress.
2. **Archive Completed Tasks by Category**: Move finished `done` tasks to `status: archived` tagged by `#cat:<category>` so the active columns stay clean while completed work remains organized by category (`Architecture`, `Backend`, `Frontend`, `Reliability`, etc.).
3. **Unblock & Balance WIP**: Promote unblocked `backlog` tasks to `ready` and enrich missing subtasks.
