# gastown-ccmux-remote-support.md

# gastown ccmux Remote Support

Plan to extend gastown fork to use remote-capable ccmux (via peering/TCP) for polecats and agents, while keeping Mayor/orchestrator on laptop.

## Current gastown Multiplexer Integration
- Uses `GASTOWN_MULTIPLEXER_BIN` env var (e.g., `ccmux-client`).
- Spawn/attach commands call CLI equivalents (new-session, send-keys, attach).

## Remote Extension Goals
- Mayor session local (laptop) → low-latency control.
- Polecats/convoys remote (gaming PC) → offload compute/token burn.
- State sync via git/beads/hooks (no new persistence layer).

## Approach: Configurable CCMUX_ADDR
1. **gastown config/env**:
   - Add `GASTOWN_CCMUX_ADDR` (default: none/local Unix).
   - Examples:
     - `tcp://localhost:9999` (via SSH tunnel)
     - `tcp://gaming-pc:9999` (future direct)

2. **Preset / spawn logic refactor**:
   - In agent runtime code (cmd/gt or internal/crew):
     ```bash
     if GASTOWN_CCMUX_ADDR set:
         ccmux-client --addr $GASTOWN_CCMUX_ADDR new --name polecat-123 --command "claude --resume"
     else:
         ccmux-client new ...
     ```
   - Same for attach/send-keys.

3. **Beads formulas / hooks**:
   - Add remote-aware presets:
     ```toml
     [agent.remote-polecat]
     command = "ssh gaming-pc 'cd ~/gt && exec ...'"  # fallback
     # or direct: ccmux-client --addr tcp://... 
     ```
   - Hook sling → if task "heavy", use remote preset.

4. **State / sync considerations**:
   - Remote WAL → survives disconnects.
   - Rig git worktrees → push/pull from laptop.
   - Beads ledger → rsync or git commit hooks.

## Quick Setup Flow (MVP)
1. Daemon on gaming PC: `ccmux-server --listen-tcp 127.0.0.1:9999`
2. Laptop: `ssh -L 9999:localhost:9999 gaming-pc &`
3. `export GASTOWN_CCMUX_ADDR=tcp://localhost:9999`
4. `gt mayor attach` → Mayor local, spawns remote panes.

## Tradeoffs
- **Pros**: Offloads heavy runs; leverages peering without full rewrite.
- **Cons**: SSH tunnel management; potential latency on pane I/O.
- **Fallback**: Keep local ccmux for debug.

Next: Implement CCMUX_ADDR parsing + flag passthrough in gastown fork.
