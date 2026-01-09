# Bug Reports

**Project**: ccmux
**Last Updated**: 2026-01-09

## Summary Statistics
- Total Bugs: 4
- New: 2
- In Progress: 0
- Resolved: 1

## Bugs by Priority

### P0 - Critical (0)

*No P0 bugs*

### P1 - High Priority (2)

#### BUG-005: Sideband parsing not integrated into PTY output flow [NEW]

**Status**: New
**Filed**: 2026-01-09
**Component**: ccmux-server
**Directory**: [BUG-005-sideband-parsing-not-integrated](BUG-005-sideband-parsing-not-integrated/)

**Description**:
Sideband commands (`<ccmux:spawn>`, `<ccmux:focus>`, etc.) output by Claude are displayed as literal text instead of being parsed and executed. The sideband parsing infrastructure (FEAT-019, FEAT-030) exists but is not wired into the PTY output flow.

**Root Cause**:
- FEAT-019 implemented `SidebandParser` and command types
- FEAT-030 implemented `CommandExecutor` with spawn functionality
- The integration point - wiring parser/executor into `PtyOutputPoller::flush()` - was never completed
- `SidebandParser` and `CommandExecutor` are only instantiated in test code
- No code in server runtime creates these components

**Impact**:
Claude cannot control ccmux via sideband commands. The core value proposition of Claude-ccmux integration (autonomous pane spawning, input routing, notifications) is non-functional.

**Key Files**:
- `ccmux-server/src/pty/output.rs:389-394` - flush() broadcasts raw data
- `ccmux-server/src/sideband/mod.rs:63-72` - executor only in tests
- `ccmux-server/src/main.rs` - no sideband instantiation

---

#### BUG-004: Client hangs when reattaching to session with dead pane [RESOLVED]

**Status**: Resolved
**Filed**: 2026-01-09
**Resolved**: 2026-01-09
**Component**: ccmux-server

**Description**:
Client becomes unresponsive when attaching to a session whose pane's shell has exited. The session/pane remained in server state as zombies after the PTY output poller exited.

**Root Cause**:
PTY output poller only broadcast `PaneClosed` to clients but did not clean up pane/session from server state, leaving zombie sessions that could be attached to but had no active output poller.

**Resolution**:
Added automatic cleanup when PTY processes exit:
- New `PaneClosedNotification` channel from output pollers
- `run_pane_cleanup_loop()` task removes dead panes, empty windows, empty sessions
- All new panes get cleanup channel via `HandlerContext`

### P2 - Medium Priority (1)

#### BUG-002: Flaky test `test_ensure_dir_nested` due to shared temp directory

**Status**: New
**Filed**: 2026-01-09
**Component**: ccmux-utils
**File**: `ccmux-utils/src/paths.rs:413`

**Description**:
The test `test_ensure_dir_nested` intermittently fails when running the full test suite in parallel, but passes when run in isolation.

**Root Cause**:
Two tests share the same base directory path using `std::process::id()`:
- `test_ensure_dir_creates_directory` uses `ccmux_test_{pid}/`
- `test_ensure_dir_nested` uses `ccmux_test_{pid}/nested/deep`

When tests run in parallel, one test may delete the shared base directory while the other test is attempting to use it, causing a race condition.

**Error Message**:
```
thread 'paths::tests::test_ensure_dir_nested' panicked at ccmux-utils/src/paths.rs:428:9:
assertion failed: result.is_ok()
```

**Steps to Reproduce**:
1. Run `cargo test --workspace`
2. Test may fail intermittently (not always reproducible)
3. Running `cargo test -p ccmux-utils test_ensure_dir_nested` passes consistently

**Suggested Fix**:
Use `tempfile::TempDir` for test isolation, or use unique directory names for each test (e.g., include test function name in the path).

Example fix:
```rust
fn test_ensure_dir_nested() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let test_dir = temp_dir.path().join("nested").join("deep");
    // ... rest of test
}
```

### P3 - Low Priority (0)

*No P3 bugs*

## Recent Activity

| Date | Bug ID | Action | Description |
|------|--------|--------|-------------|
| 2026-01-09 | BUG-005 | Filed | Sideband parsing not integrated into PTY output flow |
| 2026-01-09 | BUG-004 | Filed & Resolved | Zombie panes causing client hang |
| 2026-01-09 | BUG-002 | Filed | Flaky test due to shared temp directory |
