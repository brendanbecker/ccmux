# Session: Stream E - MCP Declarative Layout Tools

## Work Item
- **FEAT-045**: MCP Declarative Layout Tools

## Priority: P2

## Status: Complete (FEAT-046 and FEAT-045 implemented)

## Overview

Add declarative layout control to MCP tools, allowing LLMs to create complex terminal layouts from natural language descriptions like "create a 65% 35% vertical split with the right side split into quadrants".

## Problem Statement

Current `ccmux_create_pane` only creates single panes with 50/50 splits. No way to:
- Create multiple panes atomically in a layout
- Specify custom size ratios
- Target specific panes for splitting
- Resize existing panes

## New Tools to Implement

### 1. `ccmux_create_layout`
Create complex layouts declaratively in a single call.

```json
{
    "layout": {
        "direction": "horizontal",
        "splits": [
            {"ratio": 0.65, "layout": {"pane": {"command": "vim"}}},
            {"ratio": 0.35, "layout": {
                "direction": "vertical",
                "splits": [
                    {"ratio": 0.5, "layout": {"pane": {"command": "claude"}}},
                    {"ratio": 0.5, "layout": {"pane": {"command": "bash"}}}
                ]
            }}
        ]
    }
}
```

### 2. `ccmux_split_pane`
Split a specific pane with custom ratio (not just 50/50).

```json
{
    "pane_id": "abc123-uuid",
    "direction": "vertical",
    "ratio": 0.7,
    "command": "claude"
}
```

### 3. `ccmux_resize_pane`
Adjust pane sizes dynamically.

```json
{
    "pane_id": "abc123-uuid",
    "delta": 0.1
}
```

## Files to Modify

- `ccmux-server/src/mcp/tools.rs` - Tool definitions
- `ccmux-server/src/mcp/handlers.rs` - Layout parsing, pane spawning
- `ccmux-server/src/session/manager.rs` - Layout application methods
- `ccmux-client/src/ui/layout.rs` - Client-side layout sync (if needed)
- `ccmux-protocol/src/messages.rs` - Layout-related message types (if needed)

## Implementation Tasks

### Section 1: Tool Definitions
- [x] Add `ccmux_create_layout` tool definition
- [x] Add `ccmux_split_pane` tool definition
- [x] Add `ccmux_resize_pane` tool definition
- [x] Add `ccmux_rename_session` handler (was missing routing)

### Section 2: Layout Parsing
- [x] Implement `spawn_layout_panes()` to recursively parse layout spec
- [x] Handle nested layout parsing recursively
- [x] Validate ratios sum to 1.0 (normalize if not)
- [x] Add error handling for invalid specs

### Section 3: Pane Creation
- [x] Implement `spawn_single_pane()` for leaf pane creation
- [x] Spawn PTY for each leaf node
- [x] Collect pane IDs and names for response

### Section 4: Resize Implementation
- [x] Expose resize capability via `ccmux_resize_pane`
- [x] Validate delta bounds (-0.5 to 0.5)
- [x] Return resize_requested status for client-side handling

### Section 5: Testing
- [x] Unit tests for layout spec parsing
- [x] Tests for nested layouts
- [x] Test ratio normalization edge cases
- [x] Tests for split_pane, resize_pane, create_layout handlers

## Acceptance Criteria

- [x] `ccmux_create_layout` creates multi-pane layouts in one call
- [x] Nested layouts (splits within splits) work correctly
- [x] Custom ratios are respected (normalized if they don't sum to 1.0)
- [x] `ccmux_split_pane` splits a specific pane with custom ratio
- [x] `ccmux_resize_pane` adjusts pane sizes dynamically
- [ ] TUI clients display correct layout after MCP operations (client-side integration pending)

## Existing Infrastructure

The `layout.rs` in ccmux-client already has:
- `LayoutNode` - Tree structure for nested layouts
- `split_with_ratios()` - Custom ratio splits
- `LayoutPreset` - Common layout patterns

This feature primarily exposes these capabilities via MCP.

## Related Work Items

- See `feature-management/features/FEAT-045-mcp-declarative-layout-tools/PROMPT.md`

## Commands

```bash
# Build
cargo build --release

# Run tests
cargo test --workspace

# Test MCP tools
./target/release/ccmux-server mcp-bridge
```
