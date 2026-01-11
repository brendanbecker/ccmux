# Feature Tracking

**Project**: ccmux
**Last Updated**: 2026-01-11

## Summary Statistics

- **Total Features**: 59
- **Completed**: 55
- **Backlog**: 4

## Current Status

Core terminal multiplexer features complete. Four features remain in backlog focused on agent orchestration and beads integration.

**Recent Completions (2026-01-11)**:
- FEAT-028: Tag-based routing (replaces binary orchestrator/worker model)
- FEAT-036: Session-aware MCP commands (auto-defaults, pane/window naming)
- FEAT-050: Session metadata storage (arbitrary key-value metadata)
- FEAT-056: User priority lockout for MCP focus control

### Backlog Features

| ID | Title | Component | Priority | Status | Notes |
|----|-------|-----------|----------|--------|-------|
| FEAT-048 | Expose orchestration protocol via MCP tools | ccmux-server (MCP) | P2 | **Ready** | UNBLOCKED - FEAT-028 complete. Now uses tag-based routing. |
| FEAT-057 | Beads Passive Awareness | ccmux-server | P2 | **Ready** | Can use FEAT-050 metadata for state storage |
| FEAT-058 | Beads Query Integration | ccmux-server, ccmux-client | P3 | Blocked | Depends on FEAT-057. Can leverage FEAT-050/028 |
| FEAT-059 | Beads Workflow Integration | ccmux-server, ccmux-protocol | P3 | Blocked | SCOPE REDUCED - FEAT-050 provides core infrastructure |

### Backlog Dependencies

```
FEAT-048 (Ready)
  |-- FEAT-028 (complete) - Tag-based routing
  |-- FEAT-050 (complete) - Metadata storage

FEAT-057 (Ready)
  |-- FEAT-047 (complete) - Environment variables
  |-- FEAT-050 (complete) - Metadata storage

FEAT-058 (Blocked)
  |-- FEAT-057 (not started) - Beads detection
  |-- FEAT-050 (complete) - Metadata caching
  |-- FEAT-028 (complete) - Orchestration notifications

FEAT-059 (Blocked)
  |-- FEAT-057 (not started) - Beads detection
  |-- FEAT-058 (not started) - Daemon communication
  |-- FEAT-050 (complete) - Issue tracking infrastructure
  |-- FEAT-028 (complete) - Orchestration integration
```

### Implementation Priority

1. **FEAT-048** or **FEAT-057** - Both ready, no blocking dependencies
2. **FEAT-058** - After FEAT-057
3. **FEAT-059** - After FEAT-058 (scope significantly reduced by FEAT-050)

## Completed Features

All completed features are in `feature-management/completed/`. Here's a summary by wave:

### Wave 0: Foundation (6 features)
| ID | Title | Status |
|----|-------|--------|
| FEAT-007 | Protocol Layer - IPC Message Types and Codec | Complete |
| FEAT-008 | Utilities - Error Types, Logging, Path Helpers | Complete |
| FEAT-011 | Client Connection - Unix Socket Client | Complete |
| FEAT-012 | Session Management - Session/Window/Pane Hierarchy | Complete |
| FEAT-013 | PTY Management - Process Spawning and Lifecycle | Complete |
| FEAT-017 | Configuration - TOML Config with Hot Reload | Complete |

### Wave 1: Core Components (9 features)
| ID | Title | Status |
|----|-------|--------|
| FEAT-009 | Client UI - Ratatui Terminal Interface | Complete |
| FEAT-014 | Terminal Parsing - ANSI/VT100 State Machine | Complete |
| FEAT-016 | Persistence - Checkpoint and WAL for Crash Recovery | Complete |
| FEAT-001 | Pane Content Abstraction (Terminal vs Canvas) | Complete |
| FEAT-002 | Per-Session-Type Scrollback Configuration | Complete |
| FEAT-003 | Viewport Pinning with New Content Indicator | Complete |
| FEAT-005 | Response Channel for Orchestrator-Worker | Complete |
| FEAT-006 | Per-Session Log Levels and Storage | Complete |
| FEAT-004 | Worktree-Aware Orchestration | Complete |

