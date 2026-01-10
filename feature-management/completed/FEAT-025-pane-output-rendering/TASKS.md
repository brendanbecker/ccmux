# Task Breakdown: FEAT-025

**Work Item**: [FEAT-025: Pane Output Rendering](PROMPT.md)
**Status**: Not Started
**Last Updated**: 2026-01-09

## Prerequisites

- [ ] Read and understand PROMPT.md
- [ ] Review PLAN.md and update if needed
- [ ] Verify dependencies: FEAT-023 (PTY Output Polling), FEAT-022 (Message Routing)
- [ ] Review existing code in `ui/pane.rs` and `ui/app.rs`

## Design Tasks

- [ ] Understand `PaneInfo` (protocol) vs `Pane` (UI) relationship
- [ ] Decide integration approach (PaneManager in App)
- [ ] Review `render_pane()` function signature
- [ ] Plan pane lifecycle synchronization

## Implementation Tasks

### Add PaneManager to App
- [ ] Import `PaneManager` from `ui/pane.rs`
- [ ] Add `pane_manager: PaneManager` field to `App` struct
- [ ] Initialize `PaneManager::new()` in `App::new()`
- [ ] Ensure imports are correct

### Wire Pane Lifecycle Events
- [ ] Handle `ServerMessage::PaneCreated`:
  - [ ] Create UI `Pane` with correct dimensions
  - [ ] Add to `pane_manager`
  - [ ] Sync title and state from `PaneInfo`
- [ ] Handle `ServerMessage::PaneClosed`:
  - [ ] Remove from `pane_manager`
- [ ] Handle `ServerMessage::Attached`:
  - [ ] Create UI panes for all panes in list
  - [ ] Set initial dimensions based on terminal size
- [ ] Handle `ServerMessage::PaneStateChanged`:
  - [ ] Update `pane_manager` pane state

### Wire Output to Pane Terminal
- [ ] In `handle_server_message()` for `ServerMessage::Output`:
  - [ ] Get mutable reference to UI `Pane`
  - [ ] Call `pane.process_output(&data)`
  - [ ] Log warning if pane not found
- [ ] Remove TODO comment at line 565
- [ ] Remove placeholder `let _ = data;`

### Update Rendering
- [ ] Import `render_pane` from `ui/pane`
- [ ] In `draw()` method:
  - [ ] Calculate pane rectangles from layout
  - [ ] For each pane, call `render_pane()`
  - [ ] Pass tick count for animations
- [ ] Ensure focus state is synced for border highlighting

### Handle Resize
- [ ] On terminal size change:
  - [ ] Recalculate layout
  - [ ] Resize all UI panes via `pane_manager.resize_pane()`
  - [ ] Send resize to server if needed
- [ ] Handle ViewportUpdated message (line 606)

## Testing Tasks

- [ ] Manual test: shell prompt appears on attach
- [ ] Manual test: `ls -la` output displays correctly
- [ ] Manual test: colors work (try `ls --color`)
- [ ] Manual test: create second pane, both render independently
- [ ] Manual test: scroll up/down in pane
- [ ] Manual test: resize terminal, panes adjust
- [ ] Performance test: `cat` large file, no lag

## Documentation Tasks

- [ ] Update code comments in `app.rs`
- [ ] Document pane lifecycle in PLAN.md
- [ ] Add usage notes if needed

## Verification Tasks

- [ ] All acceptance criteria from PROMPT.md met
- [ ] No TODO stubs remain for output handling
- [ ] Tests passing (if any added)
- [ ] Update feature_request.json status to in_progress/completed
- [ ] Document completion in PLAN.md

## Completion Checklist

- [ ] All implementation tasks complete
- [ ] Manual testing passed
- [ ] PLAN.md updated with final approach
- [ ] Ready for review/merge

---
*Check off tasks as you complete them. Update status field above.*
