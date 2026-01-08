//! Path utilities for XDG directories

use directories::ProjectDirs;
use std::path::PathBuf;

/// Get the Unix socket path for client-server communication
pub fn socket_path() -> PathBuf {
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::temp_dir());
    runtime_dir.join("ccmux.sock")
}

/// Get the configuration directory
pub fn config_dir() -> PathBuf {
    ProjectDirs::from("", "", "ccmux")
        .map(|p| p.config_dir().to_path_buf())
        .unwrap_or_else(|| {
            dirs_fallback().join("config")
        })
}

/// Get the state directory for persistence
pub fn state_dir() -> PathBuf {
    ProjectDirs::from("", "", "ccmux")
        .map(|p| p.data_local_dir().to_path_buf())
        .unwrap_or_else(|| {
            dirs_fallback().join("state")
        })
}

fn dirs_fallback() -> PathBuf {
    std::env::var("HOME")
        .map(|h| PathBuf::from(h).join(".ccmux"))
        .unwrap_or_else(|_| PathBuf::from("/tmp/ccmux"))
}
