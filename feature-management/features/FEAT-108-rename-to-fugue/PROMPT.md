# FEAT-108: Rename Project to Fugue

**Priority**: P1
**Component**: all
**Effort**: Large
**Status**: new

## Summary

Rename the project from "ccmux" to "fugue" - a musical term meaning a contrapuntal composition where multiple independent voices enter successively and interweave around a common theme. The metaphor fits agent orchestration perfectly: multiple agents (voices) working independently but in harmony.

## Etymology

From Latin *fuga* meaning "flight" or "chase." In a fugue, each voice "chases" the previous one, entering with the same theme at different pitches while maintaining its own independent line.

## Scope

### 1. Crate Directory Renames (6 directories)

```
ccmux-client/   → fugue-client/
ccmux-server/   → fugue-server/
ccmux-protocol/ → fugue-protocol/
ccmux-utils/    → fugue-utils/
ccmux-sandbox/  → fugue-sandbox/
ccmux-compat/   → fugue-compat/
```

### 2. Cargo.toml Updates (7 files)

- Workspace root: member names, repository URL
- Each crate: package name, binary name, dependency references

### 3. MCP Tool Renames (47 tools)

All `ccmux_*` tools become `fugue_*`:

```
ccmux_list_panes      → fugue_list_panes
ccmux_create_session  → fugue_create_session
ccmux_send_input      → fugue_send_input
ccmux_report_status   → fugue_report_status
... (47 total)
```

**Key file**: `ccmux-server/src/mcp/tools.rs`

### 4. Core Constant

```rust
// ccmux-utils/src/paths.rs:10
const APP_NAME: &str = "ccmux";  →  const APP_NAME: &str = "fugue";
```

This drives path construction for config, sockets, logs, etc.

### 5. Environment Variables

```
CCMUX_PANE_ID  → FUGUE_PANE_ID
CCMUX_ADDR     → FUGUE_ADDR
CCMUX_LOG      → FUGUE_LOG
```

### 6. Exit Markers

```
___CCMUX_EXIT_<code>___ → ___FUGUE_EXIT_<code>___
```

### 7. Config/Socket Paths

```
~/.config/ccmux/     → ~/.config/fugue/
~/.ccmux/            → ~/.fugue/
ccmux.sock           → fugue.sock
ccmux.pid            → fugue.pid
ccmux.log            → fugue.log
```

### 8. Binary Names

```
ccmux        → fugue
ccmux-server → fugue-server
ccmux-compat → fugue-compat
```

### 9. Import Statements (~100+ files)

```rust
use ccmux_protocol:: → use fugue_protocol::
use ccmux_utils::    → use fugue_utils::
use ccmux_server::   → use fugue_server::
use ccmux_client::   → use fugue_client::
```

### 10. Documentation

- README.md
- docs/*.md
- CLAUDE.md / AGENTS.md
- feature-management/ PROMPT.md files

### 11. GitHub Repository

- Rename repo: `brendanbecker/ccmux` → `brendanbecker/fugue`
- Update all repository URL references

## Implementation Order

1. **Rename directories** (git mv for history preservation)
2. **Update APP_NAME constant** in paths.rs
3. **Update all Cargo.toml** files (package names, deps, binaries)
4. **Bulk find/replace** in .rs files:
   - `ccmux_` → `fugue_` (MCP tools, env vars, markers)
   - `ccmux-` → `fugue-` (crate names in use statements)
   - `ccmux::` → `fugue::` (if any)
5. **Update documentation**
6. **Build and test**
7. **Rename GitHub repo** (after merge)

## Acceptance Criteria

- [ ] All crate directories renamed
- [ ] `cargo build` succeeds
- [ ] `cargo test` passes
- [ ] Binary produces `fugue` and `fugue-server`
- [ ] MCP tools register as `fugue_*`
- [ ] Config reads from `~/.config/fugue/`
- [ ] Socket created as `fugue.sock`
- [ ] All docs updated
- [ ] No remaining "ccmux" references (except git history)

## Migration Notes

Existing users will need to:
1. Move config: `mv ~/.config/ccmux ~/.config/fugue`
2. Update MCP config to reference `fugue-server`
3. Update any scripts referencing `ccmux` binary

## Related

- This is a breaking change; consider major version bump