### Wave 2: Input & Detection (3 features)
| ID | Title | Status |
|----|-------|--------|
| FEAT-010 | Client Input - Keyboard and Mouse Event Handling | Complete |
| FEAT-015 | Claude Detection - State Detection from PTY Output | Complete |
| FEAT-019 | Sideband Protocol - XML Command Parsing | Complete |

### Wave 3: Integration (2 features)
| ID | Title | Status |
|----|-------|--------|
| FEAT-018 | MCP Server - Model Context Protocol Integration | Complete |
| FEAT-020 | Session Isolation - Per-Pane CLAUDE_CONFIG_DIR | Complete |

### Wave 4: Client-Server Integration (7 features)
| ID | Title | Status |
|----|-------|--------|
| FEAT-021 | Server Socket Listen Loop | Complete |
| FEAT-022 | Client Message Routing and Handlers | Complete |
| FEAT-023 | PTY Output Polling and Broadcasting | Complete |
| FEAT-024 | Session Selection UI | Complete |
| FEAT-025 | Pane Output Rendering | Complete |
| FEAT-026 | Input Handling Integration and Testing | Complete |
| FEAT-027 | Client Connection Registry | Complete |

### Post-MVP Features (24 features)
| ID | Title | Status |
|----|-------|--------|
| FEAT-029 | MCP Natural Language Terminal Control | Complete |
| FEAT-030 | Sideband Pane Splitting | Complete |
| FEAT-031 | Session Delete Keybind (Ctrl+D) | Complete |
| FEAT-032 | Integrated MCP Server | Complete |
| FEAT-033 | tmux-like Auto-Start Behavior | Complete |
| FEAT-034 | Mouse Scroll Support | Complete |
| FEAT-035 | Configurable Tab/Pane Switching | Complete |
| FEAT-037 | tmux-Compatible Keybinds | Complete |
| FEAT-038 | Split Pane Rendering | Complete |
| FEAT-039 | MCP Pane Creation Broadcast | Complete |
| FEAT-040 | MCP Pane Reliability Improvements | Complete |
| FEAT-041 | MCP Session/Window Targeting | Complete |
| FEAT-042 | MCP Debug Logging | Complete |
| FEAT-043 | MCP Session Rename Tool | Complete |
| FEAT-044 | Claude Session Persistence & Auto-Resume | Complete |
| FEAT-045 | MCP Declarative Layout Tools | Complete |
| FEAT-046 | MCP Focus/Select Control | Complete |
| FEAT-047 | ccmux_set_environment MCP tool | Complete |
| FEAT-049 | tmux-compatible CLI wrapper (ccmux-compat) | Complete |
| FEAT-051 | ccmux_get_environment MCP tool | Complete |
| FEAT-052 | ccmux_kill_session MCP tool | Complete |
| FEAT-053 | Auto-inject CCMUX Context Environment Variables | Complete |
| FEAT-054 | Pane-bounded text selection in copy mode | Complete |
| FEAT-055 | Full tmux keybinding parity in TUI | Complete |
| FEAT-056 | User Priority Lockout for MCP Focus Control | Complete |
| FEAT-028 | Orchestration Flexibility Refactor (Tag-based Routing) | Complete |
| FEAT-036 | Session-Aware MCP Commands | Complete |
| FEAT-050 | Session Metadata Storage | Complete |

## Test Coverage

| Wave | Features | Tests |
|------|----------|-------|
| Wave 0 | 6 | 368 |
| Wave 1 | 9 | 452 |
| Wave 2 | 3 | 224 |
| Wave 3 | 2 | 49 |
| Wave 4 | 7 | 126 |
| Post-MVP | 28 | 307 |
| **Total** | **55** | **1,526** |

## Architecture

See [WAVES.md](/WAVES.md) for the complete parallel development plan with dependency analysis.

The project follows a multi-crate workspace structure:
- `ccmux-client/` - TUI client (ratatui + crossterm)
- `ccmux-server/` - Daemon with PTY management + MCP bridge
- `ccmux-protocol/` - Message types and codec (bincode)
- `ccmux-session/` - Session/window/pane hierarchy
- `ccmux-utils/` - Shared utilities
- `ccmux-persistence/` - WAL-based crash recovery
