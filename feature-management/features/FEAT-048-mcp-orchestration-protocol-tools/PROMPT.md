# FEAT-048: Expose orchestration protocol via MCP tools

**Priority**: P2
**Component**: ccmux-server
**Type**: new_feature
**Estimated Effort**: medium
**Business Value**: high

## Overview

Add MCP tools for the existing orchestration message types, enabling agents to communicate directly without going through shell commands. ccmux already has OrchestrationMessage types in `ccmux-protocol/src/messages.rs:8-67`:

- **Message Types**: StatusUpdate, TaskAssignment, TaskComplete, HelpRequest, Broadcast, SyncRequest
- **Targets**: Orchestrator, Session(Uuid), Broadcast, Worktree(String)

Currently these are only accessible via the daemon protocol (ClientMessage::SendOrchestration), not MCP.

## Benefits

- Agents become first-class participants in orchestration rather than passive recipients of nudges
- This is the "Orchestrator API Surface" Steve Yegge called for - enabling true agent-to-agent coordination
- Eliminates need for shell command workarounds for agent communication
- Provides structured, typed communication between agents

## Existing Protocol Types

From `ccmux-protocol/src/messages.rs`:

```rust
pub enum OrchestrationMessage {
    StatusUpdate {
        session_id: Uuid,
        status: WorkerStatus,
        message: Option<String>,
    },
    TaskAssignment {
        task_id: Uuid,
        description: String,
        files: Vec<String>,
    },
    TaskComplete {
        task_id: Uuid,
        success: bool,
        summary: String,
    },
    HelpRequest {
        session_id: Uuid,
        context: String,
    },
    Broadcast {
        from_session_id: Uuid,
        message: String,
    },
    SyncRequest,
}

pub enum WorkerStatus {
    Idle,
    Working,
    WaitingForInput,
    Blocked,
    Complete,
    Error,
}

pub enum OrchestrationTarget {
    Orchestrator,
    Session(Uuid),
    Broadcast,
    Worktree(String),
}
```

## Implementation Tasks

### Section 1: Core MCP Tool

- [ ] Add `ccmux_send_orchestration` MCP tool in `ccmux-server/src/mcp/tools.rs`
- [ ] Define schema for target parameter:
  - `{orchestrator: true}` - Send to orchestrator session
  - `{session: "uuid"}` - Send to specific session
  - `{broadcast: true}` - Broadcast to all sessions in same repo
  - `{worktree: "path"}` - Send to sessions in specific worktree
- [ ] Define schema for message parameter matching OrchestrationMessage variants
- [ ] Add handler that constructs and sends ClientMessage::SendOrchestration to daemon

### Section 2: Convenience Tools

- [ ] Add `ccmux_report_status(status, message)` - shorthand for StatusUpdate
  - Auto-fills session_id from current session context
  - status: one of "idle", "working", "waiting_for_input", "blocked", "complete", "error"
  - message: optional string
- [ ] Add `ccmux_request_help(context)` - shorthand for HelpRequest
  - Auto-fills session_id from current session context
  - Automatically targets Orchestrator
- [ ] Add `ccmux_broadcast(message)` - shorthand for Broadcast
  - Auto-fills from_session_id from current session context
  - Automatically uses Broadcast target

### Section 3: Subscription/Notification

- [ ] Add `ccmux_subscribe_orchestration` MCP tool (or use notifications)
- [ ] Investigate MCP notification mechanism for async message delivery
- [ ] Consider polling alternative if notifications are complex

### Section 4: Testing

- [ ] Add unit tests for tool schema validation
- [ ] Add integration tests for message routing
- [ ] Test all OrchestrationMessage variants
- [ ] Test all OrchestrationTarget variants
- [ ] Test error cases (NoRepository, NoRecipients)

### Section 5: Documentation

- [ ] Document tool schemas in MCP tool listing
- [ ] Add usage examples for common orchestration patterns
- [ ] Update CLAUDE.md with orchestration tool guidance

## Acceptance Criteria

- [ ] `ccmux_send_orchestration` tool is available and functional
- [ ] All OrchestrationMessage variants can be sent via MCP
- [ ] All OrchestrationTarget variants are supported
- [ ] Convenience tools simplify common operations
- [ ] Agents can receive orchestration messages (via subscription or polling)
- [ ] All tests passing
- [ ] Documentation updated

## Tool Schema Design

### ccmux_send_orchestration

```json
{
  "name": "ccmux_send_orchestration",
  "description": "Send orchestration message to other sessions",
  "inputSchema": {
    "type": "object",
    "properties": {
      "target": {
        "oneOf": [
          {"type": "object", "properties": {"orchestrator": {"const": true}}, "required": ["orchestrator"]},
          {"type": "object", "properties": {"session": {"type": "string", "format": "uuid"}}, "required": ["session"]},
          {"type": "object", "properties": {"broadcast": {"const": true}}, "required": ["broadcast"]},
          {"type": "object", "properties": {"worktree": {"type": "string"}}, "required": ["worktree"]}
        ]
      },
      "message": {
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "status_update": {
                "type": "object",
                "properties": {
                  "status": {"enum": ["idle", "working", "waiting_for_input", "blocked", "complete", "error"]},
                  "message": {"type": "string"}
                },
                "required": ["status"]
              }
            }
          },
          {
            "type": "object",
            "properties": {
              "task_assignment": {
                "type": "object",
                "properties": {
                  "task_id": {"type": "string", "format": "uuid"},
                  "description": {"type": "string"},
                  "files": {"type": "array", "items": {"type": "string"}}
                },
                "required": ["task_id", "description", "files"]
              }
            }
          },
          {
            "type": "object",
            "properties": {
              "task_complete": {
                "type": "object",
                "properties": {
                  "task_id": {"type": "string", "format": "uuid"},
                  "success": {"type": "boolean"},
                  "summary": {"type": "string"}
                },
                "required": ["task_id", "success", "summary"]
              }
            }
          },
          {
            "type": "object",
            "properties": {
              "help_request": {
                "type": "object",
                "properties": {
                  "context": {"type": "string"}
                },
                "required": ["context"]
              }
            }
          },
          {
            "type": "object",
            "properties": {
              "broadcast": {
                "type": "object",
                "properties": {
                  "message": {"type": "string"}
                },
                "required": ["message"]
              }
            }
          },
          {
            "type": "object",
            "properties": {
              "sync_request": {"const": true}
            }
          }
        ]
      }
    },
    "required": ["target", "message"]
  }
}
```

### ccmux_report_status (convenience)

```json
{
  "name": "ccmux_report_status",
  "description": "Report current session status to orchestrator",
  "inputSchema": {
    "type": "object",
    "properties": {
      "status": {
        "type": "string",
        "enum": ["idle", "working", "waiting_for_input", "blocked", "complete", "error"]
      },
      "message": {
        "type": "string",
        "description": "Optional status message"
      }
    },
    "required": ["status"]
  }
}
```

## Notes

- The daemon already handles SendOrchestration messages, so this is primarily an MCP surface layer
- Need to determine how to get current session context for auto-fill features
- Consider rate limiting for broadcast messages
- Error handling should surface NoRepository and NoRecipients errors clearly
