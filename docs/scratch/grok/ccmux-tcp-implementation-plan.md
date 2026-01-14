# ccmux-tcp-implementation-plan.md

# ccmux TCP Listener Implementation Plan

Step-by-step plan to add optional TCP support to the daemon and client, enabling peering via SSH tunnels or direct (future TLS).

## Prerequisites
- Tokio runtime already in use (async I/O).
- bincode for serialization (in ccmux-protocol).
- Add clap or similar for CLI flags if not present.

## Phase 1: MVP – Configurable TCP Listener (Daemon)
1. **Config / CLI extension** (`ccmux-server`):
   - Add to config.toml:
     ```toml
     [server]
     listen_unix = "~/.ccmux/ccmux.sock"  # default
     listen_tcp = "127.0.0.1:9999"        # optional, "" to disable
     ```
   - Or CLI: `ccmux-server --listen-tcp 127.0.0.1:9999`

2. **Server startup refactor** (`ccmux-server/src/main.rs` or server init):
   - Parse config/CLI for listen addrs.
   - Spawn separate accept loops:
     ```rust
     // Unix
     if let Some(unix_path) = config.listen_unix {
         tokio::spawn(accept_unix(unix_path, tx.clone()));
     }
     // TCP
     if let Some(tcp_addr) = config.listen_tcp {
         tokio::spawn(accept_tcp(tcp_addr, tx.clone()));
     }
     ```
   - `accept_tcp`: `TcpListener::bind(addr).await?` → loop accept → handle connection.

3. **Connection handler**:
   - Same as current Unix: read bincode frames → process messages → write responses.
   - Use `tokio_util::codec::LengthDelimitedCodec` if bincode needs framing (test raw bincode first).

4. **Test locally**:
   - Run daemon with TCP → `nc localhost 9999` or custom client.
   - Verify message round-trip.

## Phase 2: Client Support
1. **Client connect logic** (`ccmux-client`):
   - Add `--addr` flag or env `CCMUX_ADDR` (default unix://...).
   - Parse: if tcp://host:port → `TcpStream::connect`.
   - Fallback to Unix if not specified.

2. **Minimal client** for testing:
   ```bash
   ccmux-client --addr tcp://localhost:9999
   ```

## Dependencies to Add (if needed)
- `tokio::net::TcpListener` / `TcpStream` (already via tokio).
- Optional: `tokio-util` for `LengthDelimitedCodec` (if bincode framing issues).

## Test Cases
- Local Unix → unchanged.
- Local TCP (127.0.0.1) → works.
- SSH tunnel: daemon TCP on remote → tunnel → client local TCP → success.
- Disconnect mid-session → client reconnects (phase 2).
- Multiple clients attach → server handles concurrent connections.

## Effort
- ~1–2 days for MVP (config + dual listener + client connect).
- Add TLS/auth in phase 3.

See peering-design.md for overall flows.
