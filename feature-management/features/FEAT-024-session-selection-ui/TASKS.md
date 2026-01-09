# Task Breakdown: FEAT-024

**Work Item**: [FEAT-024: Session Selection UI](PROMPT.md)
**Status**: Not Started
**Last Updated**: 2026-01-09

## Prerequisites

- [ ] Read and understand PROMPT.md
- [ ] Review PLAN.md and update if needed
- [ ] Review current app.rs implementation (lines 478-517, 667-711)
- [ ] Verify FEAT-021 (Server Socket) is available

## Review Tasks

- [ ] Review `handle_session_select_input()` implementation
- [ ] Review `draw_session_select()` implementation
- [ ] Compare implementation against requirements
- [ ] Document any gaps found

## Implementation Tasks

### UI Rendering
- [ ] Verify session list renders correctly
- [ ] Verify selection highlight is visible
- [ ] Verify empty state displays helpful message
- [ ] Consider if List widget would improve display

### Navigation
- [ ] Test up arrow navigation
- [ ] Test down arrow navigation
- [ ] Test 'k' for up navigation
- [ ] Test 'j' for down navigation
- [ ] Test bounds checking (can't go below 0 or above list length)

### Session Actions
- [ ] Test Enter key attaches to selected session
- [ ] Test 'n' key creates new session
- [ ] Test 'r' key refreshes session list
- [ ] Verify auto-attach after session creation

### Metadata Display
- [ ] Verify session name displays
- [ ] Verify window count displays
- [ ] Verify attached client count displays

## Testing Tasks

- [ ] Manual test: Empty session list state
- [ ] Manual test: Single session in list
- [ ] Manual test: Multiple sessions in list
- [ ] Manual test: Navigate to first/last session
- [ ] Manual test: Attach to existing session
- [ ] Manual test: Create and attach to new session
- [ ] Manual test: Refresh session list

## Documentation Tasks

- [ ] Verify help text shows correct keybindings
- [ ] Update CHANGELOG if needed

## Verification Tasks

- [ ] All acceptance criteria from PROMPT.md met
- [ ] No regressions in existing functionality
- [ ] Update feature_request.json status
- [ ] Document completion in comments.md

## Completion Checklist

- [ ] All implementation tasks complete
- [ ] All tests passing
- [ ] PLAN.md updated with final notes
- [ ] Ready for review/merge

---
*Check off tasks as you complete them. Update status field above.*
