# BUG-042: Excessive Result Nesting (Ok(Ok(...))) in MCP Handlers

## Overview
During the FEAT-064 refactor and subsequent merges, we observed pattern matching on `Ok(Ok(ServerMessage::Error { ... }))` in `ccmux-server/src/mcp/bridge/handlers.rs`. This indicates that `recv_response_from_daemon` or the handler logic is double-wrapping results, forcing verbose and brittle matching.

## Impact
- **Maintainability**: Harder to read and modify handler logic.
- **Error Handling**: Increases the chance of missing an error case if one layer of `Result` is handled but the inner one isn't.
- **Code Cleanliness**: Typical Rust idiom prefers flattening `Result` chains using `?` or mapping errors early.

## Analysis
The likely cause is the interaction between `tokio::time::timeout` (which returns a `Result`) and the underlying `recv_from_daemon` (which also returns a `Result`).

Current flow (hypothetical):
1. `recv_from_daemon()` -> `Result<ServerMessage, McpError>`
2. `tokio::time::timeout(...)` -> `Result<Result<ServerMessage, McpError>, Elapsed>`

This leads to `Ok(Ok(msg))` matching.

## Proposed Fix
1. **Flatten in Helper**: The helper method `recv_response_from_daemon` should handle the timeout `Result` internally and return a single `Result<ServerMessage, McpError>`.
   - If timeout: map `Elapsed` to `McpError::ResponseTimeout`.
   - If success: return the inner `Result`.
2. **Refactor Match Arms**: Update `handlers.rs` to match on `Ok(ServerMessage::...)` or `Err(McpError::...)`.

## Tasks
- [ ] Analyze `recv_response_from_daemon` signature and implementation in `connection.rs`.
- [ ] Refactor to flatten the return type.
- [ ] Update all call sites in `handlers.rs` to remove the double `Ok`.
