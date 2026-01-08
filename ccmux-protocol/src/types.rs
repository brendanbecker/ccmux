//! Shared data types for ccmux protocol

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Split direction for creating panes
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SplitDirection {
    Horizontal,
    Vertical,
}

/// Session information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SessionInfo {
    pub id: Uuid,
    pub name: String,
    pub created_at: u64, // Unix timestamp
    pub window_count: usize,
    pub attached_clients: usize,
}

/// Window information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WindowInfo {
    pub id: Uuid,
    pub session_id: Uuid,
    pub name: String,
    pub index: usize,
    pub pane_count: usize,
    pub active_pane_id: Option<Uuid>,
}

/// Pane information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaneInfo {
    pub id: Uuid,
    pub window_id: Uuid,
    pub index: usize,
    pub cols: u16,
    pub rows: u16,
    pub state: PaneState,
    pub title: Option<String>,
    pub cwd: Option<String>,
}

/// Pane state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum PaneState {
    /// Normal shell/process
    #[default]
    Normal,
    /// Claude Code detected
    Claude(ClaudeState),
    /// Process exited
    Exited { code: Option<i32> },
}

/// Claude Code specific state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClaudeState {
    /// Claude session ID if detected
    pub session_id: Option<String>,
    /// Current activity state
    pub activity: ClaudeActivity,
    /// Model being used
    pub model: Option<String>,
    /// Token usage if available
    pub tokens_used: Option<u64>,
}

impl Default for ClaudeState {
    fn default() -> Self {
        Self {
            session_id: None,
            activity: ClaudeActivity::Idle,
            model: None,
            tokens_used: None,
        }
    }
}

/// Claude activity states
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ClaudeActivity {
    /// Waiting for input
    Idle,
    /// Processing/thinking
    Thinking,
    /// Writing code
    Coding,
    /// Executing tools
    ToolUse,
    /// Waiting for user confirmation
    AwaitingConfirmation,
}

/// Terminal dimensions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Dimensions {
    pub cols: u16,
    pub rows: u16,
}

