# ccmux Project Handoff

> **LIVING DOCUMENT**: This handoff file is the interface between sessions. Update it constantly as you work—mark completed items, add discoveries, note blockers, revise plans. The next session (or a resumed session) relies on this being current.

## Context

**ccmux** is a Claude Code-aware terminal multiplexer in Rust. Development follows the [Context Engineering Methodology](./CONTEXT_ENGINEERING_METHODOLOGY.md).

**Current Stage**: Stage 6 (Implementation) - Wave 4 Integration
**Completed**: 20 component features (Waves 0-3) + 6/7 Wave 4 features
**Remaining**: 1 feature + 2 bugs to MVP

## Current State

Wave 4 integration is nearly complete. **1,219 tests passing.**

### What Works
- Server accepts client connections via Unix socket
- Client connects and displays session selection UI
- Full message routing (17 message types)
- PTY output broadcasting infrastructure
- Client connection registry with session tracking
- Persistence/recovery framework
- MCP server (for Claude integration)

### What's Blocking MVP
- **BUG-003**: Session creation doesn't create default window/pane (P0)
  - Client attaches to empty session, shows "No active pane"
  - Blocks all E2E testing
- **FEAT-025**: Pane output not yet wired to UI rendering

## Wave 4: Integration Features

**Goal**: Wire existing components into a working terminal multiplexer.

| ID | Feature | Priority | Effort | Status |
|----|---------|----------|--------|--------|
| FEAT-021 | Server Socket Listen Loop | P0 | 4-6h | ✅ Merged |
| FEAT-027 | Client Connection Registry | P0 | 1-2h | ✅ Merged |
| FEAT-022 | Client Message Routing | P0 | 6-8h | ✅ Merged |
| FEAT-023 | PTY Output Broadcasting | P0 | 2-3h | ✅ Merged |
| FEAT-024 | Session Selection UI | P1 | 2h | ✅ Merged |
| FEAT-025 | Pane Output Rendering | P0 | 3-4h | 🔲 Pending |
| FEAT-026 | Input Testing | P1 | 1-2h | 🔲 Pending |

## Open Bugs

| ID | Description | Priority | Status |
|----|-------------|----------|--------|
| BUG-001 | Client input not captured | P0 | ✅ Merged |
| BUG-002 | Flaky test (shared temp dir) | P2 | 🔲 Open |
| BUG-003 | Session missing default pane | P0 | 🔴 Open - BLOCKS MVP |

## Critical Path to MVP

```
BUG-003 (30 min) → FEAT-025 (3-4h) → FEAT-026 (1-2h) → HA-001 (1h)
```

**Total remaining: ~6-8 hours**

## Next Session Tasks

### Priority 1: Unblock Testing
1. **Fix BUG-003** - Session creation must auto-create window+pane+PTY
   - Location: `ccmux-server/src/handlers/session.rs:28-53`
   - After `create_session()`, create window, pane, spawn PTY

### Priority 2: Complete MVP
2. **FEAT-025** (Pane Output Rendering)
   - Wire PTY output to client pane display
   - Location: `ccmux-client/src/ui/app.rs` (stubs at lines 565, 606)

3. **FEAT-026** (Input Testing) + **HA-001** (Manual Testing)
   - End-to-end verification

### Low Priority (Can Wait)
4. **BUG-002** - Flaky test fix (P2, doesn't block MVP)

## Parallelization Options

| Parallel Track A | Parallel Track B |
|------------------|------------------|
| BUG-003 fix | BUG-002 fix |
| FEAT-025 implementation | (waits for BUG-003) |

Limited parallelism - BUG-003 must merge before testing.

## Implementation Progress

### Wave Status

| Wave | Features | Status | Tests |
|------|----------|--------|-------|
| 0 | Protocol, Utilities, Connection, Session, PTY, Config | ✅ Complete | 368 |
| 1 | Parser, Scrollback, Viewport, Worktree (3), Response, Logging, UI, Persistence | ✅ Complete | 452 |
| 2 | Client Input, Claude Detection, Sideband Protocol | ✅ Complete | 224 |
| 3 | MCP Server, Session Isolation | ✅ Complete | 49 |
| 4 | Client-Server Integration (7 features) | 🚧 6/7 Complete | 126 |

**Total Tests**: 1,219 passing

## Key Documents

| Document | Purpose |
|----------|---------|
| `WAVES.md` | Canonical wave plan with dependency graph |
| `feature-management/features/` | Wave 4 feature work items |
| `feature-management/bugs/` | Bug work items (BUG-001, 002, 003) |
| `docs/architecture/ARCHITECTURE.md` | System overview |
| `docs/architecture/CRATE_STRUCTURE.md` | Workspace layout |

## Technology Stack

- **PTY**: portable-pty 0.9
- **Parser**: vt100 0.15
- **TUI**: ratatui 0.29 + crossterm 0.28
- **Async**: tokio 1.x
- **Persistence**: okaywal (WAL) + bincode
- **Config**: notify + arc-swap

## Session Log (2026-01-09)

### Work Completed
1. Fixed MCP error handling per spec (protocol vs tool errors)
2. Added `.gitignore` for Rust project
3. Cleaned git history (922MB → 1.1MB via filter-repo)
4. Pushed clean repo to GitHub
5. Scoped Wave 4 integration work (7 features, 20-27h)
6. Created feature work items (FEAT-021 through FEAT-027)
7. Updated WAVES.md with Wave 4
8. Ran retrospective agent to validate features
9. **FEAT-021** (Server Socket Listen Loop) - merged
10. **FEAT-024** (Session Selection UI) - merged
11. **FEAT-027** (Client Connection Registry) - merged
12. **BUG-001** (Client input not captured) - fixed and merged
13. **FEAT-023** (PTY Output Broadcasting) - merged
14. **FEAT-022** (Client Message Routing) - merged
15. **HA-001** partial testing - discovered BUG-003
16. **BUG-003** filed (session missing default pane)
17. **BUG-002** work item created (flaky test)

### Key Decisions
- FEAT-027 (Connection Registry) split out as own feature
- FEAT-022 estimate raised to 6-8h (17 message types)
- BUG-003 fix should be server-side (auto-create pane)
- Discussed orchestration coupling - may generalize post-MVP

### Blockers
- **BUG-003**: Session creation doesn't create default pane
  - Blocks HA-001 manual testing
  - Blocks FEAT-025/026 verification

### Active Worktrees
| Worktree | Branch | Status |
|----------|--------|--------|
| `ccmux-wt-feat-022` | feat-022-client-message-routing | ✅ Merged (can delete) |
| `ccmux-wt-feat-023` | feat-023-pty-output-broadcasting | ✅ Merged (can delete) |

## Build & Run

```bash
# Build
cargo build --release

# Run server
./target/release/ccmux-server

# Run client (connects to server)
./target/release/ccmux

# Run MCP server mode
./target/release/ccmux-server mcp-server

# Run tests
cargo test --workspace
```

## Future Considerations

**Post-MVP discussion**: The orchestration system (FEAT-004) has methodology-specific coupling (orchestrator/worker concepts). Consider generalizing to tag-based session roles for broader usability. See session notes for analysis.
