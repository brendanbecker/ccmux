**Debugging Tokio Broadcast Channel Issue in Rust Terminal Multiplexer**

I'm building a terminal multiplexer in Rust with tokio. The architecture:

- **Server**: Manages sessions/windows/panes with PTYs, accepts Unix socket connections
- **TUI Client**: Connects via Unix socket, renders terminal output
- **MCP Bridge**: Separate process that also connects via Unix socket, allows Claude AI to control panes

**The Problem**: When the MCP bridge creates a new pane via RPC, the server correctly creates the pane and PTY, but the TUI client never receives the broadcast notification about the new pane. The TUI continues showing the old pane layout.

**What Works**:
- TUI client creating panes via keybinds → broadcasts received, UI updates
- PTY output broadcasting → all clients receive terminal output
- MCP bridge reading pane output → works fine

**What Doesn't Work**:
- MCP bridge creating pane → server logs show broadcast sent, TUI never receives it

**Architecture Details**:
- Using `tokio::sync::broadcast` channels for notifications
- Each client connection spawns a task that holds a `broadcast::Receiver`
- Server holds `broadcast::Sender`, clones it for each new subscriber
- Messages are serialized with bincode over Unix socket

**What I've Tried**:
- Added debug logging confirming broadcast is sent
- Verified client count in broadcast channel
- Tests pass (single-process tests with mock clients)

**Questions**:
1. What are common pitfalls with tokio broadcast channels that could cause one subscriber to miss messages?
2. Could there be a race condition where the TUI's receiver isn't subscribed to the right channel instance?
3. Are there known issues with broadcast channels across process boundaries (even though I'm using Unix sockets, not the channel directly)?
4. What debugging strategies would you recommend?