impl Dimensions {
    pub fn new(cols: u16, rows: u16) -> Self {
        Self { cols, rows }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== SplitDirection Tests ====================

    #[test]
    fn test_split_direction_horizontal() {
        let dir = SplitDirection::Horizontal;
        assert_eq!(dir, SplitDirection::Horizontal);
        assert_ne!(dir, SplitDirection::Vertical);
    }

    #[test]
    fn test_split_direction_vertical() {
        let dir = SplitDirection::Vertical;
        assert_eq!(dir, SplitDirection::Vertical);
        assert_ne!(dir, SplitDirection::Horizontal);
    }

    #[test]
    fn test_split_direction_clone() {
        let dir = SplitDirection::Horizontal;
        let cloned = dir.clone();
        assert_eq!(dir, cloned);
    }

    #[test]
    fn test_split_direction_copy() {
        let dir = SplitDirection::Vertical;
        let copied = dir; // Copy semantics
        assert_eq!(dir, copied);
    }

    #[test]
    fn test_split_direction_debug() {
        assert_eq!(format!("{:?}", SplitDirection::Horizontal), "Horizontal");
        assert_eq!(format!("{:?}", SplitDirection::Vertical), "Vertical");
    }

    // ==================== Dimensions Tests ====================

    #[test]
    fn test_dimensions_new() {
        let dims = Dimensions::new(80, 24);
        assert_eq!(dims.cols, 80);
        assert_eq!(dims.rows, 24);
    }

    #[test]
    fn test_dimensions_equality() {
        let dims1 = Dimensions::new(80, 24);
        let dims2 = Dimensions::new(80, 24);
        let dims3 = Dimensions::new(120, 40);

        assert_eq!(dims1, dims2);
        assert_ne!(dims1, dims3);
    }

    #[test]
    fn test_dimensions_clone_copy() {
        let dims = Dimensions::new(100, 50);
        let cloned = dims.clone();
        let copied = dims; // Copy

        assert_eq!(dims, cloned);
        assert_eq!(dims, copied);
    }

    #[test]
    fn test_dimensions_debug() {
        let dims = Dimensions::new(80, 24);
        let debug = format!("{:?}", dims);
        assert!(debug.contains("80"));
        assert!(debug.contains("24"));
    }

    #[test]
    fn test_dimensions_zero() {
        let dims = Dimensions::new(0, 0);
        assert_eq!(dims.cols, 0);
        assert_eq!(dims.rows, 0);
    }

    #[test]
    fn test_dimensions_max_values() {
        let dims = Dimensions::new(u16::MAX, u16::MAX);
        assert_eq!(dims.cols, u16::MAX);
        assert_eq!(dims.rows, u16::MAX);
    }

    // ==================== ClaudeActivity Tests ====================

    #[test]
    fn test_claude_activity_all_variants() {
        let activities = [
            ClaudeActivity::Idle,
            ClaudeActivity::Thinking,
            ClaudeActivity::Coding,
            ClaudeActivity::ToolUse,
            ClaudeActivity::AwaitingConfirmation,
        ];

        assert_eq!(activities.len(), 5);

        // All should be unique
        for (i, a) in activities.iter().enumerate() {
            for (j, b) in activities.iter().enumerate() {
                if i == j {
                    assert_eq!(a, b);
                } else {
                    assert_ne!(a, b);
                }
            }
        }
    }

    #[test]
    fn test_claude_activity_clone() {
        let activity = ClaudeActivity::Thinking;
        let cloned = activity.clone();
        assert_eq!(activity, cloned);
    }

    #[test]
    fn test_claude_activity_debug() {
        assert_eq!(format!("{:?}", ClaudeActivity::Idle), "Idle");
        assert_eq!(format!("{:?}", ClaudeActivity::Thinking), "Thinking");
        assert_eq!(format!("{:?}", ClaudeActivity::Coding), "Coding");
        assert_eq!(format!("{:?}", ClaudeActivity::ToolUse), "ToolUse");
        assert_eq!(
            format!("{:?}", ClaudeActivity::AwaitingConfirmation),
            "AwaitingConfirmation"
        );
    }

    // ==================== ClaudeState Tests ====================

    #[test]
    fn test_claude_state_default() {
        let state = ClaudeState::default();

        assert!(state.session_id.is_none());
        assert_eq!(state.activity, ClaudeActivity::Idle);
        assert!(state.model.is_none());
        assert!(state.tokens_used.is_none());
    }

    #[test]
    fn test_claude_state_with_all_fields() {
        let state = ClaudeState {
            session_id: Some("session-123".to_string()),
            activity: ClaudeActivity::Coding,
            model: Some("claude-3-opus".to_string()),
            tokens_used: Some(5000),
        };

        assert_eq!(state.session_id, Some("session-123".to_string()));
        assert_eq!(state.activity, ClaudeActivity::Coding);
        assert_eq!(state.model, Some("claude-3-opus".to_string()));
        assert_eq!(state.tokens_used, Some(5000));
    }

    #[test]
    fn test_claude_state_clone() {
        let state = ClaudeState {
            session_id: Some("test".to_string()),
            activity: ClaudeActivity::ToolUse,
            model: Some("claude-3-sonnet".to_string()),
            tokens_used: Some(1000),
        };

        let cloned = state.clone();
        assert_eq!(state, cloned);
    }

    #[test]
    fn test_claude_state_equality() {
        let state1 = ClaudeState::default();
        let state2 = ClaudeState::default();
        let state3 = ClaudeState {
            session_id: Some("x".to_string()),
            ..ClaudeState::default()
        };

        assert_eq!(state1, state2);
        assert_ne!(state1, state3);
    }

    #[test]
    fn test_claude_state_debug() {
        let state = ClaudeState::default();
        let debug = format!("{:?}", state);
        assert!(debug.contains("ClaudeState"));
        assert!(debug.contains("Idle"));
    }

    // ==================== PaneState Tests ====================

    #[test]
    fn test_pane_state_default() {
        let state = PaneState::default();
        assert_eq!(state, PaneState::Normal);
    }

    #[test]
    fn test_pane_state_normal() {
        let state = PaneState::Normal;
        assert_eq!(state.clone(), PaneState::Normal);
    }

    #[test]
    fn test_pane_state_claude() {
        let claude_state = ClaudeState::default();
        let state = PaneState::Claude(claude_state.clone());

        if let PaneState::Claude(cs) = &state {
            assert_eq!(*cs, claude_state);
        } else {
            panic!("Expected Claude variant");
        }
    }

    #[test]
    fn test_pane_state_exited_with_code() {
        let state = PaneState::Exited { code: Some(0) };

        if let PaneState::Exited { code } = state {
            assert_eq!(code, Some(0));
        }
    }

    #[test]
    fn test_pane_state_exited_without_code() {
        let state = PaneState::Exited { code: None };

        if let PaneState::Exited { code } = state {
            assert!(code.is_none());
        }
    }

    #[test]
    fn test_pane_state_exited_error_code() {
        let state = PaneState::Exited { code: Some(1) };

        if let PaneState::Exited { code } = state {
            assert_eq!(code, Some(1));
        }
    }

    #[test]
    fn test_pane_state_exited_signal() {
        // Simulating killed by signal (128 + signal number)
        let state = PaneState::Exited { code: Some(137) }; // SIGKILL

        if let PaneState::Exited { code } = state {
            assert_eq!(code, Some(137));
        }
    }

    #[test]
    fn test_pane_state_equality() {
        let normal1 = PaneState::Normal;
        let normal2 = PaneState::Normal;
        let claude = PaneState::Claude(ClaudeState::default());
        let exited = PaneState::Exited { code: Some(0) };

        assert_eq!(normal1, normal2);
        assert_ne!(normal1, claude);
        assert_ne!(normal1, exited);
        assert_ne!(claude, exited);
    }

    #[test]
    fn test_pane_state_clone() {
        let states = [
            PaneState::Normal,
            PaneState::Claude(ClaudeState::default()),
            PaneState::Exited { code: Some(42) },
        ];

        for state in states {
            let cloned = state.clone();
            assert_eq!(state, cloned);
        }
    }

    // ==================== PaneInfo Tests ====================

    #[test]
    fn test_pane_info_minimal() {
        let pane = PaneInfo {
            id: Uuid::new_v4(),
            window_id: Uuid::new_v4(),
            index: 0,
            cols: 80,
            rows: 24,
            state: PaneState::Normal,
            title: None,
            cwd: None,
        };

        assert_eq!(pane.index, 0);
        assert_eq!(pane.cols, 80);
        assert_eq!(pane.rows, 24);
        assert!(pane.title.is_none());
        assert!(pane.cwd.is_none());
    }

    #[test]
    fn test_pane_info_full() {
        let id = Uuid::new_v4();
        let window_id = Uuid::new_v4();

        let pane = PaneInfo {
            id,
            window_id,
            index: 2,
            cols: 120,
            rows: 40,
            state: PaneState::Claude(ClaudeState::default()),
            title: Some("vim".to_string()),
            cwd: Some("/home/user/project".to_string()),
        };

        assert_eq!(pane.id, id);
        assert_eq!(pane.window_id, window_id);
        assert_eq!(pane.index, 2);
        assert_eq!(pane.title, Some("vim".to_string()));
        assert_eq!(pane.cwd, Some("/home/user/project".to_string()));
    }

    #[test]
    fn test_pane_info_clone() {
        let pane = PaneInfo {
            id: Uuid::new_v4(),
            window_id: Uuid::new_v4(),
            index: 0,
            cols: 80,
            rows: 24,
            state: PaneState::Normal,
            title: Some("bash".to_string()),
            cwd: Some("/tmp".to_string()),
        };

        let cloned = pane.clone();
        assert_eq!(pane, cloned);
    }

    #[test]
    fn test_pane_info_equality() {
        let id = Uuid::new_v4();
        let window_id = Uuid::new_v4();

        let pane1 = PaneInfo {
            id,
            window_id,
            index: 0,
            cols: 80,
            rows: 24,
            state: PaneState::Normal,
            title: None,
            cwd: None,
        };

        let pane2 = PaneInfo {
            id,
            window_id,
            index: 0,
            cols: 80,
            rows: 24,
            state: PaneState::Normal,
            title: None,
            cwd: None,
        };

        let pane3 = PaneInfo {
            id,
            window_id,
            index: 1, // Different index
            cols: 80,
            rows: 24,
            state: PaneState::Normal,
            title: None,
            cwd: None,
        };

        assert_eq!(pane1, pane2);
        assert_ne!(pane1, pane3);
    }

    // ==================== WindowInfo Tests ====================

    #[test]
    fn test_window_info_minimal() {
        let window = WindowInfo {
            id: Uuid::new_v4(),
            session_id: Uuid::new_v4(),
            name: "main".to_string(),
            index: 0,
            pane_count: 1,
            active_pane_id: None,
        };

        assert_eq!(window.name, "main");
        assert_eq!(window.index, 0);
        assert_eq!(window.pane_count, 1);
        assert!(window.active_pane_id.is_none());
    }

    #[test]
    fn test_window_info_with_active_pane() {
        let pane_id = Uuid::new_v4();

        let window = WindowInfo {
            id: Uuid::new_v4(),
            session_id: Uuid::new_v4(),
            name: "editor".to_string(),
            index: 1,
            pane_count: 3,
            active_pane_id: Some(pane_id),
        };

        assert_eq!(window.active_pane_id, Some(pane_id));
        assert_eq!(window.pane_count, 3);
    }

    #[test]
    fn test_window_info_clone() {
        let window = WindowInfo {
            id: Uuid::new_v4(),
            session_id: Uuid::new_v4(),
            name: "test".to_string(),
            index: 0,
            pane_count: 2,
            active_pane_id: Some(Uuid::new_v4()),
        };

        let cloned = window.clone();
        assert_eq!(window, cloned);
    }

    #[test]
    fn test_window_info_equality() {
        let id = Uuid::new_v4();
        let session_id = Uuid::new_v4();

        let window1 = WindowInfo {
            id,
            session_id,
            name: "main".to_string(),
            index: 0,
            pane_count: 1,
            active_pane_id: None,
        };

        let window2 = WindowInfo {
            id,
            session_id,
            name: "main".to_string(),
            index: 0,
            pane_count: 1,
            active_pane_id: None,
        };

        let window3 = WindowInfo {
            id,
            session_id,
            name: "other".to_string(), // Different name
            index: 0,
            pane_count: 1,
            active_pane_id: None,
        };

        assert_eq!(window1, window2);
        assert_ne!(window1, window3);
    }

    // ==================== SessionInfo Tests ====================

    #[test]
    fn test_session_info_creation() {
        let id = Uuid::new_v4();

        let session = SessionInfo {
            id,
            name: "my-session".to_string(),
            created_at: 1704067200, // 2024-01-01 00:00:00 UTC
            window_count: 2,
            attached_clients: 1,
        };

        assert_eq!(session.id, id);
        assert_eq!(session.name, "my-session");
        assert_eq!(session.created_at, 1704067200);
        assert_eq!(session.window_count, 2);
        assert_eq!(session.attached_clients, 1);
    }

    #[test]
    fn test_session_info_no_clients() {
        let session = SessionInfo {
            id: Uuid::new_v4(),
            name: "detached".to_string(),
            created_at: 0,
            window_count: 1,
            attached_clients: 0,
        };

        assert_eq!(session.attached_clients, 0);
    }

    #[test]
    fn test_session_info_multiple_clients() {
        let session = SessionInfo {
            id: Uuid::new_v4(),
            name: "shared".to_string(),
            created_at: 0,
            window_count: 1,
            attached_clients: 5,
        };

        assert_eq!(session.attached_clients, 5);
    }

    #[test]
    fn test_session_info_clone() {
        let session = SessionInfo {
            id: Uuid::new_v4(),
            name: "test".to_string(),
            created_at: 12345,
            window_count: 3,
            attached_clients: 2,
        };

        let cloned = session.clone();
        assert_eq!(session, cloned);
    }

    #[test]
    fn test_session_info_equality() {
        let id = Uuid::new_v4();

        let session1 = SessionInfo {
            id,
            name: "test".to_string(),
            created_at: 1000,
            window_count: 1,
            attached_clients: 0,
        };

        let session2 = SessionInfo {
            id,
            name: "test".to_string(),
            created_at: 1000,
            window_count: 1,
            attached_clients: 0,
        };

        let session3 = SessionInfo {
            id,
            name: "different".to_string(),
            created_at: 1000,
            window_count: 1,
            attached_clients: 0,
        };

        assert_eq!(session1, session2);
        assert_ne!(session1, session3);
    }

    #[test]
    fn test_session_info_debug() {
        let session = SessionInfo {
            id: Uuid::nil(),
            name: "debug-test".to_string(),
            created_at: 0,
            window_count: 0,
            attached_clients: 0,
        };

        let debug = format!("{:?}", session);
        assert!(debug.contains("SessionInfo"));
        assert!(debug.contains("debug-test"));
    }

    // ==================== Serialization Round-trip Tests ====================

    #[test]
    fn test_split_direction_serde() {
        let dir = SplitDirection::Horizontal;
        let serialized = bincode::serialize(&dir).unwrap();
        let deserialized: SplitDirection = bincode::deserialize(&serialized).unwrap();
        assert_eq!(dir, deserialized);
    }

    #[test]
    fn test_dimensions_serde() {
        let dims = Dimensions::new(80, 24);
        let serialized = bincode::serialize(&dims).unwrap();
        let deserialized: Dimensions = bincode::deserialize(&serialized).unwrap();
        assert_eq!(dims, deserialized);
    }

    #[test]
    fn test_claude_activity_serde() {
        for activity in [
            ClaudeActivity::Idle,
            ClaudeActivity::Thinking,
            ClaudeActivity::Coding,
            ClaudeActivity::ToolUse,
            ClaudeActivity::AwaitingConfirmation,
        ] {
            let serialized = bincode::serialize(&activity).unwrap();
            let deserialized: ClaudeActivity = bincode::deserialize(&serialized).unwrap();
            assert_eq!(activity, deserialized);
        }
    }

    #[test]
    fn test_claude_state_serde() {
        let state = ClaudeState {
            session_id: Some("abc".to_string()),
            activity: ClaudeActivity::Coding,
            model: Some("claude-3".to_string()),
            tokens_used: Some(100),
        };

        let serialized = bincode::serialize(&state).unwrap();
        let deserialized: ClaudeState = bincode::deserialize(&serialized).unwrap();
        assert_eq!(state, deserialized);
    }

    #[test]
    fn test_pane_state_serde() {
        let states = [
            PaneState::Normal,
            PaneState::Claude(ClaudeState::default()),
            PaneState::Exited { code: Some(0) },
            PaneState::Exited { code: None },
        ];

        for state in states {
            let serialized = bincode::serialize(&state).unwrap();
            let deserialized: PaneState = bincode::deserialize(&serialized).unwrap();
            assert_eq!(state, deserialized);
        }
    }

    #[test]
    fn test_pane_info_serde() {
        let pane = PaneInfo {
            id: Uuid::new_v4(),
            window_id: Uuid::new_v4(),
            index: 0,
            cols: 80,
            rows: 24,
            state: PaneState::Normal,
            title: Some("test".to_string()),
            cwd: Some("/home".to_string()),
        };

        let serialized = bincode::serialize(&pane).unwrap();
        let deserialized: PaneInfo = bincode::deserialize(&serialized).unwrap();
        assert_eq!(pane, deserialized);
    }

    #[test]
    fn test_window_info_serde() {
        let window = WindowInfo {
            id: Uuid::new_v4(),
            session_id: Uuid::new_v4(),
            name: "main".to_string(),
            index: 0,
            pane_count: 2,
            active_pane_id: Some(Uuid::new_v4()),
        };

        let serialized = bincode::serialize(&window).unwrap();
        let deserialized: WindowInfo = bincode::deserialize(&serialized).unwrap();
        assert_eq!(window, deserialized);
    }

    #[test]
    fn test_session_info_serde() {
        let session = SessionInfo {
            id: Uuid::new_v4(),
            name: "test-session".to_string(),
            created_at: 1234567890,
            window_count: 3,
            attached_clients: 1,
        };

        let serialized = bincode::serialize(&session).unwrap();
        let deserialized: SessionInfo = bincode::deserialize(&serialized).unwrap();
        assert_eq!(session, deserialized);
    }
}
