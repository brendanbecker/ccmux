//! Orchestration module for worktree-aware session management
//!
//! Provides detection and tracking of git worktrees for coordinating
//! parallel development workflows.

mod worktree;

pub use worktree::{WorktreeDetector, WorktreeInfo};
