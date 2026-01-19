# BUG-061: ccmux_send_orchestration target parameter parsing fails

**Priority**: P2
**Component**: mcp
**Severity**: medium
**Status**: new

## Problem

`ccmux_send_orchestration` fails with "Invalid target" error even when valid target objects are provided. The target parameter is not being parsed correctly when passed through the MCP protocol.

```
MCP error -32602: Invalid target: must specify 'tag', 'session', 'broadcast', or 'worktree'
```

## Reproduction Steps

1. Attach to a session: `ccmux_attach_session`
2. Call `ccmux_send_orchestration` with any valid target:
   - `{"target": {"broadcast": true}, "msg_type": "test", "payload": {}}`
   - `{"target": {"tag": "orchestrator"}, "msg_type": "test", "payload": {}}`
3. Observe: "Invalid target" error returned

## Expected Behavior

Message is sent to the specified target (broadcast to all, or to sessions with tag).

## Actual Behavior

All target formats fail with the same error, indicating the target object is not being parsed correctly.

## Root Cause Analysis

The `ccmux_send_orchestration` handler in `mod.rs:502-508` extracts the target as:
```rust
let target = &arguments["target"];
```

But when MCP clients (including Claude Code) pass object parameters, they may arrive as JSON strings rather than parsed objects. The `ccmux_create_layout` tool handles this case (lines 447-455):

```rust
let layout = match &raw_layout {
    serde_json::Value::String(s) => {
        serde_json::from_str(s).map_err(|e| {
            McpError::InvalidParams(format!("Invalid layout JSON string: {}", e))
        })?
    }
    other => other.clone(),
};
```

The `ccmux_send_orchestration` tool lacks this string-to-object parsing.

## Fix

Add the same string parsing logic to `ccmux_send_orchestration` in `ccmux-server/src/mcp/bridge/mod.rs`:

```rust
"ccmux_send_orchestration" => {
    let raw_target = arguments["target"].clone();
    let target = match &raw_target {
        serde_json::Value::String(s) => {
            serde_json::from_str(s).map_err(|e| {
                McpError::InvalidParams(format!("Invalid target JSON string: {}", e))
            })?
        }
        other => other.clone(),
    };
    let msg_type = arguments["msg_type"]
        .as_str()
        .ok_or_else(|| McpError::InvalidParams("Missing 'msg_type' parameter".into()))?;
    let payload = arguments["payload"].clone();
    handlers.tool_send_orchestration(&target, msg_type, payload).await
}
```

## Acceptance Criteria

- [ ] `ccmux_send_orchestration` accepts target as JSON object
- [ ] `ccmux_send_orchestration` accepts target as JSON string (for compatibility)
- [ ] All target types work: `tag`, `session`, `broadcast`, `worktree`
- [ ] Add test coverage for both object and string parameter formats

## Related Files

- `ccmux-server/src/mcp/bridge/mod.rs:502-508` - dispatch logic (needs fix)
- `ccmux-server/src/mcp/bridge/mod.rs:447-455` - reference implementation in create_layout
- `ccmux-server/src/mcp/bridge/handlers.rs:1027-1073` - handler (correct, issue is in dispatch)

## Notes

- Discovered during QA of BUG-060 fix
- Other orchestration tools (`ccmux_report_status`, `ccmux_broadcast`, `ccmux_set_tags`) work correctly
- Only `ccmux_send_orchestration` is affected due to its complex object parameter
