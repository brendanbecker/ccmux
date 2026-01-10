# Bug Reports

**Project**: ccmux
**Last Updated**: 2026-01-10

## Summary Statistics
- Total Bugs: 16
- New: 10
- In Progress: 0
- Resolved: 5
- Deprecated: 1

## Bugs by Priority

### P0 - Critical (0)

*No open P0 bugs*

### P1 - High Priority (7)

#### BUG-020: Session reattach from session manager creates client without PTY [NEW]

**Status**: New
**Filed**: 2026-01-10
**Component**: ccmux-server
**Directory**: [BUG-020-session-reattach-no-pty](BUG-020-session-reattach-no-pty/)

**Description**:
When a user selects an existing session from the session manager/session selection UI, it creates an additional client connection to the session but the client doesn't get a PTY. The user cannot interact with the terminal - they see the pane but can't type or see output.

**Symptoms**:
- Selecting an existing session from session manager UI
- Client appears to connect (additional client registered)
- No PTY is assigned/visible to the new client
- Cannot interact with the terminal pane

**Suspected Root Cause**:
May be related to how session attachment handles PTY reader cloning or output poller registration. The attach handler may not be properly connecting the new client to the existing PTY output stream.

**Impact**:
Session reattachment via session manager is broken. Users cannot reconnect to existing sessions through the UI.

#### BUG-016: PTY output not routed to pane state - breaks Claude detection and MCP read_pane [NEW]

**Status**: New
**Filed**: 2026-01-10
**Component**: ccmux-server
**Directory**: [BUG-016-pty-output-not-routed-to-pane-state](BUG-016-pty-output-not-routed-to-pane-state/)

**Description**:
The PtyOutputPoller broadcasts PTY output to connected TUI clients via ServerMessage::Output, but never routes the output back to the pane's scrollback buffer or through pane.process() for Claude detection. This causes two critical failures:
1. MCP read_pane returns empty because the scrollback buffer is never populated
2. Claude detection never triggers because pane.process() is never called

**Symptoms**:
- All panes show is_claude: false regardless of what's running
- MCP ccmux_read_pane returns empty strings
- Claude detection is completely non-functional

**Root Cause**:
PtyOutputPoller::flush() only broadcasts to clients via registry.broadcast_to_session(). There is no call to route data to pane.process() which handles scrollback and Claude detection.

**Impact**:
Core MCP functionality broken. read_pane always returns empty. Claude detection non-functional, breaking orchestration features.

#### BUG-018: TUI pane interaction failure - can't see input bar or interact with pane [NEW]

**Status**: Needs Investigation
**Filed**: 2026-01-10
**Component**: ccmux-client
**Directory**: [BUG-018-tui-pane-interaction-failure](BUG-018-tui-pane-interaction-failure/)

**Description**:
In the ccmux TUI, the user cannot see the text input bar and cannot interact with a pane that visually shows Claude Code output. The pane shows Claude conversation with purple/magenta theme but the Claude Code input prompt ("> ") is not visible at the bottom and keyboard input doesn't reach the pane.

**Symptoms**:
- Visual: Pane shows Claude conversation output
- Missing input: Claude Code input prompt not visible
- No interaction: Keyboard input doesn't reach pane
- MCP diagnostics show `is_claude: false`, empty `read_pane`

**Suspected Root Cause**:
Multiple potential causes: focus issue (pane not focused), scroll position (view scrolled up, input bar below visible area), Claude process state (exited/hung), layout/resize bug (dimensions wrong), or related to BUG-015 (layout not recalculated).

**Impact**:
Blocks user interaction with Claude sessions in TUI.

#### BUG-017: MCP send_input doesn't handle Enter key [NEW]

**Status**: New
**Filed**: 2026-01-10
**Component**: ccmux-server/mcp
**Directory**: [BUG-017-mcp-send-input-no-enter-key](BUG-017-mcp-send-input-no-enter-key/)

**Description**:
The MCP `ccmux_send_input` tool doesn't properly handle sending Enter key to submit commands. When sending text via MCP, the Enter key is not appended/translated, so commands are typed but not executed.

**Symptoms**:
- Text sent via MCP appears in pane
- Command is not executed (no Enter sent)
- User must manually press Enter or find workaround

**Suspected Root Cause**:
The `send_input` tool takes text but doesn't interpret `\n` as Enter key sequence or provide a way to send control characters.

