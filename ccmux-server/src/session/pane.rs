// Allow unused fields that are reserved for future use
#![allow(dead_code)]

use std::time::SystemTime;
use uuid::Uuid;
use ccmux_protocol::{ClaudeActivity, ClaudeState, PaneInfo, PaneState};

/// A terminal pane within a window
#[derive(Debug)]
pub struct Pane {
    /// Unique pane identifier
    id: Uuid,
    /// Parent window ID
    window_id: Uuid,
    /// Index within the window
    index: usize,
    /// Terminal dimensions
    cols: u16,
    rows: u16,
    /// Current pane state
    state: PaneState,
    /// Terminal title (from escape sequences)
    title: Option<String>,
    /// Current working directory
    cwd: Option<String>,
    /// When the pane was created
    created_at: SystemTime,
    /// When state last changed
    state_changed_at: SystemTime,
}

impl Pane {
    /// Create a new pane
    pub fn new(window_id: Uuid, index: usize) -> Self {
        let now = SystemTime::now();
        Self {
            id: Uuid::new_v4(),
            window_id,
            index,
            cols: 80,
            rows: 24,
            state: PaneState::Normal,
            title: None,
            cwd: None,
            created_at: now,
            state_changed_at: now,
        }
    }

    /// Get pane ID
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Get window ID
    pub fn window_id(&self) -> Uuid {
        self.window_id
    }

    /// Get pane index
    pub fn index(&self) -> usize {
        self.index
    }

    /// Set pane index
    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    /// Get dimensions
    pub fn dimensions(&self) -> (u16, u16) {
        (self.cols, self.rows)
    }

