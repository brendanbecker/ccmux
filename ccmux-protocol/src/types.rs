//! Shared data types

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Information about a session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub id: Uuid,
    pub name: String,
}

/// Information about a window
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowInfo {
    pub id: Uuid,
    pub name: String,
}

/// Information about a pane
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaneInfo {
    pub id: Uuid,
    pub state: PaneState,
}

/// State of a pane
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaneState {
    Normal,
    Claude(ClaudeState),
}

/// Claude Code state within a pane
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeState {
    pub session_id: Option<String>,
    pub active: bool,
}
