# ccmux

**Claude Code-aware terminal multiplexer**

A terminal multiplexer built in Rust that understands Claude Code. Unlike tmux, ccmux knows when Claude is thinking, waiting, or ready for input—and can be controlled by Claude itself via MCP.

## Features

### Working Now
- **tmux-like experience**: Prefix keybinds (Ctrl+b), sessions, windows, panes
- **Auto-start**: Just run `ccmux`, server starts automatically if not running
- **Session persistence**: Sessions survive server restarts
- **MCP integration**: Claude can control ccmux via 11 MCP tools
- **Sideband commands**: Claude can spawn panes via `<ccmux:spawn>` tags in output
- **Claude detection**: Tracks Claude state (thinking, waiting, tool use)
- **Configurable**: Hot-reload config, customizable keybinds and default commands

### Keybinds

**Session Selection** (on startup):
| Key | Action |
|-----|--------|
| `n` | Create new session |
| `Ctrl+D` | Delete selected session |
| `Enter` | Attach to session |
| `j/k` or arrows | Navigate |
| `q` | Quit |

**Prefix Mode** (Ctrl+b, then...):
| Key | Action |
|-----|--------|
| `c` | Create window |
| `%` | Split vertical |
| `"` | Split horizontal |
| `n/p` | Next/prev window |
| `o` | Next pane |
| `h/j/k/l` | Vim-style pane navigation |
| `z` | Zoom pane (fullscreen) |
| `d` | Detach |
| `s` | Session picker |
| `x` | Close pane |
| `&` | Close window |

**Quick Navigation** (no prefix):
| Key | Action |
|-----|--------|
| `Ctrl+PageUp/Down` | Switch windows |
| `Ctrl+Shift+PageUp/Down` | Switch panes |

## Installation

```bash
# Build
cargo build --release

# Run (auto-starts server)
./target/release/ccmux

# Run with custom command
./target/release/ccmux bash
./target/release/ccmux claude --resume
```

## Configuration

Config file: `~/.config/ccmux/config.toml`

```toml
[general]
# Auto-launch Claude in every new session
default_command = "claude"
```

## MCP Integration

Claude Code can control ccmux sessions via MCP. Add to `~/.claude/mcp.json`:

```json
{
  "mcpServers": {
    "ccmux": {
      "command": "/path/to/ccmux-server",
      "args": ["mcp-bridge"]
    }
  }
}
```

**Available MCP Tools**:
- `ccmux_list_sessions`, `ccmux_create_session`, `ccmux_rename_session`
- `ccmux_list_windows`, `ccmux_create_window`
- `ccmux_list_panes`, `ccmux_create_pane`, `ccmux_close_pane`, `ccmux_focus_pane`
- `ccmux_read_pane`, `ccmux_send_input`, `ccmux_get_status`

## Architecture

```
ccmux/
├── ccmux-client/     # TUI client (ratatui + crossterm)
├── ccmux-server/     # Daemon with PTY management + MCP bridge
├── ccmux-protocol/   # Message types and codec (bincode)
├── ccmux-session/    # Session/window/pane hierarchy
├── ccmux-utils/      # Shared utilities
└── ccmux-persistence/# WAL-based crash recovery
```

**Communication**:
- Client ↔ Server: Unix socket with bincode-serialized messages
- MCP Bridge: Connects to same daemon, translates JSON-RPC to internal commands

## Development

```bash
# Build
cargo build --release

# Run tests (1,200+ tests)
cargo test --workspace

# Run server manually
./target/release/ccmux-server

# Run client
./target/release/ccmux
```

## Known Issues

- `kill -9` on server corrupts terminal (run `reset` to fix)
- Mouse scroll not working (BUG-013)
- Large paste can crash session (BUG-011)

## Prior Art

- **tmux** - The standard, but process-unaware
- **Zellij** - Modern Rust multiplexer with plugins
- **Wezterm** - GPU-accelerated terminal with mux mode

## License

MIT License. See [LICENSE.md](LICENSE.md) for details.
