# ccmux Project Handoff

> **LIVING DOCUMENT**: This handoff file is the interface between sessions. Update it constantly as you work—mark completed items, add discoveries, note blockers, revise plans. The next session (or a resumed session) relies on this being current.

## Context

**ccmux** is a Claude Code-aware terminal multiplexer in Rust. Development follows the [Context Engineering Methodology](./CONTEXT_ENGINEERING_METHODOLOGY.md).

**Current Stage**: Stage 5 In Progress (Implementation Planning)
**Next Stage**: Stage 6 (Implementation)

## Completed Work

### Stage 1: Ideation
Produced `docs/DEEP_RESEARCH_PROMPT.md` covering 7 research areas.

### Stage 2: Deep Research
Gathered research from three LLMs:
- `docs/research/claude_research.md` (24KB, ~4,500 tokens)
- `docs/research/gemini_research.md` (36KB, ~8,500 tokens)
- `docs/research/chatgpt_research.pdf` (351KB, ~12,000 tokens)

### Stage 3: Document Parsing
Parsed all research into structured, navigable chunks:
- 12 parsed output files in `docs/research/parsed/`
- Key deliverable: `docs/research/SYNTHESIS.md` - unified cross-source analysis

### Stage 4: Architecture Generation
Generated formal architecture documentation:

```
docs/architecture/
├── ARCHITECTURE.md         # System overview, client-server model, data flow
├── CRATE_STRUCTURE.md      # Workspace layout, dependency graph
├── CLAUDE_INTEGRATION.md   # Detection, session management, communication
├── PERSISTENCE.md          # Checkpoint + WAL, recovery flow
├── CONFIGURATION.md        # Hot-reload with ArcSwap, validation
└── ADR/
    ├── 001-terminal-parser.md      # vt100 first, migrate if needed
    ├── 002-claude-communication.md # Both MCP and sideband protocols
    └── 003-session-isolation.md    # CLAUDE_CONFIG_DIR approach
```

### Stage 5: Implementation Planning (In Progress)
Feature enumeration completed via parallel agents (4 batches of 4 agents each):

**14 Features Identified:**

| # | Feature | Crate | Description |
|---|---------|-------|-------------|
| 1 | Protocol Layer | ccmux-protocol | Client/server message types, codec |
| 2 | Utilities | ccmux-utils | Error types, logging, path utilities |
| 3 | Client UI | ccmux-client | Ratatui widgets, pane rendering |
| 4 | Client Input | ccmux-client | Keyboard/mouse handling, prefix keys |
| 5 | Client Connection | ccmux-client | Unix socket client, message framing |
| 6 | Session Management | ccmux-server | Session/window/pane hierarchy |
| 7 | PTY Management | ccmux-server | portable-pty lifecycle, I/O |
| 8 | Terminal Parsing | ccmux-server | vt100 parser integration |
| 9 | Claude Detection | ccmux-server | State detection, session ID capture |
| 10 | Persistence | ccmux-server | Checkpoint + WAL, crash recovery |
| 11 | Configuration | ccmux-server | Hot-reload with ArcSwap |
| 12 | MCP Server | ccmux-server | Model Context Protocol tools |
| 13 | Sideband Protocol | ccmux-server | XML command parsing from Claude output |
| 14 | Session Isolation | ccmux-server | CLAUDE_CONFIG_DIR per pane |

**Implementation Order** (based on dependencies):
1. Protocol Layer + Utilities (foundation, no deps)
2. Client Connection + PTY Management (infrastructure)
3. Terminal Parsing + Session Management (core server)
4. Client UI + Client Input (core client)
5. Claude Detection + Session Isolation (Claude awareness)
6. Persistence + Configuration (reliability)
7. MCP Server + Sideband Protocol (advanced integration)

## Key Architectural Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Architecture | Client-server | Crash isolation, detach/attach |
| Workspace | 4 crates | client, server, protocol, utils |
| Terminal Parser | vt100 (start) | Simpler API, migration path to alacritty_terminal |
| Claude Communication | MCP + Sideband | Different use cases benefit from different protocols |
| Session Isolation | CLAUDE_CONFIG_DIR | Preserves shell environment |
| Persistence | Checkpoint + WAL | Balance durability and performance |
| Config Reload | ArcSwap | Lock-free for 60fps rendering |

## Key Documents

| Document | Purpose |
|----------|---------|
| `docs/research/SYNTHESIS.md` | Unified research findings, technology stack |
| `docs/architecture/ARCHITECTURE.md` | System overview, component responsibilities |
| `docs/architecture/CRATE_STRUCTURE.md` | Workspace layout, dependencies |
| `docs/CONTEXT_ENGINEERING_METHODOLOGY.md` | Development process reference |

## Technology Stack

From research consensus:
- **PTY**: portable-pty 0.9
- **Parser**: vt100 0.15
- **TUI**: ratatui 0.29 + crossterm 0.28
- **Async**: tokio 1.x
- **Persistence**: okaywal (WAL) + bincode (serialization)
- **Config**: notify + arc_swap

---

## Session Log

*Track what's done, what's blocked, what changed.*

### 2026-01-07 (Stage 5 - Feature Enumeration)
- Identified 14 features from architecture documents
- Launched parallel planning agents in 4 batches (4 agents each)
- Each agent produced detailed implementation plan:
  - Files to create
  - Data structures and types
  - Key functions/methods
  - Dependencies
  - Testing approach
- Established implementation order based on dependency analysis
- Updated HANDOFF.md with feature inventory

### 2026-01-07 (Stage 4 Completion)
- Created `docs/architecture/` directory structure
- Wrote 5 architecture documents:
  - ARCHITECTURE.md: Client-server model, data flow diagrams
  - CRATE_STRUCTURE.md: Four-crate workspace layout
  - CLAUDE_INTEGRATION.md: Detection methods, session isolation
  - PERSISTENCE.md: Hybrid checkpoint + WAL strategy
  - CONFIGURATION.md: Hot-reload with ArcSwap
- Created 3 ADRs for contested decisions
- Added MIT license
- Initial commit to repository
