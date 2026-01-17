# BUG-049: send_input with submit: true doesn't reliably submit input

**Priority**: P2
**Component**: mcp
**Severity**: medium
**Status**: new

## Problem Statement

When using ccmux_send_input with submit: true, the text appears in the target pane's input area but doesn't always get submitted. A workaround is to send a separate empty input with submit: true to trigger the Enter key.

## Evidence

User-reported issue during MCP automation with Gemini CLI.

## Steps to Reproduce

1. Call ccmux_send_input with a message and submit: true
2. Observe text appears in target pane's input box
3. Note that Enter key is not reliably triggered
4. Workaround: call ccmux_send_input again with empty input and submit: true

## Expected Behavior

When submit: true is set, the input should be reliably submitted (Enter pressed) after the text is sent.

## Actual Behavior

Text appears in the target pane's input box but isn't submitted. Requires a second call with empty input and submit: true to actually submit.

## Root Cause

Possibly a race condition between writing text and sending Enter key, or the Enter key being sent before text write completes. Investigation needed in:

1. `ccmux-server/src/handlers/mcp.rs` - the send_input handler
2. `ccmux-mcp-bridge/src/handlers.rs` - bridge-side handling
3. PTY write logic - how text and Enter key are sequenced

## Implementation Tasks

### Section 1: Investigation
- [ ] Trace the send_input flow from MCP handler to PTY write
- [ ] Identify where submit: true triggers Enter key
- [ ] Check if text write and Enter are sent atomically or sequentially
- [ ] Add debug logging to observe timing of write operations

### Section 2: Fix Implementation
- [ ] Ensure text write completes before Enter key is sent
- [ ] Consider using a single PTY write with text + newline combined
- [ ] Add flush/sync if needed to ensure ordering

### Section 3: Testing
- [ ] Add test case for send_input with submit: true
- [ ] Test with various input lengths
- [ ] Test rapid successive calls
- [ ] Verify fix works with Gemini CLI and other targets

### Section 4: Verification
- [ ] Confirm submit: true reliably submits input
- [ ] Verify no regression in send_input without submit
- [ ] Verify no side effects in related functionality

## Acceptance Criteria

- [ ] send_input with submit: true reliably submits input in a single call
- [ ] No need for workaround of sending empty input separately
- [ ] Tests added to prevent regression
- [ ] Root cause documented

## Notes

- Has workaround (send empty input with submit: true after main input)
- Medium severity due to workaround availability
- Likely a timing/ordering issue in PTY write operations
