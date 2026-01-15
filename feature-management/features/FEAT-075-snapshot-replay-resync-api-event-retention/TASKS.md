# Tasks

## Section 1: Protocol changes
- [x] Add GetEventsSince request/response messages to ccmux-protocol.
- [x] Add snapshot response payload and commit_seq metadata.
- [x] Update ADR-004/ADR-005 references in protocol docs if needed.

## Section 2: Server retention and replay
- [x] Add commit_seq tracking to event publication pipeline.
- [x] Implement replay buffer with retention window and pruning.
- [x] Add snapshot generation and fallback path.

## Section 3: Client resync flow
- [x] Track last_seen_commit_seq in client state.
- [x] Implement gap detection and GetEventsSince call.
- [x] Apply snapshot + replay to converge client state.
