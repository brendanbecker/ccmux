# FEAT-049: Add tmux-compatible CLI wrapper (ccmux-compat)

**Priority**: P2
**Component**: ccmux-compat
**Type**: new_feature
**Estimated Effort**: medium
**Business Value**: high

## Overview

Create a CLI binary that accepts tmux command syntax and translates to ccmux MCP calls, enabling drop-in replacement for existing tools.

### Context

- Gas Town shells out to tmux via `exec.Command("tmux", args...)` in internal/tmux/tmux.go:35
- A compatibility layer allows one-line swap for incremental migration
- No changes needed in Gas Town codebase for Phase 1

## Benefits

- **Incremental Adoption**: Existing tools can switch from tmux to ccmux with a single-line change (rename binary or symlink)
- **Zero Code Changes**: Consumer applications like Gas Town require no modifications
- **Familiar Interface**: Users comfortable with tmux syntax can use ccmux immediately
- **Migration Path**: Gradual transition from tmux to native ccmux MCP calls

## Implementation

### 1. Create new crate: ccmux-compat/

Add a new binary crate to the workspace that provides the tmux-compatible CLI.

### 2. Command Translation Table

| tmux Command | ccmux MCP Translation |
|--------------|----------------------|
| `new-session -d -s NAME [-c DIR] [CMD]` | ccmux_create_session + ccmux_send_input |
| `send-keys -t TARGET [-l] TEXT` | ccmux_send_input |
| `kill-session -t NAME` | ccmux_kill_session |
| `has-session -t =NAME` | ccmux_list_sessions + filter (exit 0/1) |
| `list-sessions [-F FORMAT]` | ccmux_list_sessions + format |
| `capture-pane -p -t TARGET [-S -N]` | ccmux_read_pane |
| `set-environment -t SESS KEY VAL` | ccmux_set_environment |
| `show-environment -t SESS KEY` | ccmux_get_environment |

### 3. Connection to ccmux daemon

Connect via Unix socket (same as ccmux-client) and issue MCP commands through the protocol layer.

### 4. Output Compatibility

Output must match tmux format for seamless compatibility:
- Exit codes match tmux behavior (0 for success, 1 for "not found", etc.)
- list-sessions format strings work identically
- capture-pane output is raw terminal content

## Implementation Tasks

### Section 1: Crate Setup
- [ ] Create ccmux-compat/ crate directory
- [ ] Add to workspace Cargo.toml
- [ ] Set up basic CLI argument parsing (clap)
- [ ] Add dependencies: ccmux-protocol, ccmux-utils

### Section 2: Command Parsing
- [ ] Parse tmux-style subcommands (new-session, send-keys, etc.)
- [ ] Handle common flags: -d, -s, -t, -c, -l, -p, -F, -S, -N
- [ ] Validate target session/window/pane syntax

### Section 3: MCP Translation Layer
- [ ] Implement session creation translation
- [ ] Implement send-keys translation
- [ ] Implement kill-session translation
- [ ] Implement has-session translation
- [ ] Implement list-sessions translation
- [ ] Implement capture-pane translation
- [ ] Implement environment variable commands

### Section 4: Connection Layer
- [ ] Connect to ccmux daemon via Unix socket
- [ ] Reuse existing protocol/codec from ccmux-protocol
- [ ] Handle connection errors gracefully

### Section 5: Output Formatting
- [ ] Match tmux exit codes
- [ ] Implement list-sessions format strings (-F)
- [ ] Output capture-pane as raw content

### Section 6: Testing
- [ ] Unit tests for command parsing
- [ ] Integration tests comparing tmux vs ccmux-compat output
- [ ] Test with Gas Town tmux wrapper patterns

### Section 7: Documentation
- [ ] Document supported commands and flags
- [ ] Migration guide for existing tmux users
- [ ] Add to workspace README

## Acceptance Criteria

- [ ] `ccmux-compat new-session -d -s test` creates a detached session
- [ ] `ccmux-compat send-keys -t test "echo hello" Enter` sends input
- [ ] `ccmux-compat kill-session -t test` terminates the session
- [ ] `ccmux-compat has-session -t =test` exits 0 if exists, 1 if not
- [ ] `ccmux-compat list-sessions` outputs session list in tmux format
- [ ] `ccmux-compat capture-pane -p -t test` outputs pane content
- [ ] Exit codes match tmux behavior
- [ ] Can be used as drop-in replacement in Gas Town

## Dependencies

None - this is a new standalone crate that depends only on existing ccmux crates.

## Notes

### Phase 1 Scope

Focus on the subset of tmux commands used by Gas Town:
- Session creation/destruction
- Sending input
- Session existence checks
- Pane content capture

### Future Enhancements

- Window management commands
- Pane splitting commands
- More format string options
- Configuration file compatibility

### Related Work

- FEAT-033 (tmux-like auto-start) provides similar UX patterns
- MCP tools in ccmux-server handle the actual session operations
