# Task Breakdown: BUG-047

**Work Item**: [BUG-047: Clean up compiler warnings across ccmux crates](PROMPT.md)
**Status**: Not Started
**Last Updated**: 2026-01-16

## Prerequisites

- [ ] Read and understand PROMPT.md
- [ ] Review PLAN.md and update if needed
- [ ] Run `cargo check 2>&1 | grep warning | wc -l` to get baseline warning count

## Phase 1: Auto-fixable Warnings (Unused Imports)

- [ ] Run `cargo fix --allow-dirty` to auto-remove unused imports
- [ ] Review the changes made by cargo fix
- [ ] Run `cargo check` to verify warnings reduced
- [ ] Run `cargo test` to ensure no breakage
- [ ] Commit: "fix: remove unused imports (cargo fix)"

## Phase 2: Deprecated Usage (PaneState::Claude)

- [ ] Find all uses: `rg "PaneState::Claude" --type rust`
- [ ] Replace with `PaneState::Agent` at each location:
  - [ ] ccmux-protocol/src/types.rs:527
  - [ ] ccmux-client/src/ui/app.rs:1882
  - [ ] ccmux-server/src/handlers/mcp_bridge.rs:57
  - [ ] ccmux-server/src/persistence/restoration.rs:308
  - [ ] ccmux-server/src/persistence/restoration.rs:354
- [ ] Run `cargo check` to verify deprecation warnings gone
- [ ] Run `cargo test` to ensure no breakage
- [ ] Commit: "fix: replace deprecated PaneState::Claude with PaneState::Agent"

## Phase 3: Dead Code Triage

### Triage Categories
For each dead code item, decide: REMOVE, KEEP (with allow), or WIRE UP

### beads.rs
- [ ] Review `is_beads_tracked` - REMOVE/KEEP/WIRE?
- [ ] Review `SocketNotFound` - REMOVE/KEEP/WIRE?
- [ ] Review `BEADS_*` constants - REMOVE/KEEP/WIRE?
- [ ] Review `repo_root` field - REMOVE/KEEP/WIRE?
- [ ] Apply decision and add `#[allow(dead_code)]` with comment if keeping

### handlers/mod.rs
- [ ] Review `GlobalBroadcast` variant - REMOVE/KEEP/WIRE?
- [ ] Review `resolve_active_pane` method - REMOVE/KEEP/WIRE?

### mcp/handlers.rs
- [ ] Review `mirror_pane` method - REMOVE/KEEP/WIRE?

### mcp/server.rs
- [ ] Review `with_managers` function - REMOVE/KEEP/WIRE?

### orchestration/router.rs
- [ ] Review `MessageRouter` and all methods - REMOVE/KEEP/WIRE?
- [ ] Review `RouterError` - REMOVE/KEEP/WIRE?

### orchestration/worktree.rs
- [ ] Review `is_git_repo` - REMOVE/KEEP/WIRE?

### persistence/ module
- [ ] Determine if persistence scaffolding is for planned feature
- [ ] Review checkpoint.rs: `extract_sequence`, `validate`
- [ ] Review replay.rs: `range`, `clear`
- [ ] Review restoration.rs: `without_pty_spawn`
- [ ] Review scrollback.rs: `ScrollbackConfig`, `ScrollbackCapture`, `ScrollbackRestore`
- [ ] Review types.rs: `Checkpoint::new`, `WalSegmentHeader`
- [ ] Review wal.rs: `WalConfig`, `Wal` methods, `WalReader`

### agents/claude/mod.rs
- [ ] Review `ClaudeAgentDetector` methods - REMOVE/KEEP/WIRE?

### observability/metrics.rs
- [ ] Review `record_replay_failed` - REMOVE/KEEP/WIRE?

### Apply Decisions
- [ ] Remove truly dead code
- [ ] Add `#[allow(dead_code)]` with justification for scaffolding
- [ ] Run `cargo check` to verify dead code warnings resolved
- [ ] Run `cargo test` to ensure no breakage
- [ ] Commit: "refactor: clean up dead code (BUG-047)"

## Phase 4: Unused Variables

- [ ] Fix agents/claude/mod.rs:67: `text` -> `_text` or remove
- [ ] Fix handlers/pane.rs:469: `pane` -> `_pane` or remove
- [ ] Fix mcp/handlers.rs:1112: `split_direction` -> `_split_direction` or remove
- [ ] Fix ccmux-client/src/ui/app.rs:2213: `ui_pane` -> `_ui_pane` or remove
- [ ] Run `cargo check` to verify unused variable warnings resolved
- [ ] Commit: "fix: prefix unused variables with underscore"

## Verification Tasks

- [ ] Run `cargo check 2>&1 | grep warning | wc -l` - compare to baseline
- [ ] Run `cargo test` - all tests pass
- [ ] Run `cargo clippy` - check for any new clippy warnings
- [ ] Update bug_report.json status to "fixed"
- [ ] Document any intentionally remaining warnings

## Completion Checklist

- [ ] All unused import warnings eliminated
- [ ] No deprecated PaneState::Claude usage
- [ ] Dead code handled (removed or explicitly allowed)
- [ ] Unused variables addressed
- [ ] Warning count near zero
- [ ] All tests passing
- [ ] PLAN.md updated with final approach
- [ ] Ready for review/merge

---
*Check off tasks as you complete them. Update status field above.*
