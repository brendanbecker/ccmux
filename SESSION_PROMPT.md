# SESSION_PROMPT: FEAT-003 Viewport Pinning

**Feature**: FEAT-003 - Viewport Pinning with New Content Indicator
**Priority**: P2 | **Effort**: Medium | **Component**: tui

## Objective

When user scrolls up in a pane, new output buffers without yanking the viewport. A visual indicator shows the number of new lines below (e.g., "▼ 47 new lines below") with click or keypress to jump to bottom.

## Key Deliverables

1. **ViewportState** tracking per-pane
2. **Protocol messages** for viewport sync
3. **New content indicator** widget
4. **Jump-to-bottom** navigation (keyboard + click)

## Technical Design

```rust
// Viewport state model
pub struct ViewportState {
    offset_from_bottom: usize,  // 0 = at bottom, following new content
    is_pinned: bool,            // User scrolled up
    new_lines_since_pin: usize, // Lines received while pinned
}

// Protocol messages
// Client -> Server
SetViewportOffset { pane_id: PaneId, offset: usize }

// Server -> Client
ViewportUpdated { pane_id: PaneId, state: ViewportState }
```

## Existing Code Context

- `ccmux-protocol/src/messages.rs` - Existing ClientMessage/ServerMessage enums
- `ccmux-client/src/ui.rs` - UI stub ("UI components - to be implemented")
- `ccmux-server/src/session/pane.rs` - Pane struct with dimensions
- No scroll position tracking exists

## Implementation Sections

### Section 1: Protocol Extension
- [x] Add `ViewportState` struct to ccmux-protocol/src/types.rs
- [x] Add `SetViewportOffset` to ClientMessage enum
- [x] Add `ViewportUpdated` to ServerMessage enum
- [x] Update codec serialization (bincode/serde derives work automatically)

### Section 2: Server-Side State
- [ ] Add viewport state field to Pane struct
- [ ] Track "pinned" status when offset > 0
- [ ] Count new lines received while pinned
- [ ] Handle viewport position updates from client
- [ ] Broadcast ViewportUpdated on state change

### Section 3: Client UI State
- [ ] Add viewport tracking to client UI state
- [ ] Implement scroll input handling (mouse wheel, keyboard)
- [ ] Detect scroll-up to set "pinned" state
- [ ] Render pane content with viewport offset

### Section 4: New Content Indicator
- [ ] Create indicator widget ("▼ N new lines below")
- [ ] Position at bottom-right of pane content area
- [ ] Update line count as new content arrives
- [ ] Style with distinct color (yellow/orange)
- [ ] Debounce updates during rapid output

### Section 5: Navigation Actions
- [ ] Implement "jump to bottom" action
- [ ] Add keybinding: `G` and `Ctrl+End`
- [ ] Add click handler on indicator
- [ ] Implement smooth scroll option (configurable)
- [ ] Clear indicator and unpin on jump

### Section 6: Configuration
- [ ] Add scroll behavior to config schema
- [ ] `smooth_scroll` vs `instant_jump` setting
- [ ] Configurable keybindings for scroll actions

### Section 7: Testing
- [ ] Unit tests for viewport state management
- [ ] Integration tests for scroll behavior
- [ ] Test indicator updates with rapid output
- [ ] Test edge cases (empty pane, single line)

## Acceptance Criteria

- [ ] Scrolling up pins viewport and prevents auto-scroll
- [ ] New lines counted and displayed in indicator
- [ ] Indicator shows "▼ N new lines below" format
- [ ] `G` or `Ctrl+End` jumps to bottom
- [ ] Clicking indicator jumps to bottom
- [ ] Indicator disappears when at bottom
- [ ] Smooth scroll option works when configured
- [ ] No performance degradation with rapid output
- [ ] All tests passing
- [ ] `cargo clippy` clean

## Commands

```bash
# Build
cargo build -p ccmux-protocol -p ccmux-client -p ccmux-server

# Test
cargo test -p ccmux-protocol -p ccmux-client -p ccmux-server

# Clippy
cargo clippy --all -- -D warnings
```

## Notes

- Debounce indicator updates to prevent flicker during rapid output
- Use distinct color (yellow/orange) for visibility
- Consider showing indicator briefly even when following to acknowledge new content
- Future: click on indicator could show preview of latest content
