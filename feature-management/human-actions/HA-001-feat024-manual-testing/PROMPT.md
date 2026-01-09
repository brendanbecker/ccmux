# HA-001: Manual Testing for FEAT-024 Session Selection UI

**Type**: Manual Testing
**Related**: FEAT-024
**Priority**: P1
**Status**: blocked
**Blocked By**: BUG-001 (Client input not captured)
**Created**: 2026-01-09

## Description

FEAT-024 (Session Selection UI) implementation is complete but requires manual testing to verify the UI works correctly with a live server connection. Automated unit tests pass, but visual and interaction testing requires human verification.

## Prerequisites

- FEAT-021 (Server Socket Listen Loop) merged ✅
- FEAT-024 (Session Selection UI) merged ✅

## Test Procedure

### Setup

```bash
# Terminal 1: Start server
cargo run --bin ccmux-server

# Terminal 2: Start client
cargo run --bin ccmux
```

### Test Cases

- [ ] **Empty session list state**: On first connect with no sessions, displays "No sessions available. Press 'n' to create one."
- [ ] **Single session in list**: After creating one session, displays it with name, window count, client count
- [ ] **Multiple sessions in list**: Create multiple sessions, verify all display correctly
- [ ] **Navigate to first/last session**: Use up/down arrows and j/k, verify bounds work (can't go past first or last)
- [ ] **Attach to existing session**: Select session, press Enter, verify client attaches
- [ ] **Create and attach to new session**: Press 'n', verify new session created and client auto-attaches
- [ ] **Refresh session list**: Press 'r', verify list refreshes

### Visual Verification

- [ ] Selection highlight (cyan color, "> " prefix) is clearly visible
- [ ] Help text at bottom shows correct keybindings
- [ ] Session metadata (name, windows, clients) displays correctly

## Completion

When all tests pass:
1. Check off items above
2. Update `feature-management/features/FEAT-024-session-selection-ui/TASKS.md` manual test checkboxes
3. Mark this human action as `completed`

## Notes

- If any tests fail, file a bug report in `feature-management/bugs/`
- The session selection UI uses ratatui List widget with stateful selection
