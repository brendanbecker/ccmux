# Session: Stream E - MCP Tool Enhancements

## Work Items
- **FEAT-045**: MCP Declarative Layout Tools
- **FEAT-046**: MCP Focus/Select Control

## Priority: P1/P2

---

## FEAT-046: MCP Focus/Select Control (P1 - Do First)

### Problem

Currently, `ccmux_create_pane` auto-switches focus to new pane. This is problematic for orchestration where an LLM wants to spawn workers without losing its place.

### New Tools to Add

1. **Update `ccmux_create_pane`**: Add `select: bool` parameter (default: false)
2. **`ccmux_select_pane`**: Switch focus to specific pane by UUID
3. **`ccmux_select_window`**: Switch to specific window/tab
4. **`ccmux_select_session`**: Switch to different session
5. **Update `ccmux_list_panes`**: Add `focused` field to response

### Files to Modify

- `ccmux-server/src/mcp/tools.rs` - Tool definitions
- `ccmux-server/src/mcp/handlers.rs` - Implement handlers
- `ccmux-server/src/handlers/mcp_bridge.rs` - Route focus commands
- `ccmux-protocol/src/messages.rs` - Add focus message types
- `ccmux-client/src/ui/app.rs` - Handle focus messages

---

## FEAT-045: MCP Declarative Layout Tools (P2)

### Problem

Current `ccmux_create_pane` only creates single panes with 50/50 splits. No way to:
- Create multiple panes atomically in a layout
- Specify custom size ratios
- Target specific panes for splitting
- Resize existing panes

### New Tools to Add

1. **`ccmux_create_layout`**: Create complex layouts declaratively
   ```json
   {
     "layout": {
       "direction": "horizontal",
       "splits": [
         {"ratio": 0.65, "layout": {"pane": {"command": "vim"}}},
         {"ratio": 0.35, "layout": {"pane": {"command": "claude"}}}
       ]
     }
   }
   ```

2. **`ccmux_split_pane`**: Split specific pane with custom ratio

3. **`ccmux_resize_pane`**: Adjust pane sizes dynamically

### Files to Modify

- `ccmux-server/src/mcp/tools.rs` - Tool definitions
- `ccmux-server/src/mcp/handlers.rs` - Layout parsing, pane spawning
- `ccmux-server/src/session/manager.rs` - Layout application methods
- `ccmux-client/src/ui/layout.rs` - Client-side layout sync

---

## Implementation Order

1. **FEAT-046 first** - Simpler, critical for orchestration
2. **FEAT-045 second** - More complex, builds on FEAT-046

## Acceptance Criteria

### FEAT-046
- [x] `ccmux_create_pane` does NOT auto-switch focus by default (added `select: bool` parameter, defaults to false)
- [x] `ccmux_select_pane/window/session` work correctly (ccmux_focus_pane existed, added ccmux_select_window and ccmux_select_session)
- [x] `ccmux_list_panes` includes `is_focused` field

### FEAT-045
- [ ] `ccmux_create_layout` creates multi-pane layouts
- [ ] Custom ratios are respected (not just 50/50)
- [ ] `ccmux_resize_pane` adjusts sizes dynamically

## Related Work Items

- See `feature-management/features/FEAT-045-mcp-declarative-layout-tools/PROMPT.md`
- See `feature-management/features/FEAT-046-mcp-focus-select-control/PROMPT.md`

## Commands

```bash
# Build
cargo build --release

# Run tests
cargo test --workspace

# Test MCP tools
./target/release/ccmux-server mcp-bridge
```
