//! Cross-session orchestration for ccmux
//!
//! This module provides messaging infrastructure for communication between
//! ccmux sessions, particularly between worktree sessions and their orchestrator.

mod router;

pub use router::{MessageReceiver, MessageRouter, MessageSender, RouterError};
