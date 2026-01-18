# AGENTS.md - ccmux Project Instructions

## Build & Test

```bash
cargo build           # Build all crates
cargo test            # Run all tests
cargo clippy          # Lint
```

## Architecture

ccmux is a Rust terminal multiplexer with MCP integration for AI agent orchestration.

**Key crates:**
- `ccmux-server/` - Daemon with MCP bridge
- `ccmux-client/` - TUI client
- `ccmux-protocol/` - Shared types
- `ccmux-utils/` - Utilities

**Key locations:**
- MCP handlers: `ccmux-server/src/mcp/bridge/handlers.rs`
- MCP tool schemas: `ccmux-server/src/mcp/tools.rs`
- PTY handling: `ccmux-server/src/pty/`
- Session management: `ccmux-server/src/session/`

## Status Reporting (IMPORTANT)

When running inside ccmux with MCP tools available, **report your status** to enable orchestration awareness:

### Required Status Reports

| When | Call |
|------|------|
| Starting work | `ccmux_report_status` with `status: "working"` |
| Need user input | `ccmux_report_status` with `status: "waiting_for_input"` |
| Blocked on approval | `ccmux_report_status` with `status: "blocked"` |
| Task complete | `ccmux_report_status` with `status: "complete"` |
| Error encountered | `ccmux_report_status` with `status: "error"` |

### Example

```json
// When starting a task
{"tool": "ccmux_report_status", "input": {"status": "working", "message": "Implementing FEAT-096"}}

// When blocked and need help
{"tool": "ccmux_request_help", "input": {"context": "Cannot find the module to modify"}}

// When done
{"tool": "ccmux_report_status", "input": {"status": "complete", "message": "FEAT-096 implemented and tested"}}
```

### Why This Matters

Orchestrators monitor worker sessions via status. Without status reports, the orchestrator cannot:
- Know when you're done
- Know when you need input
- Route help requests
- Aggregate progress

## Orchestration Tags

Sessions can be tagged for message routing:

```json
// Mark yourself as a worker
{"tool": "ccmux_set_tags", "input": {"add": ["worker", "feat-096"]}}

// Send message to orchestrator
{"tool": "ccmux_send_orchestration", "input": {
  "target": {"tag": "orchestrator"},
  "msg_type": "task.progress",
  "payload": {"percent": 50, "current_step": "implementing polling loop"}
}}
```

## Working in Worktrees

This project uses git worktrees for parallel development. Check your branch:

```bash
git branch --show-current
```

Ensure you're on the correct feature/bug branch before committing.

## Feature Management

- Features: `feature-management/features/`
- Bugs: `feature-management/bugs/`
- Each has a `PROMPT.md` with implementation spec

## Commit Convention

```
<type>: <description>
```

Types: `feat`, `fix`, `refactor`, `docs`, `test`, `chore`
