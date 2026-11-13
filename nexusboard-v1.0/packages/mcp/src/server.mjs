/**
 * NexusBoard Model Context Protocol (MCP) Server v1.0
 * Supports stdio & Streamable HTTP/SSE for Claude Code, OpenAI Codex, Pi Agent, Cursor & Grok Bot
 */
export const NEXUSBOARD_MCP_TOOLS = [
  {
    name: "nxb_list_tabs",
    description: "List all project tabs and global preferences from .nexusboard/workspace.yaml",
    inputSchema: { type: "object", properties: {} }
  },
  {
    name: "nxb_get_board_yaml",
    description: "Read the authoritative YAML state of a project tab (.nexusboard/projects/<slug>.yaml)",
    inputSchema: {
      type: "object",
      properties: { tab_slug: { type: "string" } },
      required: ["tab_slug"]
    }
  },
  {
    name: "nxb_upsert_task",
    description: "Create or update a task card with full Dual-Path Authority and 3-Way CAS revision merge",
    inputSchema: {
      type: "object",
      properties: {
        tab_slug: { type: "string" },
        task_id: { type: "string" },
        title: { type: "string" },
        status: { type: "string", enum: ["backlog", "ready", "in_progress", "review", "done"] },
        priority: { type: "string", enum: ["P0", "P1", "P2", "P3"] },
        actor_id: { type: "string" },
        idempotency_key: { type: "string" }
      },
      required: ["tab_slug", "title", "actor_id"]
    }
  },
  {
    name: "nxb_batch_maintain",
    description: "Execute autonomous board maintenance (triage, subtask decomposition, DAG cycle check, WIP rebalance, cold archive)",
    inputSchema: {
      type: "object",
      properties: {
        tab_slug: { type: "string" },
        operations: { type: "array", items: { type: "string" } },
        actor_id: { type: "string" }
      },
      required: ["tab_slug", "actor_id"]
    }
  },
  {
    name: "nxb_generate_summary",
    description: "Generate scheduled or on-demand multilingual executive summary digest (.nexusboard/summaries/*.yaml)",
    inputSchema: {
      type: "object",
      properties: {
        cadence: { type: "string", enum: ["hourly", "daily", "weekly"] },
        locale: { type: "string", enum: ["en", "zh-CN", "zh-TW", "ja", "ko", "es", "fr", "de"] }
      }
    }
  },
  {
    name: "nxb_rollback_revision",
    description: "Roll back a project tab to a specific atomic YAML snapshot in .nexusboard/history/",
    inputSchema: {
      type: "object",
      properties: { tab_slug: { type: "string" }, target_revision: { type: "number" } },
      required: ["tab_slug", "target_revision"]
    }
  }
];