**Impact**:
MCP automation cannot execute commands, only type them. Blocks full MCP orchestration capability.

#### BUG-014: Large output causes viewport/buffer overflow, making input unresponsive [NEW]

**Status**: New
**Filed**: 2026-01-10
**Component**: terminal-buffer
**Directory**: [BUG-014-large-output-buffer-overflow](BUG-014-large-output-buffer-overflow/)

**Description**:
When a Claude session generates large output (such as an extensive README diff), the viewport/scrollback/buffer cannot hold the full output. Input becomes completely unresponsive - the user cannot interact with the session. Detach (Ctrl+B d) works, but reattaching shows the same unresponsive state, indicating a persistent buffer capacity issue.

**Symptoms**:
- Input completely unresponsive after large output
- Detach works but reattach shows same issue
- Session itself is running (not crashed)

**Suspected Root Cause**:
Multiple potential causes: unbounded scrollback buffer, output flooding faster than client can consume, missing backpressure mechanism, event loop starvation, or server-side state bloat.

**Impact**:
Blocks user interaction entirely when working with Claude on tasks that generate large outputs. User must abandon the session.

#### BUG-010: MCP pane creation broadcast not received by TUI [NEW]

**Status**: New
**Filed**: 2026-01-09
**Component**: ccmux-server / ccmux-client
**Directory**: [BUG-010-mcp-pane-broadcast-not-received](BUG-010-mcp-pane-broadcast-not-received/)

**Description**:
When panes are created via MCP tools (e.g., `ccmux_create_pane`), the TUI client does not receive the `PaneCreated` broadcast. The pane exists on the server but the TUI is unaware of it - no split is rendered, and `Ctrl+B o` cannot switch to the new pane.

**Symptoms**:
- MCP `ccmux_create_pane` returns success
- Server shows 2 panes (via `ccmux_list_panes`)
- TUI shows only 1 pane (no split)
- New pane has default 80x24 dimensions (not resized)

**Suspected Root Cause**:
FEAT-039 implemented `ResponseWithBroadcast` but the broadcast is not reaching TUI clients. Possible issues: session ID mismatch, client not registered in session_clients, or channel delivery failure.

**Impact**:
MCP-based pane splitting is broken. Claude cannot effectively split panes via MCP tools.

#### BUG-004: Client hangs when reattaching to session with dead pane [RESOLVED]

**Status**: Resolved
**Filed**: 2026-01-09
**Resolved**: 2026-01-09
**Component**: ccmux-server

**Description**:
Client becomes unresponsive when attaching to a session whose pane's shell has exited. The session/pane remained in server state as zombies after the PTY output poller exited.

**Root Cause**:
PTY output poller only broadcast `PaneClosed` to clients but did not clean up pane/session from server state, leaving zombie sessions that could be attached to but had no active output poller.

**Resolution**:
Added automatic cleanup when PTY processes exit:
- New `PaneClosedNotification` channel from output pollers
- `run_pane_cleanup_loop()` task removes dead panes, empty windows, empty sessions
- All new panes get cleanup channel via `HandlerContext`

#### BUG-005: Sideband parsing not integrated into PTY output flow [RESOLVED]

**Status**: Resolved
**Filed**: 2026-01-09
**Resolved**: 2026-01-09
**Component**: ccmux-server
**Directory**: [BUG-005-sideband-parsing-not-integrated](BUG-005-sideband-parsing-not-integrated/)

**Description**:
Sideband commands (`<ccmux:spawn>`, `<ccmux:focus>`, etc.) output by Claude are displayed as literal text instead of being parsed and executed. The sideband parsing infrastructure (FEAT-019, FEAT-030) exists but is not wired into the PTY output flow.

**Root Cause**:
- FEAT-019 implemented `SidebandParser` and command types
- FEAT-030 implemented `CommandExecutor` with spawn functionality
- The integration point - wiring parser/executor into `PtyOutputPoller::flush()` - was never completed
- `SidebandParser` and `CommandExecutor` are only instantiated in test code

**Resolution**:
Integrated `SidebandParser` into `PtyOutputPoller` to filter output and execute commands before broadcasting to clients.

#### BUG-006: Viewport not sizing to terminal dimensions [RESOLVED]

**Status**: Resolved
**Filed**: 2026-01-09
**Resolved**: 2026-01-09
**Component**: ccmux-client

