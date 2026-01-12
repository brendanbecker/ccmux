# Task Breakdown: FEAT-049

**Work Item**: [FEAT-049: Add tmux-compatible CLI wrapper (ccmux-compat)](PROMPT.md)
**Status**: Not Started
**Last Updated**: 2026-01-10

## Prerequisites

- [ ] Read and understand PROMPT.md
- [ ] Review PLAN.md and update if needed
- [ ] Review Gas Town's tmux usage patterns (internal/tmux/tmux.go)
- [ ] Review existing ccmux MCP tools

## Crate Setup Tasks

- [ ] Create ccmux-compat/ directory structure
- [ ] Create Cargo.toml with dependencies
- [ ] Add ccmux-compat to workspace Cargo.toml
- [ ] Create src/main.rs with basic structure
- [ ] Verify crate builds

## CLI Parsing Tasks

- [ ] Set up clap with tmux-style subcommands
- [ ] Implement `new-session` argument parsing
  - [ ] -d (detached)
  - [ ] -s NAME (session name)
  - [ ] -c DIR (working directory)
  - [ ] [CMD] (initial command)
- [ ] Implement `send-keys` argument parsing
  - [ ] -t TARGET (target)
  - [ ] -l (literal)
  - [ ] TEXT... (keys to send)
- [ ] Implement `kill-session` argument parsing
  - [ ] -t NAME (target)
- [ ] Implement `has-session` argument parsing
  - [ ] -t =NAME (exact match target)
- [ ] Implement `list-sessions` argument parsing
  - [ ] -F FORMAT (format string)
- [ ] Implement `capture-pane` argument parsing
  - [ ] -p (print to stdout)
  - [ ] -t TARGET (target)
  - [ ] -S start (start line)
  - [ ] -E end (end line)

## Connection Layer Tasks

- [ ] Create connection module
- [ ] Implement Unix socket connection
- [ ] Implement protocol codec (reuse from ccmux-protocol)
- [ ] Handle connection errors with appropriate exit codes
- [ ] Auto-start server if not running (like ccmux-client)

## Command Translation Tasks

- [ ] Create translation module
- [ ] Implement new-session translation
  - [ ] Map to ccmux_create_session
  - [ ] Handle initial command via ccmux_send_input
- [ ] Implement send-keys translation
  - [ ] Map to ccmux_send_input
  - [ ] Handle special keys (Enter, C-c, etc.)
  - [ ] Handle -l literal mode
- [ ] Implement kill-session translation
  - [ ] Map to ccmux_kill_session
- [ ] Implement has-session translation
  - [ ] Map to ccmux_list_sessions + filter
  - [ ] Return exit code 0/1
- [ ] Implement list-sessions translation
  - [ ] Map to ccmux_list_sessions
  - [ ] Apply format string
- [ ] Implement capture-pane translation
  - [ ] Map to ccmux_read_pane
  - [ ] Handle line range options

## Output Formatting Tasks

- [ ] Create output module
- [ ] Implement format string parser
- [ ] Implement format variable substitution
  - [ ] #{session_name}
  - [ ] #{session_windows}
  - [ ] #{session_created}
  - [ ] #{session_attached}
- [ ] Match tmux default format exactly
- [ ] Ensure newlines and whitespace match

## Exit Code Tasks

- [ ] Document tmux exit codes for each command
- [ ] Implement matching exit codes
  - [ ] 0: success
  - [ ] 1: not found / no sessions
  - [ ] 2: invalid arguments
- [ ] Test exit codes match tmux

## Testing Tasks

- [ ] Create tests/ directory
- [ ] Add unit tests for CLI parsing
- [ ] Add unit tests for format string parsing
- [ ] Add integration tests for each command
- [ ] Create comparison test script (tmux vs ccmux-compat)
- [ ] Test with mock Gas Town usage patterns

## Documentation Tasks

- [ ] Add README.md for ccmux-compat
- [ ] Document supported commands
- [ ] Document known differences from tmux
- [ ] Add migration guide
- [ ] Update workspace README

## Verification Tasks

- [ ] All acceptance criteria from PROMPT.md met
- [ ] Tests passing
- [ ] Update feature_request.json status
- [ ] Manual testing with Gas Town patterns

## Completion Checklist

- [ ] All implementation tasks complete
- [ ] All tests passing
- [ ] Documentation updated
- [ ] PLAN.md reflects final implementation
- [ ] Ready for review/merge

---
*Check off tasks as you complete them. Update status field above.*
