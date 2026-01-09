# BUG-003: Session Creation Doesn't Create Default Window/Pane

**Type**: Bug
**Priority**: P0
**Status**: open
**Created**: 2026-01-09
**Found During**: HA-001 manual testing

## Description

When a new session is created via `CreateSession`, the session is empty with 0 windows and 0 panes. The client attaches to an empty session and shows "No active pane" with no input handling.

## Reproduction Steps

1. Start server: `./target/release/ccmux-server`
2. Start client: `./target/release/ccmux`
3. Press 'n' to create new session
4. Observe: Client shows "No active pane", keyboard input ignored

## Server Logs

```
Client attached to session 0b53814a-c918-4b05-9a8e-c5950c5cab51 (0 windows, 0 panes)
```

## Expected Behavior

When a session is created, it should automatically contain:
- 1 default window
- 1 default pane with a spawned PTY (shell)

## Root Cause

`handle_create_session()` in `ccmux-server/src/handlers/session.rs` calls `session_manager.create_session()` which creates an empty session. No window or pane is auto-created.

## Fix Location

`ccmux-server/src/handlers/session.rs:28-53` - `handle_create_session()`

After creating the session, should:
1. Create a default window in the session
2. Create a default pane in that window
3. Spawn a PTY for the pane
4. Return session info with the window/pane included

## Alternative Considered

Client-side fix (send CreateWindow+CreatePane after Attached with empty panes) - rejected as server-side is cleaner and ensures sessions are never empty.
