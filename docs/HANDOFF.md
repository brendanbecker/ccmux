# ccmux Project Handoff

> **LIVING DOCUMENT**: This handoff file is the interface between sessions. Update it constantly as you work—mark completed items, add discoveries, note blockers, revise plans. The next session (or a resumed session) relies on this being current.

## Context

**ccmux** is a Claude Code-aware terminal multiplexer in Rust. Development follows the [Context Engineering Methodology](./CONTEXT_ENGINEERING_METHODOLOGY.md).

**Current Stage**: Stage 6 (Implementation) - Wave 1 In Progress
**Completed**: Wave 0 + 4 Wave 1 features (10 of 20 features implemented)

## Implementation Progress

### Wave Status

| Wave | Features | Status |
|------|----------|--------|
| 0 | Protocol, Utilities, Connection, Session, PTY, Config | ✅ Complete |
| 1 | Pane Content, Scrollback, Viewport, Worktree, Response Channel, Logging, Client UI, Terminal Parsing, Persistence | ⏳ In Progress (4/9) |
| 2 | Client Input, Claude Detection, Sideband Protocol | ⏸️ Blocked by Wave 1 |
| 3 | MCP Server, Session Isolation | ⏸️ Blocked by Wave 2 |

### Feature Implementation Status

| ID | Feature | Component | Status | Tests | Priority |
|----|---------|-----------|--------|-------|----------|
| FEAT-001 | Pane Content Abstraction | session/pane | ⏳ Wave 1 | - | P1 |
| FEAT-002 | Per-Session Scrollback Config | config | ✅ Done | 47 | P1 |
| FEAT-003 | Viewport Pinning | tui | ✅ Done | 23 | P2 |
| FEAT-004 | Worktree-Aware Orchestration | orchestration | ⏳ Wave 1 | - | P2 |
| FEAT-005 | Response Channel | orchestration | ✅ Done | 72 | P1 |
| FEAT-006 | Per-Session Log Levels | logging | ✅ Done | 40 | P2 |
| FEAT-007 | Protocol Layer | ccmux-protocol | ✅ Done | 86 | P1 |
| FEAT-008 | Utilities | ccmux-utils | ✅ Done | 108 | P1 |
| FEAT-009 | Client UI | ccmux-client | ⏳ Wave 1 | - | P1 |
| FEAT-010 | Client Input | ccmux-client | ⏸️ Wave 2 | - | P1 |
| FEAT-011 | Client Connection | ccmux-client | ✅ Done | 31 | P1 |
| FEAT-012 | Session Management | ccmux-server | ✅ Done | 88 | P1 |
| FEAT-013 | PTY Management | ccmux-server | ✅ Done | 17 | P1 |
| FEAT-014 | Terminal Parsing | ccmux-server | ⏳ Wave 1 | - | P1 |
| FEAT-015 | Claude Detection | ccmux-server | ⏸️ Wave 2 | - | P1 |
| FEAT-016 | Persistence | ccmux-server | ⏳ Wave 1 | - | P2 |
| FEAT-017 | Configuration | ccmux-server | ✅ Done | 38 | P1 |
| FEAT-018 | MCP Server | ccmux-server | ⏸️ Wave 3 | - | P2 |
| FEAT-019 | Sideband Protocol | ccmux-server | ⏸️ Wave 2 | - | P2 |
| FEAT-020 | Session Isolation | ccmux-server | ⏸️ Wave 3 | - | P1 |

**Total Tests**: 566 passing

## Orchestration Pattern

This project uses **git worktrees** for parallel feature development:

1. Create worktree per feature: `git worktree add -b feature/FEAT-XXX-name ../ccmux-wt-name main`
2. Write `SESSION_PROMPT.md` in each worktree with implementation instructions
3. Launch parallel Claude Code sessions, one per worktree
4. Merge branches back to main after wave completion
5. Run test-runner agent to validate and add tests
6. Repeat for next wave

## Wave 1 Features (In Progress)