**Description**:
The ccmux viewport does not size itself to match the actual terminal dimensions. When ccmux is started in a full-screen terminal, the viewport renders at approximately quarter-screen size instead of filling the available space.

**Root Cause**:
Chicken-and-egg problem: Server creates panes at 80x24 default, client used server's dimensions instead of its own terminal size when creating UI panes, and no resize was sent on attach.

**Resolution**:
Modified `ccmux-client/src/ui/app.rs` to use client's terminal size when creating UI panes on attach, and send resize messages to server for all panes immediately after attach.

#### BUG-007: Shift+Tab not passed through to PTY [RESOLVED]

**Status**: Resolved
**Filed**: 2026-01-09
**Resolved**: 2026-01-09
**Component**: ccmux-client

**Description**:
Shift+Tab keystrokes are silently dropped instead of being sent to the PTY. Programs like Claude Code that use Shift+Tab don't receive the keystroke.

**Root Cause**:
`ccmux-client/src/input/keys.rs` has no match arm for `KeyCode::BackTab`. Crossterm sends `KeyCode::BackTab` for Shift+Tab (not `KeyCode::Tab` with SHIFT modifier), so it falls through to `_ => None` and is dropped.

**Resolution**:
Added `KeyCode::BackTab => Some(b"\x1b[Z".to_vec())` to `keys.rs`.

### P2 - Medium Priority (5)

#### BUG-015: Layout doesn't recalculate when panes are closed - remaining pane stays at partial size [NEW]

**Status**: New
**Filed**: 2026-01-10
**Component**: ccmux-client
**Directory**: [BUG-015-layout-not-recalculated-on-pane-close](BUG-015-layout-not-recalculated-on-pane-close/)

**Description**:
When multiple panes exist (e.g., quadrant layout with 4 panes) and some panes are closed, the remaining pane(s) do not expand to fill the available window space. Instead, the remaining pane stays at its previous size leaving empty/unused space.

**Symptoms**:
- Remaining pane stays at partial size after other panes are closed
- Empty/dead space visible in the window
- Layout tree not recalculated when nodes are removed

**Suspected Root Cause**:
The TUI layout system does not trigger a recalculation when panes are closed, or the layout tree is not being pruned/simplified when nodes are removed.

**Impact**:
Wastes screen space and requires user to restart ccmux to restore full-window pane. Workaround exists (restart) but disrupts workflow.

#### BUG-013: Mouse scroll wheel not working for scrollback [NEW]

**Status**: New
**Filed**: 2026-01-10
**Component**: ccmux-client
**Directory**: [BUG-013-mouse-scroll-wheel-not-working](BUG-013-mouse-scroll-wheel-not-working/)

**Description**:
Mouse scroll wheel does not scroll through terminal scrollback history. FEAT-034 (Mouse Scroll Support) was supposedly implemented and merged, but scrolling with the mouse wheel is not functioning.

**Symptoms**:
- Scroll wheel does nothing in the TUI
- Cannot scroll back through terminal output using mouse
- FEAT-034 claims this feature works but it doesn't

**Suspected Root Cause**:
Multiple potential causes to investigate:
1. Mouse scroll events not being captured after recent changes
2. FEAT-034 implementation may have a bug or regression
3. Mouse capture mode may be interfering
4. Scroll events captured but not translated to viewport scroll
5. Configuration issue or mouse mode not enabled

**Impact**:
UX issue - users cannot scroll through terminal output with mouse. Can still use copy mode or keyboard navigation as workaround.

#### BUG-011: Large paste input crashes ccmux session [NEW]

**Status**: New
**Filed**: 2026-01-10
**Component**: ccmux-client / ccmux-server
**Directory**: [BUG-011-large-paste-crashes-session](BUG-011-large-paste-crashes-session/)

**Description**:
Pasting an extremely large amount of text into a ccmux terminal session causes the session to crash. There is no graceful handling or error message - the session simply dies.

**Symptoms**:
- Session crash on large paste
- No graceful handling or error message
- Requires session reattachment after crash

**Suspected Root Cause**:
Multiple potential causes to investigate:
1. Buffer overflow in input handling path
2. Message size limit exceeded on Unix socket protocol
3. PTY write buffer overwhelmed (no chunking)
4. Bincode serialization failing on huge payloads
5. Memory exhaustion from allocating large input buffer

**Impact**:
Bad user experience when accidentally pasting large content. Session loss requires reattachment and may lose unsaved work.

