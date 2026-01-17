# BUG-048: TUI flickers on every keystroke when Claude Code is detected

**Priority**: P1
**Component**: tui
**Severity**: high
**Status**: new

## Problem Statement

Once Claude Code is detected in a pane, every keystroke causes the entire screen to flash/flicker. This is caused by the agent detector returning state on every call instead of only when state changes.

## Evidence

**File**: `ccmux-server/src/agents/claude/mod.rs` (lines 127-137)

```rust
fn analyze(&mut self, text: &str) -> Option<AgentState> {
    let _change = self.inner.analyze(text);  // <-- Return value IGNORED!

    if self.inner.is_claude() {
        self.state()  // Returns Some ALWAYS if Claude detected
    } else {
        None
    }
}
```

The inner `ClaudeDetector.analyze()` returns `Option<ClaudeStateChange>` - only returning `Some` when state **changes**. But this wrapper ignores the return value and returns `Some(state)` on EVERY call once Claude is detected.

## Chain of Events Causing Flicker

1. User types a key
2. Key is echoed back as PTY output
3. `pane.process(&data)` calls `agent_detector.analyze(&text)`
4. `analyze()` returns `Some(state)` because Claude is detected (BUG - should only return on change)
5. Server broadcasts `PaneStateChanged` to TUI
6. TUI receives `PaneStateChanged` and sets `needs_redraw = true` (added in FEAT-082)
7. Main loop sees `needs_redraw = true` and calls `terminal.clear()`
8. Screen flickers

## Steps to Reproduce

1. Start ccmux and attach to a session
2. Launch Claude Code in a pane (`claude-code` or `cc`)
3. Wait for Claude Code to be detected
4. Type any character
5. Observe screen flicker on every keystroke

## Expected Behavior

TUI should only redraw when agent state actually changes, not on every keystroke.

## Actual Behavior

TUI redraws and flickers on every keystroke once Claude Code is detected.

## Root Cause

`ClaudeAgentDetector.analyze()` ignores the return value of `inner.analyze()` which only returns `Some` when state changes. Instead, it always returns `Some(state)` whenever Claude is detected, causing excessive `PaneStateChanged` broadcasts.

## Implementation Tasks

### Section 1: Fix Implementation
- [ ] Modify `analyze()` in `ccmux-server/src/agents/claude/mod.rs` to only return `Some(state)` when inner detector reports a change
- [ ] Preserve the change-detection semantics of the inner `ClaudeDetector`

### Section 2: Testing
- [ ] Add unit test to verify `analyze()` returns `None` when called repeatedly without state change
- [ ] Add unit test to verify `analyze()` returns `Some` only on actual state transitions
- [ ] Manual test: verify no flicker when typing in Claude Code pane

### Section 3: Verification
- [ ] Confirm TUI no longer flickers on keystrokes
- [ ] Verify agent state still updates correctly when real changes occur
- [ ] Verify no regressions in agent detection functionality

## Acceptance Criteria

- [ ] `analyze()` returns `None` when no state change occurs
- [ ] `analyze()` returns `Some(state)` only when state transitions
- [ ] TUI does not flicker on normal keystrokes
- [ ] Agent detection still works correctly
- [ ] No regressions in existing functionality

## The Fix

```rust
fn analyze(&mut self, text: &str) -> Option<AgentState> {
    // Only return state if inner detector reports a change
    if self.inner.analyze(text).is_some() {
        self.state()
    } else {
        None
    }
}
```

## Related Work Items

- **FEAT-084**: Abstract agent state (introduced the wrapper)
- **FEAT-082**: Adaptive layout engine (added `needs_redraw` to `PaneStateChanged` handler)

## Notes

This is a straightforward fix - the original code was already designed to detect changes, but the wrapper was not properly propagating that information. The fix restores the intended change-detection semantics.