4 of 9 features complete. Remaining 5 can be developed **in parallel**.

### Critical Path (P1) - Remaining

| ID | Feature | Component | Dependencies | Effort |
|----|---------|-----------|--------------|--------|
| FEAT-009 | Client UI | ccmux-client | FEAT-007, FEAT-011 | large |
| FEAT-014 | Terminal Parsing | ccmux-server | FEAT-013 | medium |
| FEAT-001 | Pane Content Abstraction | session/pane | none | large |

### Additional (P2) - Remaining

| ID | Feature | Component | Dependencies | Effort |
|----|---------|-----------|--------------|--------|
| FEAT-016 | Persistence | ccmux-server | FEAT-012 | large |
| FEAT-004 | Worktree Orchestration | orchestration | none | xl |

### Completed This Wave

| ID | Feature | Tests |
|----|---------|-------|
| FEAT-002 | Per-Session Scrollback | 47 |
| FEAT-003 | Viewport Pinning | 23 |
| FEAT-005 | Response Channel | 72 |
| FEAT-006 | Per-Session Logging | 40 |

## Critical Path

```
FEAT-013 (done) → FEAT-014 → FEAT-015 → FEAT-020
```

FEAT-014 and FEAT-015 are on the critical path—prioritize these to minimize total completion time.

## Completed Work

### Stage 1-4: Research & Architecture
- Deep research from 3 LLMs (Claude, Gemini, ChatGPT)
- Parsed into `docs/research/parsed/` and `SYNTHESIS.md`
- Architecture docs in `docs/architecture/`
- 3 ADRs for key decisions

### Stage 6: Implementation

**2026-01-08 - Wave 1 Partial (4/9 features)**
- Merged 4 feature branches from worktrees:
  - `feature/FEAT-002-scrollback`: Per-session scrollback configuration (47 tests)
  - `feature/FEAT-003-viewport`: Viewport pinning with ViewportState protocol type (23 tests)
  - `feature/FEAT-005-response`: Response channel with PaneTarget, ReplyMessage, ReplyResult (72 tests)
  - `feature/FEAT-006-logging`: Per-session log levels (40 tests)
- Resolved merge conflicts in protocol types (combined ViewportState + Reply types)
- Test count: 384 → 566 (+182 tests)
- Remaining Wave 1: FEAT-001, FEAT-004, FEAT-009, FEAT-014, FEAT-016

**2026-01-08 - Wave 0 Complete**
- Merged 6 feature branches:
  - `feature/FEAT-007-protocol`: IPC messages and codec (86 tests)
  - `feature/FEAT-008-utilities`: Error types, logging, XDG paths (108 tests)
  - `feature/FEAT-011-connection`: Unix socket client with async I/O (31 tests)
  - `feature/FEAT-012-session`: Session/Window/Pane hierarchy (88 tests)
  - `feature/FEAT-013-pty`: portable-pty integration (17 tests)
  - `feature/FEAT-017-config`: Hot-reload config with ArcSwap (38 tests)
- Initialized 4-crate workspace structure
- All tests passing, no clippy warnings

## Key Documents

| Document | Purpose |
|----------|---------|
| `WAVES.md` | Canonical wave plan with dependency graph |
| `docs/architecture/ARCHITECTURE.md` | System overview |
| `docs/architecture/CRATE_STRUCTURE.md` | Workspace layout |
| `docs/FEATURE_HANDOFF.md` | Parallel task: featmgmt backfill |

## Technology Stack

- **PTY**: portable-pty 0.9
- **Parser**: vt100 0.15
- **TUI**: ratatui 0.29 + crossterm 0.28
- **Async**: tokio 1.x
- **Persistence**: okaywal (WAL) + bincode
- **Config**: notify + arc-swap

## Note on Feature Management

Features are tracked informally in this file. The canonical wave structure is in `WAVES.md`.
A parallel effort is backfilling the formal `feature-management/` system - see `docs/FEATURE_HANDOFF.md`.