    /// Resize the pane
    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.cols = cols;
        self.rows = rows;
    }

    /// Get current state
    pub fn state(&self) -> &PaneState {
        &self.state
    }

    /// Set state
    pub fn set_state(&mut self, state: PaneState) {
        self.state = state;
        self.state_changed_at = SystemTime::now();
    }

    /// Check if this is a Claude pane
    pub fn is_claude(&self) -> bool {
        matches!(self.state, PaneState::Claude(_))
    }

    /// Get Claude state if this is a Claude pane
    pub fn claude_state(&self) -> Option<&ClaudeState> {
        match &self.state {
            PaneState::Claude(state) => Some(state),
            _ => None,
        }
    }

    /// Check if pane is awaiting user input (AwaitingConfirmation or Idle state)
    ///
    /// Returns true if:
    /// - This is a Claude pane in AwaitingConfirmation state
    /// - This is a Claude pane in Idle state (also waiting for input)
    pub fn is_awaiting_input(&self) -> bool {
        match &self.state {
            PaneState::Claude(state) => matches!(
                state.activity,
                ClaudeActivity::AwaitingConfirmation | ClaudeActivity::Idle
            ),
            _ => false,
        }
    }

    /// Check specifically if pane is awaiting confirmation (tool use approval, etc.)
    pub fn is_awaiting_confirmation(&self) -> bool {
        match &self.state {
            PaneState::Claude(state) => {
                matches!(state.activity, ClaudeActivity::AwaitingConfirmation)
            }
            _ => false,
        }
    }

    /// Update Claude state
    pub fn set_claude_state(&mut self, state: ClaudeState) {
        self.state = PaneState::Claude(state);
        self.state_changed_at = SystemTime::now();
    }

    /// Get title
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Set title
    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }

    /// Get current working directory
    pub fn cwd(&self) -> Option<&str> {
        self.cwd.as_deref()
    }

    /// Set current working directory
    pub fn set_cwd(&mut self, cwd: Option<String>) {
        self.cwd = cwd;
    }

    /// Convert to protocol PaneInfo
    pub fn to_info(&self) -> PaneInfo {
        PaneInfo {
            id: self.id,
            window_id: self.window_id,
            index: self.index,
            cols: self.cols,
            rows: self.rows,
            state: self.state.clone(),
            title: self.title.clone(),
            cwd: self.cwd.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ccmux_protocol::ClaudeActivity;

    #[test]
    fn test_pane_creation() {
        let window_id = Uuid::new_v4();
        let pane = Pane::new(window_id, 0);

        assert_eq!(pane.window_id(), window_id);
        assert_eq!(pane.index(), 0);
        assert_eq!(pane.dimensions(), (80, 24));
        assert!(!pane.is_claude());
    }

    #[test]
    fn test_pane_resize() {
        let window_id = Uuid::new_v4();
        let mut pane = Pane::new(window_id, 0);

        pane.resize(120, 40);
        assert_eq!(pane.dimensions(), (120, 40));
    }

    #[test]
    fn test_pane_claude_state() {
        let window_id = Uuid::new_v4();
        let mut pane = Pane::new(window_id, 0);

        assert!(!pane.is_claude());

        pane.set_claude_state(ClaudeState::default());
        assert!(pane.is_claude());
        assert!(pane.claude_state().is_some());
    }

    #[test]
    fn test_pane_to_info() {
        let window_id = Uuid::new_v4();
        let pane = Pane::new(window_id, 0);
        let info = pane.to_info();

        assert_eq!(info.id, pane.id());
        assert_eq!(info.window_id, window_id);
    }

    #[test]
    fn test_pane_id_is_unique() {
        let window_id = Uuid::new_v4();
        let pane1 = Pane::new(window_id, 0);
        let pane2 = Pane::new(window_id, 1);

        assert_ne!(pane1.id(), pane2.id());
    }

    #[test]
    fn test_pane_set_index() {
        let window_id = Uuid::new_v4();
        let mut pane = Pane::new(window_id, 0);

        assert_eq!(pane.index(), 0);
        pane.set_index(5);
        assert_eq!(pane.index(), 5);
    }

    #[test]
    fn test_pane_state_getter_setter() {
        let window_id = Uuid::new_v4();
        let mut pane = Pane::new(window_id, 0);

        assert!(matches!(pane.state(), PaneState::Normal));

        pane.set_state(PaneState::Exited { code: Some(0) });
        assert!(matches!(pane.state(), PaneState::Exited { code: Some(0) }));
    }

    #[test]
    fn test_pane_title_getter_setter() {
        let window_id = Uuid::new_v4();
        let mut pane = Pane::new(window_id, 0);

        assert!(pane.title().is_none());

        pane.set_title(Some("my-title".to_string()));
        assert_eq!(pane.title(), Some("my-title"));

        pane.set_title(None);
        assert!(pane.title().is_none());
    }

    #[test]
    fn test_pane_cwd_getter_setter() {
        let window_id = Uuid::new_v4();
        let mut pane = Pane::new(window_id, 0);

        assert!(pane.cwd().is_none());

        pane.set_cwd(Some("/home/user".to_string()));
        assert_eq!(pane.cwd(), Some("/home/user"));

        pane.set_cwd(None);
        assert!(pane.cwd().is_none());
    }

    #[test]
    fn test_pane_claude_state_none_when_not_claude() {
        let window_id = Uuid::new_v4();
        let pane = Pane::new(window_id, 0);

        assert!(pane.claude_state().is_none());
    }

    #[test]
    fn test_pane_claude_state_with_activity() {
        let window_id = Uuid::new_v4();
        let mut pane = Pane::new(window_id, 0);

        let state = ClaudeState {
            session_id: Some("test-session".to_string()),
            activity: ClaudeActivity::Thinking,
            model: Some("claude-3-opus".to_string()),
            tokens_used: Some(5000),
        };

        pane.set_claude_state(state.clone());
        let claude_state = pane.claude_state().unwrap();
        assert_eq!(claude_state.activity, ClaudeActivity::Thinking);
        assert_eq!(claude_state.session_id, Some("test-session".to_string()));
        assert_eq!(claude_state.tokens_used, Some(5000));
    }

    #[test]
    fn test_pane_resize_to_zero() {
        let window_id = Uuid::new_v4();
        let mut pane = Pane::new(window_id, 0);

        pane.resize(0, 0);
        assert_eq!(pane.dimensions(), (0, 0));
    }

    #[test]
    fn test_pane_resize_large_dimensions() {
        let window_id = Uuid::new_v4();
        let mut pane = Pane::new(window_id, 0);

        pane.resize(u16::MAX, u16::MAX);
        assert_eq!(pane.dimensions(), (u16::MAX, u16::MAX));
    }

    #[test]
    fn test_pane_to_info_includes_all_fields() {
        let window_id = Uuid::new_v4();
        let mut pane = Pane::new(window_id, 3);
        pane.resize(100, 50);
        pane.set_title(Some("test-title".to_string()));
        pane.set_cwd(Some("/tmp".to_string()));
        pane.set_state(PaneState::Exited { code: Some(1) });

        let info = pane.to_info();

        assert_eq!(info.id, pane.id());
        assert_eq!(info.window_id, window_id);
        assert_eq!(info.index, 3);
        assert_eq!(info.cols, 100);
        assert_eq!(info.rows, 50);
        assert_eq!(info.title, Some("test-title".to_string()));
        assert_eq!(info.cwd, Some("/tmp".to_string()));
        assert!(matches!(info.state, PaneState::Exited { code: Some(1) }));
    }

    #[test]
    fn test_pane_debug_format() {
        let window_id = Uuid::new_v4();
        let pane = Pane::new(window_id, 0);

        let debug_str = format!("{:?}", pane);
        assert!(debug_str.contains("Pane"));
        assert!(debug_str.contains("cols: 80"));
        assert!(debug_str.contains("rows: 24"));
    }

    #[test]
    fn test_pane_state_transition_exited() {
        let window_id = Uuid::new_v4();
        let mut pane = Pane::new(window_id, 0);

        pane.set_state(PaneState::Exited { code: Some(0) });
        assert!(matches!(pane.state(), PaneState::Exited { code: Some(0) }));
        assert!(!pane.is_claude());
    }

    #[test]
    fn test_pane_state_transition_exited_no_code() {
        let window_id = Uuid::new_v4();
        let mut pane = Pane::new(window_id, 0);

        pane.set_state(PaneState::Exited { code: None });
        assert!(matches!(pane.state(), PaneState::Exited { code: None }));
    }

    #[test]
    fn test_pane_multiple_resizes() {
        let window_id = Uuid::new_v4();
        let mut pane = Pane::new(window_id, 0);

        pane.resize(100, 50);
        assert_eq!(pane.dimensions(), (100, 50));

        pane.resize(200, 100);
        assert_eq!(pane.dimensions(), (200, 100));

        pane.resize(80, 24);
        assert_eq!(pane.dimensions(), (80, 24));
    }

    // ==================== Input-Wait Detection Tests ====================

    #[test]
    fn test_pane_is_awaiting_input_normal_pane() {
        let window_id = Uuid::new_v4();
        let pane = Pane::new(window_id, 0);

        // Normal panes are never awaiting input
        assert!(!pane.is_awaiting_input());
        assert!(!pane.is_awaiting_confirmation());
    }

    #[test]
    fn test_pane_is_awaiting_input_idle_claude() {
        let window_id = Uuid::new_v4();
        let mut pane = Pane::new(window_id, 0);

        pane.set_claude_state(ClaudeState {
            session_id: None,
            activity: ClaudeActivity::Idle,
            model: None,
            tokens_used: None,
        });

        // Idle Claude panes are awaiting input
        assert!(pane.is_awaiting_input());
        assert!(!pane.is_awaiting_confirmation());
    }

    #[test]
    fn test_pane_is_awaiting_input_awaiting_confirmation() {
        let window_id = Uuid::new_v4();
        let mut pane = Pane::new(window_id, 0);

        pane.set_claude_state(ClaudeState {
            session_id: None,
            activity: ClaudeActivity::AwaitingConfirmation,
            model: None,
            tokens_used: None,
        });

        // AwaitingConfirmation Claude panes are awaiting input
        assert!(pane.is_awaiting_input());
        assert!(pane.is_awaiting_confirmation());
    }

    #[test]
    fn test_pane_is_awaiting_input_thinking() {
        let window_id = Uuid::new_v4();
        let mut pane = Pane::new(window_id, 0);

        pane.set_claude_state(ClaudeState {
            session_id: None,
            activity: ClaudeActivity::Thinking,
            model: None,
            tokens_used: None,
        });

        // Thinking Claude panes are NOT awaiting input
        assert!(!pane.is_awaiting_input());
        assert!(!pane.is_awaiting_confirmation());
    }

    #[test]
    fn test_pane_is_awaiting_input_coding() {
        let window_id = Uuid::new_v4();
        let mut pane = Pane::new(window_id, 0);

        pane.set_claude_state(ClaudeState {
            session_id: None,
            activity: ClaudeActivity::Coding,
            model: None,
            tokens_used: None,
        });

        // Coding Claude panes are NOT awaiting input
        assert!(!pane.is_awaiting_input());
        assert!(!pane.is_awaiting_confirmation());
    }

    #[test]
    fn test_pane_is_awaiting_input_tool_use() {
        let window_id = Uuid::new_v4();
        let mut pane = Pane::new(window_id, 0);

        pane.set_claude_state(ClaudeState {
            session_id: None,
            activity: ClaudeActivity::ToolUse,
            model: None,
            tokens_used: None,
        });

        // ToolUse Claude panes are NOT awaiting input
        assert!(!pane.is_awaiting_input());
        assert!(!pane.is_awaiting_confirmation());
    }

    #[test]
    fn test_pane_is_awaiting_input_exited_pane() {
        let window_id = Uuid::new_v4();
        let mut pane = Pane::new(window_id, 0);

        pane.set_state(PaneState::Exited { code: Some(0) });

        // Exited panes are never awaiting input
        assert!(!pane.is_awaiting_input());
        assert!(!pane.is_awaiting_confirmation());
    }

    #[test]
    fn test_pane_is_awaiting_input_state_transitions() {
        let window_id = Uuid::new_v4();
        let mut pane = Pane::new(window_id, 0);

        // Start as Claude Idle - awaiting input
        pane.set_claude_state(ClaudeState {
            session_id: None,
            activity: ClaudeActivity::Idle,
            model: None,
            tokens_used: None,
        });
        assert!(pane.is_awaiting_input());

        // Transition to Thinking - NOT awaiting input
        pane.set_claude_state(ClaudeState {
            session_id: None,
            activity: ClaudeActivity::Thinking,
            model: None,
            tokens_used: None,
        });
        assert!(!pane.is_awaiting_input());

        // Transition to AwaitingConfirmation - awaiting input
        pane.set_claude_state(ClaudeState {
            session_id: None,
            activity: ClaudeActivity::AwaitingConfirmation,
            model: None,
            tokens_used: None,
        });
        assert!(pane.is_awaiting_input());
        assert!(pane.is_awaiting_confirmation());

        // Back to Idle - awaiting input
        pane.set_claude_state(ClaudeState {
            session_id: None,
            activity: ClaudeActivity::Idle,
            model: None,
            tokens_used: None,
        });
        assert!(pane.is_awaiting_input());
        assert!(!pane.is_awaiting_confirmation());
    }
}
