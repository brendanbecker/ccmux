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
| **Stream A** | User Interface | `../ccmux-stream-a` | Implement TUI Mailbox widget and info pane (FEAT-073). | Ready |
| **Stream B** | UX / Safety | `../ccmux-stream-b` | Implement Adaptive Layout engine (FEAT-082). | Ready |
| **Stream C** | Observability | `../ccmux-stream-c` | Implement Backend Telemetry (FEAT-074). | Ready |

### Recent Activity (2026-01-16)

### Completed
- **BUG-041 (Robust Fix)**: Implemented first-class `ClientMessage::Paste` and server-side bracketed paste re-wrapping.
- **FEAT-078**: Per-client focus state and `should_focus` protocol flag implementation.
- **FEAT-076**: Capability signaling protocol and per-pane metadata.
- **FEAT-061**: Screen redraw command and prefix keybinding.
- **FEAT-073 (Part 2)**: Stuck detection logic, status badges, and sideband Mail command support.
- **BUG-032**: MCP handlers missing TUI broadcasts (Stream A).
- **FEAT-064**: Refactor MCP bridge.rs into modular components (Stream A).
- **BUG-033, BUG-034, BUG-035, BUG-039, BUG-040**: Core stability and validation fixes (Stream A).
- **FEAT-080, FEAT-081, FEAT-071**: Sideband Config, Landlock Sandboxing, Per-pane Claude config.
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

### Architecture & Refactoring (Protocol Normalization)
- **FEAT-085 (P1)**: ADR: The "Dumb Pipe" Strategy. (Formalize the shift away from monolithic agent OS).
- **FEAT-083 (P1)**: Protocol Generalization: Generic Widget System. (Replace hardcoded BeadsTask).
- **FEAT-084 (P2)**: Abstract Agent State. (Generalize ClaudeState).

### Strategic Features
- **FEAT-082**: Adaptive Layout (Responsive UX).
- **FEAT-076+**: Peer-to-peer sideband synchronization.

## Reference

- **Features**: `feature-management/features/features.md`
- **Bugs**: `feature-management/bugs/bugs.md`
- **Retrospective**: `feature-management/RETROSPECTIVE_2026_01_14.md`

---

## Session Log (2026-01-16) - Unified Stability & Advanced Health Logic

### Work Completed This Session
1. **Health & Dashboard Foundation (FEAT-073)**
   - Implemented `check_stuck_status` in `Pane` to detect slow/stuck agents based on activity timeouts.
   - Added status badges to TUI status bar (`[STUCK 120s]`, `[SLOW 60s]`).
   - Implemented `Mail` sideband command for workers to send summaries/intent to the dashboard.

2. **Refined UX & Protocol (FEAT-076, FEAT-078, FEAT-061)**
   - Integrated per-client focus state into all handlers, ensuring `should_focus` is respected across broadcasts.
   - Added `Redraw` message and keybinding (Prefix+R) for recovery from display corruption.
   - Added per-pane metadata storage for capability signaling and agent hints.

3. **Massive Regression Cleanup**
   - Fixed dozens of test failures across `codec.rs`, `messages.rs`, and `types.rs` caused by protocol field additions.
   - Resolved multi-way conflicts in `app.rs` and `mcp_bridge.rs` between broadcasts, focus state, and redraw logic.

### Next Steps
- **Stream A**: Implement the Mailbox UI widget in `ccmux-client`.
- **Stream B**: Proceed with the Adaptive Layout engine implementation.
- **Stream C**: Start Backend Telemetry (metrics/tracing).