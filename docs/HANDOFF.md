# ccmux Project Handoff

> **LIVING DOCUMENT**: This handoff file is the interface between sessions. Update it constantly as you work—mark completed items, add discoveries, note blockers, revise plans.

## Context

**ccmux** is a Claude Code-aware terminal multiplexer in Rust.
**Current Stage**: Stage 7 (Post-MVP Stabilization & Enhancement)
**Status**: Production-ready core, currently stabilizing extended MCP capabilities and adding agent-specific features.

## Current State (2026-01-16)

**Post-QA Assessment**: Hands-on QA testing revealed critical MCP stability issues. Three new bugs filed (BUG-043, BUG-044, BUG-045). MCP tools are intermittently unusable due to bridge hangs.

### Active Workstreams

| Stream | Name | Branch | Worktree | Current Task | Status |
|--------|------|--------|----------|--------------|--------|
| **A** | MCP Stability | `stream-a-mcp-stability` | `../ccmux-stream-a` | BUG-043: Unwrap Sequenced | **DO FIRST** |
| **B** | Client/TUI | `stream-b-client-tui` | `../ccmux-stream-b` | BUG-045: Windows as splits | Ready (after A) |
| **C** | Core Daemon | `stream-c-core-daemon` | `../ccmux-stream-c` | BUG-031: Metadata persistence | Ready (parallel) |
| **D** | Architecture | `stream-d-architecture` | `../ccmux-stream-d` | FEAT-085: ADR Dumb Pipe | Ready (parallel) |

### Execution Plan

**Streams A, C, D can run in parallel** - they touch completely different files:

| Stream | Files Modified | Can Parallel? |
|--------|----------------|---------------|
| A | `ccmux-server/src/mcp/bridge/connection.rs` | Yes |
| C | `ccmux-persistence/`, `ccmux-server/src/handlers/session.rs` | Yes |
| D | `docs/adr/ADR-001-*.md` (new file) | Yes |
| B | `ccmux-client/src/app.rs`, `ccmux-client/src/ui/` | **Wait for A** |

```
    A (BUG-043) ─────┐
    C (BUG-031) ─────┼──→ merge all ──→ B (BUG-045)
    D (FEAT-085) ────┘
```

**Stream B should wait** because verifying window rendering fixes is easier with working MCP tools (after BUG-043 is merged).

### How to Start

Each worktree has a `SESSION.md` with detailed task instructions:
```bash
cd ../ccmux-stream-a && cat SESSION.md  # BUG-043: Unwrap Sequenced
cd ../ccmux-stream-c && cat SESSION.md  # BUG-031: Metadata persistence
cd ../ccmux-stream-d && cat SESSION.md  # FEAT-085: ADR Dumb Pipe
```

Launch Claude sessions in each worktree to begin parallel work.

### Stream Details

**Stream A: MCP Stability (CRITICAL PATH)**
- **BUG-043** (P1): MCP handlers fail to unwrap `Sequenced` message wrapper. Fix: unwrap in `recv_response_from_daemon()`.
- **BUG-044** (P1): MCP bridge hangs indefinitely, stops reading stdin. Root cause unclear - async/sync mixing suspected.
- **BUG-037** (P2): close_pane returns AbortError.
- **FEAT-063** (P1): Add file logging to MCP bridge for debugging.

**Stream B: Client/TUI**
- **BUG-041** (P1): Claude Code crashes on paste inside ccmux.
- **BUG-045** (P2): Windows rendered as horizontal splits instead of tabs.
- **FEAT-061** (P2): Screen redraw command.
- **FEAT-073** (P2): Visibility dashboard (stuck detection, mailbox).

**Stream C: Core Daemon**
- **BUG-031** (P1): Metadata not persisting across restarts.

**Stream D: Architecture**
- **FEAT-064/065** (P2): Refactor MCP bridge into modular components.
- **FEAT-083** (P1): Protocol Generalization - Generic Widget System.
- **FEAT-084** (P2): Protocol Generalization - Abstract Agent State.
- **FEAT-085** (P1): ADR: The "Dumb Pipe" Strategy.

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

## Backlog Summary

### Open Bugs (7)
| Priority | Bugs |
|----------|------|
| P1 | BUG-043, BUG-044, BUG-041, BUG-031 |
| P2 | BUG-045, BUG-037 |
| P3 | BUG-042 |

### Backlog Features (13)
| Priority | Features |
|----------|----------|
| P1 | FEAT-063, FEAT-083, FEAT-085 |
| P2 | FEAT-061, FEAT-064, FEAT-065, FEAT-073, FEAT-074, FEAT-084 |
| P3 | FEAT-062, FEAT-076, FEAT-058, FEAT-059 |

### Critical Path
1. **BUG-043** → Simple fix, unblocks MCP response parsing
2. **FEAT-063** → Enables debugging for BUG-044
3. **BUG-044** → Fixes MCP bridge hang (may be complex)
4. Then: Stream B/C/D work can proceed in parallel

## Reference

- **Features**: `feature-management/features/features.md`
- **Bugs**: `feature-management/bugs/bugs.md`
- **Retrospective**: `feature-management/RETROSPECTIVE_2026_01_14.md`

---

## Session Log (2026-01-16) - QA Testing & Bug Discovery

### Work Completed This Session

1. **Hands-on QA Testing**
   - Attempted to run DEMO-QA.md comprehensive test suite.
   - MCP tools work initially but bridge hangs after several operations.
   - Discovered response parsing issues with `Sequenced` message wrapper.

2. **Bug Discovery & Filing**
   - **BUG-043**: MCP handlers fail to unwrap `Sequenced` wrapper from daemon responses. Root cause identified in `is_broadcast_message()` and `recv_response_from_daemon()`.
   - **BUG-044**: MCP bridge hangs indefinitely, stops reading stdin. Socket analysis showed 1323 bytes stuck in stdin queue, 6912 bytes from Claude Code blocked. Killing bridge and respawning works as workaround.
   - **BUG-045**: Windows rendered as horizontal splits instead of tabs. `create_window` splits the view rather than creating a separate tab.

3. **Workstream Reorganization**
   - Reorganized backlog into 4 focused streams (A: MCP Stability, B: Client/TUI, C: Core Daemon, D: Architecture).
   - Stream A is critical path - MCP is currently unreliable.

### Key Findings

**BUG-043 Root Cause** (confirmed):
- Daemon wraps responses in `ServerMessage::Sequenced { seq, inner }` for persistence (FEAT-075).
- `is_broadcast_message()` doesn't include `Sequenced` variant.
- Tool handlers receive `Sequenced { inner: PaneList }` but expect `PaneList`.
- Fix: Unwrap `Sequenced` in `recv_response_from_daemon()`.

**BUG-044 Root Cause** (suspected):
- Main loop uses sync `stdin.lock().lines()` inside async function.
- When `handle_request().await` blocks, stdin reading stops.
- 25-second timeout not triggering despite code being present.
- May be related to BUG-043 causing unexpected control flow.

### Next Steps
- **Immediate**: Fix BUG-043 (simple fix, high impact).
- **Then**: Add FEAT-063 (MCP bridge logging) to diagnose BUG-044.
- **Then**: Fix BUG-044 (may require async stdin refactor).

---

## Session Log (2026-01-16 earlier) - Adaptive Layout & Unified Core

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
