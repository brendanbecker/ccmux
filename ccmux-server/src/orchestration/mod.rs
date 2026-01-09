//! Orchestration - worktree-aware session coordination
//!
//! Provides detection and tracking of git worktrees for coordinating
//! parallel development workflows.

mod worktree;

#[allow(unused_imports)]
pub use worktree::{WorktreeDetector, WorktreeInfo};