#### BUG-009: Flaky persistence/recovery tests due to test isolation issues [NEW]

**Status**: New
**Filed**: 2026-01-09
**Component**: ccmux-server
**Directory**: [BUG-009-flaky-persistence-tests](BUG-009-flaky-persistence-tests/)

**Description**:
The persistence/recovery tests have intermittent race conditions. A different test fails on each run - it's not one specific test but rather test isolation issues affecting the entire persistence test suite. Tests pass when run individually but fail ~30% of parallel runs.

**Affected Tests**:
- `persistence::recovery::tests::test_recovery_from_wal`
- `persistence::recovery::tests::test_recovery_active_window_pane`
- `persistence::recovery::tests::test_recovery_pane_updates`
- `persistence::tests::test_persistence_log_operations`

**Suspected Root Cause**:
Same pattern as BUG-002 - tests likely share temp directories or file handles. Requires deep investigation of test isolation patterns.

**Impact**:
CI/test noise makes it difficult to verify if new features are working correctly. Has been plaguing the project for multiple sessions.

#### BUG-002: Flaky test `test_ensure_dir_nested` due to shared temp directory [RESOLVED]

**Status**: Resolved
**Filed**: 2026-01-09
**Resolved**: 2026-01-09
**Component**: ccmux-utils
**File**: `ccmux-utils/src/paths.rs:413`

**Description**:
The test `test_ensure_dir_nested` intermittently fails when running the full test suite in parallel, but passes when run in isolation.

**Root Cause**:
Two tests share the same base directory path using `std::process::id()`:
- `test_ensure_dir_creates_directory` uses `ccmux_test_{pid}/`
- `test_ensure_dir_nested` uses `ccmux_test_{pid}/nested/deep`

When tests run in parallel, one test may delete the shared base directory while the other test is attempting to use it, causing a race condition.

**Error Message**:
```
thread 'paths::tests::test_ensure_dir_nested' panicked at ccmux-utils/src/paths.rs:428:9:
assertion failed: result.is_ok()
```

**Steps to Reproduce**:
1. Run `cargo test --workspace`
2. Test may fail intermittently (not always reproducible)
3. Running `cargo test -p ccmux-utils test_ensure_dir_nested` passes consistently

**Resolution**:
Used `tempfile::TempDir` for test isolation in ensure_dir tests.

### P3 - Low Priority (0)

*No P3 bugs*

## Recent Activity

| Date | Bug ID | Action | Description |
|------|--------|--------|-------------|
| 2026-01-10 | BUG-020 | Filed | Session reattach from session manager creates client without PTY |
| 2026-01-10 | BUG-018 | Filed | TUI pane interaction failure - can't see input bar or interact with pane |
| 2026-01-10 | BUG-017 | Filed | MCP send_input doesn't handle Enter key |
| 2026-01-10 | BUG-016 | Filed | PTY output not routed to pane state - breaks Claude detection and MCP read_pane |
| 2026-01-10 | BUG-015 | Filed | Layout doesn't recalculate when panes are closed |
| 2026-01-10 | BUG-014 | Filed | Large output causes buffer overflow, making input unresponsive |
| 2026-01-10 | BUG-013 | Filed | Mouse scroll wheel not working for scrollback |
| 2026-01-10 | BUG-012 | Deprecated | Shift+click works for native selection (by design) |
| 2026-01-10 | BUG-011 | Filed | Large paste input crashes ccmux session |
| 2026-01-09 | BUG-010 | Filed | MCP pane broadcast not received by TUI |
| 2026-01-09 | BUG-009 | Filed | Flaky persistence tests due to test isolation issues |
| 2026-01-09 | BUG-005 | Resolved | Integrated sideband parsing into PTY output flow |
| 2026-01-09 | BUG-007 | Resolved | Added KeyCode::BackTab handler |
| 2026-01-09 | BUG-007 | Filed | Shift+Tab not passed through (missing BackTab case) |
| 2026-01-09 | BUG-006 | Resolved | Client now uses terminal size on attach |
| 2026-01-09 | BUG-006 | Filed | Viewport not sizing to terminal dimensions |
| 2026-01-09 | BUG-005 | Filed | Sideband parsing not integrated into output flow |
| 2026-01-09 | BUG-004 | Filed & Resolved | Zombie panes causing client hang |
| 2026-01-09 | BUG-002 | Filed | Flaky test due to shared temp directory |
