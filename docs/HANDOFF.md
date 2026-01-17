# ccmux Project Handoff

> **LIVING DOCUMENT**: This handoff file is the interface between sessions. Update it constantly as you work—mark completed items, add discoveries, note blockers, revise plans.

## Context

**ccmux** is a Claude Code-aware terminal multiplexer in Rust.
**Current Stage**: Stage 7 (Post-MVP Stabilization & Enhancement)
**Status**: Production-ready core. All critical bugs resolved. Architecture generalized for any AI agent.

## Current State (2026-01-16)

**Post-Stabilization**: Major stabilization sprint completed. Protocol generalized per ADR-001 "Dumb Pipe" strategy. New P1 bug discovered (TUI flicker).

### Active Bugs

| Bug | Priority | Description |
|-----|----------|-------------|
| **BUG-048** | P1 | TUI flickers on every keystroke when Claude detected |
| BUG-047 | P3 | 51+ compiler warnings need cleanup |

### Latest Session (2026-01-16 evening)

**Completed:**
- **BUG-046**: MCP select_session/select_window now sync TUI view (session-level isolation)

**Discovered:**
- **BUG-048** (P1): TUI flickers caused by `ClaudeAgentDetector.analyze()` returning state on every call instead of only on changes. Root cause in `ccmux-server/src/agents/claude/mod.rs:127-137`. Interaction between FEAT-084 (agent abstraction) and FEAT-082 (needs_redraw on PaneStateChanged).
- **BUG-047** (P3): 51+ compiler warnings across crates (unused imports, dead code, deprecated usage)

### Previous Session Completions

| Stream | Completed Items | Status |
|--------|-----------------|--------|
| ~~A~~ | BUG-043, FEAT-063, BUG-044, BUG-037 | **Closed** |
| ~~B~~ | BUG-045, BUG-041 | **Closed** |
| ~~C~~ | BUG-031, FEAT-074, FEAT-086 | **Closed** |
| ~~D~~ | FEAT-085, FEAT-083, FEAT-084 | **Closed** |

### Recent Completions (2026-01-16)

**MCP Stability (Stream A)**:
- **BUG-043**: Unwrap `Sequenced` wrapper in `recv_response_from_daemon()`
- **FEAT-063**: File logging for MCP bridge (`~/.local/state/ccmux/log/mcp-bridge.log`)
- **BUG-044**: Async stdin I/O to prevent bridge hangs
- **BUG-037**: Unbounded channel to prevent I/O task blocking

**Client/TUI (Stream B)**:
- **BUG-045**: Render only active window panes (windows as tabs, not splits)
- **BUG-041**: Client-side bracketed paste wrapping (fixes Claude Code crashes)

**Core Daemon (Stream C)**:
- **BUG-031**: Metadata persistence via WAL
- **FEAT-074**: Prometheus-compatible metrics endpoint (`/metrics`)
- **FEAT-086**: Environment variable persistence via WAL

**Architecture (Stream D)**:
- **FEAT-085**: ADR-001 "Dumb Pipe" Strategy document
- **FEAT-083**: Generic Widget System (replaces hardcoded BeadsTask)
- **FEAT-084**: Abstract Agent State (replaces Claude-specific ClaudeState)

**Enhancement**:
- **FEAT-062**: Mirror Pane (Picture-in-Picture) for multi-agent monitoring

## Backlog Summary

### Open Bugs (2)

| Bug | Priority | Severity | Description |
|-----|----------|----------|-------------|
| **BUG-048** | P1 | high | TUI flicker - agent detector returns state on every call |
| BUG-047 | P3 | low | 51+ compiler warnings need cleanup |

### Backlog Features (6)

| Priority | Feature | Description |
|----------|---------|-------------|
| P2 | FEAT-087 | Refactor app.rs (31.8k tokens) |
| P2 | FEAT-088 | Refactor handlers/mcp_bridge.rs (27.2k tokens) |
| P3 | FEAT-089 | Refactor protocol/types.rs (20.5k tokens) |
| P3 | FEAT-090 | Refactor server/main.rs (18.3k tokens) |
| P3 | FEAT-091 | Refactor mcp/handlers.rs (17.1k tokens) |
| P3 | FEAT-092 | Refactor protocol/messages.rs (15.1k tokens) |

**Note**: BUG-048 (TUI flicker) is the only functional bug. Backlog otherwise contains only refactoring and cleanup tasks.

## Architecture Notes

### ADR-001: The Dumb Pipe Strategy (Implemented)

ccmux is now agent-agnostic:
- `Widget` type replaces hardcoded `BeadsTask`
- `AgentState` replaces Claude-specific `ClaudeState`
- External systems push data via generic widget protocol
- See: `docs/adr/ADR-001-dumb-pipe-strategy.md`

### Key Files

| Component | Location |
|-----------|----------|
| Protocol types | `ccmux-protocol/src/types.rs` |
| Widget system | `ccmux-protocol/src/types.rs` (Widget, WidgetUpdate) |
| Agent detection | `ccmux-server/src/agents/` |
| MCP bridge | `ccmux-server/src/mcp/bridge/` |
| Persistence | `ccmux-server/src/persistence/` |
| Observability | `ccmux-server/src/observability/` |

## Reference

- **Features**: `feature-management/features/features.md`
- **Bugs**: `feature-management/bugs/bugs.md`
- **ADR**: `docs/adr/ADR-001-dumb-pipe-strategy.md`
- **Retrospective**: `feature-management/RETROSPECTIVE_2026_01_14.md`

---

## Session Log (2026-01-16) - Stabilization Sprint

### Summary

Completed major stabilization sprint across 4 parallel workstreams:
- 6 bugs fixed (all P1/P2 resolved)
- 6 features implemented
- Protocol generalized for any AI agent
- All workstreams merged and closed

### Metrics

| Metric | Start of Day | End of Day |
|--------|--------------|------------|
| Open Bugs | 7 | 2 |
| P1 Bugs | 4 | 1 |
| Backlog Features | 12 | 6 |

**Note**: BUG-048 (P1 flicker) was discovered during BUG-046 testing. BUG-047 (P3 warnings) filed for tech debt.

### Key Architectural Changes

1. **Generic Widget System** - Any external system can push widgets
2. **Abstract Agent State** - Works with any AI agent, not just Claude
3. **Environment Persistence** - Full session state survives restarts
4. **Observability** - Prometheus metrics for monitoring
