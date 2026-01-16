# ccmux Project Handoff

> **LIVING DOCUMENT**: This handoff file is the interface between sessions. Update it constantly as you work—mark completed items, add discoveries, note blockers, revise plans.

## Context

**ccmux** is a Claude Code-aware terminal multiplexer in Rust.
**Current Stage**: Stage 7 (Post-MVP Stabilization & Enhancement)
**Status**: Production-ready core, currently stabilizing extended MCP capabilities and adding agent-specific features.

## Current State (2026-01-16)

**All Parallel Streams Merged**. The codebase is currently unified and stable.

### Active Workstreams (Next Tasks)

| Stream | Focus | Worktree | Objective | Status |
|--------|-------|----------|-----------|--------|
| **Stream A** | User Interface | `../ccmux-stream-a` | Complete Visibility dashboard (intent log, graph view). | Ready |
| **Stream B** | UX / Safety | `../ccmux-stream-b` | Implement Adaptive Layout engine. | Ready |
| **Stream C** | Observability | `../ccmux-stream-c` | Implement Backend Telemetry (FEAT-074). | Ready |

### Recent Activity (2026-01-16)

### Completed
- **BUG-041 (Robust Fix)**: Implemented first-class `ClientMessage::Paste` and server-side bracketed paste re-wrapping (Stream C).
- **FEAT-078**: Per-client focus state and `should_focus` flag implementation (Stream B).
- **FEAT-076**: Capability signaling protocol and per-pane metadata (Stream B).
- **FEAT-061**: Screen redraw command and prefix keybinding (Stream B).
- **FEAT-073 (Part 1)**: Pane stuck detection logic implemented in `ccmux-server` (Stream A).
- **BUG-032**: MCP handlers missing TUI broadcasts (Stream A).
- **FEAT-064**: Refactor MCP bridge.rs into modular components (Stream A).
- **BUG-033, BUG-034, BUG-035, BUG-039, BUG-040**: Core stability and validation fixes (Stream A).
- **FEAT-080, FEAT-081, FEAT-071**: Sideband Config, Landlock Sandboxing, Per-pane Claude config (Stream C).
- **FEAT-066, FEAT-067, FEAT-068, FEAT-070**: Remote Peering and Gastown integration complete.
- **FEAT-075**: Snapshot + replay resync API complete.

### In Progress
- **FEAT-073 (P2)**: Visibility dashboard TUI views (Stream A).
- **FEAT-082 (P2)**: Adaptive layout engine (Stream B).
- **FEAT-074 (P2)**: Telemetry and observability dashboard (Stream C).

## Backlog Highlights

### High Priority (P0/P1)
- **FEAT-074**: Backend Telemetry (Crucial for large-scale orchestration monitoring).
- **BUG-039**: MCP tools hang intermittently (Added robust logging, need to monitor).

### Strategic Features
- **FEAT-082**: Adaptive Layout (Responsive UX).
- **FEAT-076+**: Peer-to-peer sideband synchronization.

## Reference

- **Features**: `feature-management/features/features.md`
- **Bugs**: `feature-management/bugs/bugs.md`
- **Retrospective**: `feature-management/RETROSPECTIVE_2026_01_14.md`

---

## Session Log (2026-01-16) - Unified Core Stability

### Work Completed This Session
1. **Unification Merge**
   - Merged all three parallel worktrees into `main`.
   - Resolved complex conflicts in `app.rs`, `mcp_bridge.rs`, `session.rs`, and `protocol/types.rs`.
   - Fixed multiple test regression failures caused by protocol field additions (`stuck_status`, `should_focus`, `metadata`).

2. **Robust Paste (BUG-041)**
   - Moved paste handling from manual client-side wrapping to a formal `ClientMessage::Paste`.
   - Server now intelligently re-wraps paste data in bracketed markers based on pane state.

3. **Client Focus (FEAT-078)**
   - Switched from global active session/window/pane to per-client focus state.
   - Allows multiple users/agents to view different parts of the same session without interference.

4. **Stuck Detection (FEAT-073)**
   - Implemented logic to detect when Claude agents are "stuck" (e.g., thinking for >2m or tool-using for >5m).
   - This state is now part of `PaneInfo` and will drive the Visibility Dashboard.

### Next Steps
- **Stream A**: Implement the Visibility Dashboard TUI views.
- **Stream B**: Start the Adaptive Layout engine implementation.
- **Stream C**: Proceed with Backend Telemetry (FEAT-074).
