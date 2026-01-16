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
| **Stream A** | User Interface | `../ccmux-stream-a` | Implement TUI Mailbox widget and info pane (FEAT-073). | **Completed** (Ready to Merge) |
| **Stream B** | UX / Safety | `../ccmux-stream-b` | Implement Collaborative Editing Infra (FEAT-083). | Ready |
| **Stream C** | Observability | `../ccmux-stream-c` | Implement Backend Telemetry (FEAT-074). | **Needs Attention** (Re-init required) |

### Recent Activity (2026-01-16)

### Completed
- **FEAT-082**: Adaptive layout engine (Responsive UI).
- **BUG-041 (Robust Fix)**: Implemented first-class `ClientMessage::Paste` and server-side bracketed paste re-wrapping.
- **FEAT-078**: Per-client focus state and `should_focus` protocol flag implementation.
- **FEAT-076**: Capability signaling protocol and per-pane metadata.
- **FEAT-061**: Screen redraw command and prefix keybinding.
- **FEAT-073 (Part 2)**: Stuck detection logic, status badges, and sideband Mail command support.
- **BUG-032, BUG-033, BUG-034, BUG-035, BUG-039, BUG-040**: Core stability and validation fixes.
- **FEAT-080, FEAT-081, FEAT-071**: Sideband Config, Landlock Sandboxing, Per-pane Claude config.
- **FEAT-066, FEAT-067, FEAT-068, FEAT-070**: Remote Peering and Gastown integration complete.
- **FEAT-075**: Snapshot + replay resync API complete.

### In Progress
- **FEAT-073 (P2)**: Visibility dashboard TUI views (Stream A).
- **FEAT-074 (P2)**: Telemetry and observability dashboard (Stream C).

## Backlog Highlights

### High Priority (P0/P1)
- **FEAT-074**: Backend Telemetry (Crucial for large-scale orchestration monitoring).
- **BUG-039**: MCP tools hang intermittently (Added robust logging, need to monitor).

### Architecture & Refactoring (Protocol Normalization)
- **FEAT-085 (P1)**: ADR: The "Dumb Pipe" Strategy. (Formalize the shift away from monolithic agent OS).
- **FEAT-083 (P1)**: Protocol Generalization: Generic Widget System. (Replace hardcoded BeadsTask).
- **FEAT-084 (P2)**: Abstract Agent State. (Generalize ClaudeState).

### Strategic Features
- **FEAT-083**: Collaborative Editing Infrastructure (Stream B).
- **FEAT-076+**: Peer-to-peer sideband synchronization.

## Reference

- **Features**: `feature-management/features/features.md`
- **Bugs**: `feature-management/bugs/bugs.md`
- **Retrospective**: `feature-management/RETROSPECTIVE_2026_01_14.md`

---

## Session Log (2026-01-16) - Adaptive Layout & Unified Core

### Work Completed This Session
1. **Adaptive Layout Engine (FEAT-082)**
   - Implemented `Adaptive` layout policy that scales pane sizes based on focus and agent activity (Thinking/Coding/ToolUse).
   - Added layout policy cycling (Prefix+E) and `layout-policy` CLI command.
   - Integrated pane weights calculation into the main rendering loop.

2. **Core Unification**
   - Merged Stream B (Adaptive Layout) and Stream A (Health Logic) into `main`.
   - Resolved multi-way conflicts in `app.rs`, `mcp_bridge.rs`, and `registry.rs`.
   - All tests passing (1,645).

3. **Strategic Planning**
   - Defined work items for Protocol Generalization (FEAT-083, FEAT-084) and "Dumb Pipe" ADR (FEAT-085).

### Next Steps
- **Immediate Action**: Merge Stream A (Dashboard UI) into main. Expect conflicts in `app.rs` due to Adaptive Layout changes.
- **Stream C Recovery**: Reset Stream C (`git checkout main && git reset --hard origin/main`) to clear the stalled cherry-pick state before starting Telemetry work.
- **Strategic**: Begin implementing the "Dumb Pipe" ADR (FEAT-085).
