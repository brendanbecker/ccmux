//! Git worktree detection and management
//!
//! Provides utilities for detecting and working with git worktrees.

use std::path::{Path, PathBuf};
use std::process::Command;

/// Information about a git worktree
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorktreeInfo {
    /// Absolute path to the worktree
    pub path: PathBuf,
    /// Branch name (if any)
    pub branch: Option<String>,
    /// Whether this is the main worktree
    pub is_main: bool,
}

/// Detects and provides information about git worktrees
pub struct WorktreeDetector;

impl WorktreeDetector {
    /// Get the worktree root for a given path
    ///
    /// Returns the root directory of the worktree containing the given path,
    /// or None if the path is not inside a git repository/worktree.
    pub fn get_worktree_root(path: &Path) -> Option<PathBuf> {
        let output = Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .current_dir(path)
            .output()
            .ok()?;

        if output.status.success() {
            let root = String::from_utf8_lossy(&output.stdout)
                .trim()
                .to_string();
            Some(PathBuf::from(root))
        } else {
            None
        }
    }

    /// Get the main repository root (the parent repo for worktrees)
    ///
    /// For the main worktree, this returns the same as get_worktree_root.
    /// For linked worktrees, this returns the main repository's root.
    pub fn get_main_repo_root(path: &Path) -> Option<PathBuf> {
        // First get the worktree root
        let worktree_root = Self::get_worktree_root(path)?;

        // Check the git common dir to find the main repo
        let output = Command::new("git")
            .args(["rev-parse", "--git-common-dir"])
            .current_dir(&worktree_root)
            .output()
            .ok()?;

        if output.status.success() {
            let common_dir = String::from_utf8_lossy(&output.stdout)
                .trim()
                .to_string();

            // If common dir ends with .git, parent is the main repo
            let common_path = PathBuf::from(&common_dir);
            if common_dir.ends_with(".git") {
                common_path.parent().map(|p| p.to_path_buf())
            } else {
                // For worktrees, common dir points to main repo's .git
                // e.g., /path/to/main/.git
                common_path.parent().map(|p| p.to_path_buf())
            }
        } else {
            None
        }
    }

    /// List all worktrees for the repository at the given path
    pub fn list_worktrees(path: &Path) -> Vec<WorktreeInfo> {
        let output = match Command::new("git")
            .args(["worktree", "list", "--porcelain"])
            .current_dir(path)
            .output()
        {
            Ok(o) if o.status.success() => o,
            _ => return vec![],
        };

        let stdout = String::from_utf8_lossy(&output.stdout);
        Self::parse_worktree_list(&stdout)
    }

    /// Parse the porcelain output of `git worktree list`
    fn parse_worktree_list(output: &str) -> Vec<WorktreeInfo> {
        let mut worktrees = Vec::new();
        let mut current_path: Option<PathBuf> = None;
        let mut current_branch: Option<String> = None;
        let mut is_first = true;

        for line in output.lines() {
            if let Some(worktree_path) = line.strip_prefix("worktree ") {
                // Save previous worktree if we have one
                if let Some(path) = current_path.take() {
                    worktrees.push(WorktreeInfo {
                        path,
                        branch: current_branch.take(),
                        is_main: is_first,
                    });
                    is_first = false;
                }
                current_path = Some(PathBuf::from(worktree_path));
            } else if let Some(branch_ref) = line.strip_prefix("branch ") {
                // Branch ref like "refs/heads/main"
                current_branch = branch_ref
                    .strip_prefix("refs/heads/")
                    .map(|s| s.to_string())
                    .or_else(|| Some(branch_ref.to_string()));
            } else if line.starts_with("HEAD ") {
                // Detached HEAD - no branch
            } else if line.is_empty() {
                // End of worktree entry - will be handled on next "worktree" line
            }
        }

        // Don't forget the last worktree
        if let Some(path) = current_path {
            worktrees.push(WorktreeInfo {
                path,
                branch: current_branch,
                is_main: is_first,
            });
        }

        worktrees
    }

    /// Get worktree info for a specific path
    #[allow(dead_code)]
    pub fn get_worktree_info(path: &Path) -> Option<WorktreeInfo> {
        let worktree_root = Self::get_worktree_root(path)?;
        let worktrees = Self::list_worktrees(&worktree_root);

        worktrees.into_iter().find(|w| w.path == worktree_root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_worktree_list_single() {
        let output = "worktree /path/to/repo\nHEAD abc123\nbranch refs/heads/main\n\n";
        let worktrees = WorktreeDetector::parse_worktree_list(output);

        assert_eq!(worktrees.len(), 1);
        assert_eq!(worktrees[0].path, PathBuf::from("/path/to/repo"));
        assert_eq!(worktrees[0].branch, Some("main".to_string()));
        assert!(worktrees[0].is_main);
    }

    #[test]
    fn test_parse_worktree_list_multiple() {
        let output = "worktree /path/to/repo\nHEAD abc123\nbranch refs/heads/main\n\nworktree /path/to/worktree1\nHEAD def456\nbranch refs/heads/feature-1\n\n";
        let worktrees = WorktreeDetector::parse_worktree_list(output);

        assert_eq!(worktrees.len(), 2);

        assert_eq!(worktrees[0].path, PathBuf::from("/path/to/repo"));
        assert_eq!(worktrees[0].branch, Some("main".to_string()));
        assert!(worktrees[0].is_main);

        assert_eq!(worktrees[1].path, PathBuf::from("/path/to/worktree1"));
        assert_eq!(worktrees[1].branch, Some("feature-1".to_string()));
        assert!(!worktrees[1].is_main);
    }

    #[test]
    fn test_parse_worktree_list_detached_head() {
        let output = "worktree /path/to/repo\nHEAD abc123\ndetached\n\n";
        let worktrees = WorktreeDetector::parse_worktree_list(output);

        assert_eq!(worktrees.len(), 1);
        assert!(worktrees[0].branch.is_none());
    }

    #[test]
    fn test_parse_worktree_list_empty() {
        let output = "";
        let worktrees = WorktreeDetector::parse_worktree_list(output);
        assert!(worktrees.is_empty());
    }

    // Integration test - only runs if we're in a git repo
    #[test]
    fn test_get_worktree_root_current_dir() {
        use std::env;

        let cwd = env::current_dir().unwrap();
        let root = WorktreeDetector::get_worktree_root(&cwd);

        // This test will pass if we're in a git repo
        if root.is_some() {
            assert!(root.unwrap().exists());
        }
    }

    #[test]
    fn test_get_worktree_root_nonexistent() {
        let path = PathBuf::from("/nonexistent/path/that/does/not/exist");
        let root = WorktreeDetector::get_worktree_root(&path);
        assert!(root.is_none());
    }
}
