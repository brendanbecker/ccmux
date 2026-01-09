# BUG-005: Sideband Parsing Not Integrated into PTY Output Flow

**Priority**: P0 (Critical)
**Component**: ccmux-server
**Status**: new
**Created**: 2026-01-09
**Discovered During**: Manual Testing (orchestrator pane splitting)

## Summary

Sideband commands (`<ccmux:spawn>`, `<ccmux:input>`, etc.) output by Claude are displayed as literal text instead of being parsed and executed. The sideband parsing infrastructure exists but is not wired into the PTY output flow.

## Reproduction Steps

1. Start ccmux server and client
2. Create a session with Claude running
3. Have Claude output: `<ccmux:spawn direction="horizontal" command="echo hello" />`
4. Observe the tag appears as literal text in the terminal
5. No new pane is created

## Expected Behavior

- Sideband tags should be parsed from PTY output
- Commands should be executed (spawn creates new pane, etc.)
- Tags should be stripped from output before displaying to user
- New panes should have output pollers started

## Actual Behavior

- Tags are broadcast as raw text to clients
- No parsing occurs
- No commands are executed
- Tags visible in terminal output

## Root Cause Analysis

FEAT-030 was marked as merged but is incomplete. The implementation has two layers:

### Implemented (FEAT-019 + FEAT-030)

1. `SidebandParser` (`ccmux-server/src/sideband/parser.rs`) - parses `<ccmux:*>` XML tags
2. `CommandExecutor` (`ccmux-server/src/sideband/executor.rs`) - executes parsed commands
3. `SessionManager.split_pane()` - creates new panes in same window
4. `SpawnResult` - returns PTY reader for output poller integration

### Missing

The integration layer that connects these components:

1. **`pty/output.rs`** - `PtyOutputPoller::flush()` broadcasts raw data:
   ```rust
   async fn flush(&mut self) {
       // ...
       let msg = ServerMessage::Output {
           pane_id: self.pane_id,
           data,  // Raw output, no sideband parsing!
       };
       self.registry.broadcast_to_session(self.session_id, msg).await;
   }
   ```

2. **`main.rs`** - No `CommandExecutor` instantiated in server runtime

3. **No code path** runs output through `SidebandParser` before broadcasting

### Evidence

Grep shows `SidebandParser` and `CommandExecutor` are only used in tests:
- `sideband/parser.rs` - definition and tests
- `sideband/executor.rs` - definition and tests
- `sideband/mod.rs` - integration tests only

The intended flow is documented in `sideband/mod.rs:37-39`:
```
PTY Output → SidebandParser → (display_text, commands)
                   ├─→ Commands → CommandExecutor → SessionManager
```

But this flow was never implemented in production code.

## Impact

- **Multi-agent orchestration broken**: Claude cannot spawn worker panes
- **Core feature non-functional**: Sideband protocol is the primary Claude-ccmux communication channel
- **FEAT-030 incomplete**: Feature marked as merged but missing critical integration

## Proposed Fix

Modify `PtyOutputPoller` to:

1. Maintain per-pane `SidebandParser` instance
2. Pass output through parser before broadcasting
3. Execute any extracted commands via `CommandExecutor`
4. Broadcast only the filtered display text (tags stripped)
5. Start output pollers for newly spawned panes

### Key Changes Required

| File | Change |
|------|--------|
| `pty/output.rs` | Add `SidebandParser` to `PtyOutputPoller`, filter output in `handle_output()` |
| `main.rs` | Create shared `CommandExecutor`, pass to output pollers |
| `pty/output.rs` | Add channel/callback for spawn results to start new pollers |

### Alternative: Centralized Parsing

Instead of per-pane parsers, could have a central sideband processing task:
- Output pollers send raw output to central processor
- Processor parses, executes commands, broadcasts filtered output
- Simpler lifetime management but adds latency

## Testing

- Unit tests for parser/executor already exist and pass
- Need integration test: output with sideband tag → pane created
- Manual test: Claude outputs spawn command → new pane appears

## Related

- **FEAT-019**: Sideband protocol parsing (complete)
- **FEAT-030**: Sideband pane splitting (executor complete, integration missing)
- **FEAT-029**: MCP natural language control (alternative control path, works)

## Files to Modify

- `ccmux-server/src/pty/output.rs`
- `ccmux-server/src/pty/mod.rs`
- `ccmux-server/src/main.rs`
- `ccmux-server/src/handlers/session.rs` (for new session pane pollers)
