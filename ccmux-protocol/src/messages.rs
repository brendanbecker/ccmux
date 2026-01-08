//! Client-server message types

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::*;

/// Messages sent from client to server
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClientMessage {
    /// Initial connection handshake
    Connect {
        client_id: Uuid,
        protocol_version: u32,
    },

    /// Request list of sessions
    ListSessions,

    /// Create a new session
    CreateSession { name: String },

    /// Attach to existing session
    AttachSession { session_id: Uuid },

    /// Create new window in session
    CreateWindow {
        session_id: Uuid,
        name: Option<String>,
    },

    /// Create new pane (split)
    CreatePane {
        window_id: Uuid,
        direction: SplitDirection,
    },

    /// Send input to pane
    Input { pane_id: Uuid, data: Vec<u8> },

    /// Resize pane
    Resize { pane_id: Uuid, cols: u16, rows: u16 },

    /// Close pane
    ClosePane { pane_id: Uuid },

    /// Select/focus pane
    SelectPane { pane_id: Uuid },

    /// Detach from session (keep session running)
    Detach,

    /// Request full state sync
    Sync,

    /// Ping for keepalive
    Ping,
}

/// Messages sent from server to client
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServerMessage {
    /// Connection accepted
    Connected {
        server_version: String,
        protocol_version: u32,
    },

    /// List of available sessions
    SessionList { sessions: Vec<SessionInfo> },

    /// Session created
    SessionCreated { session: SessionInfo },

    /// Attached to session - full state
    Attached {
        session: SessionInfo,
        windows: Vec<WindowInfo>,
        panes: Vec<PaneInfo>,
    },

    /// Window created
    WindowCreated { window: WindowInfo },

    /// Pane created
    PaneCreated { pane: PaneInfo },

    /// Pane output data
    Output { pane_id: Uuid, data: Vec<u8> },

    /// Pane state changed
    PaneStateChanged { pane_id: Uuid, state: PaneState },

    /// Claude state update (for Claude-detected panes)
    ClaudeStateChanged { pane_id: Uuid, state: ClaudeState },

    /// Pane closed
    PaneClosed {
        pane_id: Uuid,
        exit_code: Option<i32>,
    },

    /// Window closed
    WindowClosed { window_id: Uuid },

    /// Session ended
    SessionEnded { session_id: Uuid },

    /// Error response
    Error { code: ErrorCode, message: String },

    /// Pong response to ping
    Pong,
}

/// Error codes for protocol errors
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorCode {
    SessionNotFound,
    WindowNotFound,
    PaneNotFound,
    InvalidOperation,
    ProtocolMismatch,
    InternalError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_message_connect() {
        let client_id = Uuid::new_v4();
        let msg = ClientMessage::Connect {
            client_id,
            protocol_version: 1,
        };

        // Test clone
        let cloned = msg.clone();
        assert_eq!(msg, cloned);

        // Test debug
        let debug = format!("{:?}", msg);
        assert!(debug.contains("Connect"));
        assert!(debug.contains(&client_id.to_string()));
    }

    #[test]
    fn test_client_message_list_sessions() {
        let msg = ClientMessage::ListSessions;
        assert_eq!(msg.clone(), ClientMessage::ListSessions);
    }

    #[test]
    fn test_client_message_create_session() {
        let msg = ClientMessage::CreateSession {
            name: "test-session".to_string(),
        };
        if let ClientMessage::CreateSession { name } = &msg {
            assert_eq!(name, "test-session");
        } else {
            panic!("Wrong variant");
        }
    }

    #[test]
    fn test_client_message_attach_session() {
        let session_id = Uuid::new_v4();
        let msg = ClientMessage::AttachSession { session_id };
        if let ClientMessage::AttachSession { session_id: id } = msg {
            assert_eq!(id, session_id);
        }
    }

    #[test]
    fn test_client_message_create_window() {
        let session_id = Uuid::new_v4();

        // With name
        let msg = ClientMessage::CreateWindow {
            session_id,
            name: Some("main".to_string()),
        };
        if let ClientMessage::CreateWindow {
            session_id: sid,
            name,
        } = msg
        {
            assert_eq!(sid, session_id);
            assert_eq!(name, Some("main".to_string()));
        }

        // Without name
        let msg2 = ClientMessage::CreateWindow {
            session_id,
            name: None,
        };
        if let ClientMessage::CreateWindow { name, .. } = msg2 {
            assert!(name.is_none());
        }
    }

    #[test]
    fn test_client_message_create_pane() {
        let window_id = Uuid::new_v4();

        let msg_h = ClientMessage::CreatePane {
            window_id,
            direction: SplitDirection::Horizontal,
        };
        let msg_v = ClientMessage::CreatePane {
            window_id,
            direction: SplitDirection::Vertical,
        };

        assert_ne!(msg_h, msg_v);
    }

    #[test]
    fn test_client_message_input() {
        let pane_id = Uuid::new_v4();
        let data = vec![0x1b, 0x5b, 0x41]; // Up arrow escape sequence

        let msg = ClientMessage::Input {
            pane_id,
            data: data.clone(),
        };

        if let ClientMessage::Input {
            pane_id: pid,
            data: d,
        } = msg
        {
            assert_eq!(pid, pane_id);
            assert_eq!(d, data);
        }
    }

    #[test]
    fn test_client_message_resize() {
        let pane_id = Uuid::new_v4();
        let msg = ClientMessage::Resize {
            pane_id,
            cols: 120,
            rows: 40,
        };

        if let ClientMessage::Resize { cols, rows, .. } = msg {
            assert_eq!(cols, 120);
            assert_eq!(rows, 40);
        }
    }

    #[test]
    fn test_client_message_close_pane() {
        let pane_id = Uuid::new_v4();
        let msg = ClientMessage::ClosePane { pane_id };
        if let ClientMessage::ClosePane { pane_id: pid } = msg {
            assert_eq!(pid, pane_id);
        }
    }

    #[test]
    fn test_client_message_select_pane() {
        let pane_id = Uuid::new_v4();
        let msg = ClientMessage::SelectPane { pane_id };
        if let ClientMessage::SelectPane { pane_id: pid } = msg {
            assert_eq!(pid, pane_id);
        }
    }

    #[test]
    fn test_client_message_simple_variants() {
        assert_eq!(ClientMessage::Detach.clone(), ClientMessage::Detach);
        assert_eq!(ClientMessage::Sync.clone(), ClientMessage::Sync);
        assert_eq!(ClientMessage::Ping.clone(), ClientMessage::Ping);

        // All should be different
        assert_ne!(ClientMessage::Detach, ClientMessage::Sync);
        assert_ne!(ClientMessage::Sync, ClientMessage::Ping);
        assert_ne!(ClientMessage::Ping, ClientMessage::Detach);
    }

    #[test]
    fn test_server_message_connected() {
        let msg = ServerMessage::Connected {
            server_version: "1.0.0".to_string(),
            protocol_version: 1,
        };

        if let ServerMessage::Connected {
            server_version,
            protocol_version,
        } = msg.clone()
        {
            assert_eq!(server_version, "1.0.0");
            assert_eq!(protocol_version, 1);
        }

        assert_eq!(msg.clone(), msg);
    }

    #[test]
    fn test_server_message_session_list() {
        let sessions = vec![
            SessionInfo {
                id: Uuid::new_v4(),
                name: "session1".to_string(),
                created_at: 1000,
                window_count: 2,
                attached_clients: 1,
            },
            SessionInfo {
                id: Uuid::new_v4(),
                name: "session2".to_string(),
                created_at: 2000,
                window_count: 1,
                attached_clients: 0,
            },
        ];

        let msg = ServerMessage::SessionList {
            sessions: sessions.clone(),
        };

        if let ServerMessage::SessionList { sessions: s } = msg {
            assert_eq!(s.len(), 2);
            assert_eq!(s[0].name, "session1");
            assert_eq!(s[1].name, "session2");
        }
    }

    #[test]
    fn test_server_message_session_created() {
        let session = SessionInfo {
            id: Uuid::new_v4(),
            name: "new-session".to_string(),
            created_at: 12345,
            window_count: 0,
            attached_clients: 1,
        };

        let msg = ServerMessage::SessionCreated {
            session: session.clone(),
        };

        if let ServerMessage::SessionCreated { session: s } = msg {
            assert_eq!(s.name, "new-session");
            assert_eq!(s.window_count, 0);
        }
    }

    #[test]
    fn test_server_message_attached() {
        let session_id = Uuid::new_v4();
        let window_id = Uuid::new_v4();
        let pane_id = Uuid::new_v4();

        let msg = ServerMessage::Attached {
            session: SessionInfo {
                id: session_id,
                name: "test".to_string(),
                created_at: 0,
                window_count: 1,
                attached_clients: 1,
            },
            windows: vec![WindowInfo {
                id: window_id,
                session_id,
                name: "main".to_string(),
                index: 0,
                pane_count: 1,
                active_pane_id: Some(pane_id),
            }],
            panes: vec![PaneInfo {
                id: pane_id,
                window_id,
                index: 0,
                cols: 80,
                rows: 24,
                state: PaneState::Normal,
                title: None,
                cwd: None,
            }],
        };

        if let ServerMessage::Attached {
            session,
            windows,
            panes,
        } = msg
        {
            assert_eq!(session.id, session_id);
            assert_eq!(windows.len(), 1);
            assert_eq!(panes.len(), 1);
        }
    }

    #[test]
    fn test_server_message_window_created() {
        let window = WindowInfo {
            id: Uuid::new_v4(),
            session_id: Uuid::new_v4(),
            name: "new-window".to_string(),
            index: 1,
            pane_count: 0,
            active_pane_id: None,
        };

        let msg = ServerMessage::WindowCreated {
            window: window.clone(),
        };

        if let ServerMessage::WindowCreated { window: w } = msg {
            assert_eq!(w.name, "new-window");
            assert_eq!(w.index, 1);
        }
    }

    #[test]
    fn test_server_message_pane_created() {
        let pane = PaneInfo {
            id: Uuid::new_v4(),
            window_id: Uuid::new_v4(),
            index: 0,
            cols: 80,
            rows: 24,
            state: PaneState::Normal,
            title: Some("bash".to_string()),
            cwd: Some("/home/user".to_string()),
        };

        let msg = ServerMessage::PaneCreated { pane: pane.clone() };

        if let ServerMessage::PaneCreated { pane: p } = msg {
            assert_eq!(p.title, Some("bash".to_string()));
            assert_eq!(p.cwd, Some("/home/user".to_string()));
        }
    }

    #[test]
    fn test_server_message_output() {
        let pane_id = Uuid::new_v4();
        let data = b"Hello, World!\n".to_vec();

        let msg = ServerMessage::Output {
            pane_id,
            data: data.clone(),
        };

        if let ServerMessage::Output {
            pane_id: pid,
            data: d,
        } = msg
        {
            assert_eq!(pid, pane_id);
            assert_eq!(d, data);
        }
    }

    #[test]
    fn test_server_message_pane_state_changed() {
        let pane_id = Uuid::new_v4();

        // Normal state
        let msg1 = ServerMessage::PaneStateChanged {
            pane_id,
            state: PaneState::Normal,
        };

        // Claude state
        let msg2 = ServerMessage::PaneStateChanged {
            pane_id,
            state: PaneState::Claude(ClaudeState::default()),
        };

        // Exited state
        let msg3 = ServerMessage::PaneStateChanged {
            pane_id,
            state: PaneState::Exited { code: Some(0) },
        };

        assert_ne!(msg1, msg2);
        assert_ne!(msg2, msg3);
        assert_ne!(msg1, msg3);
    }

    #[test]
    fn test_server_message_claude_state_changed() {
        let pane_id = Uuid::new_v4();
        let state = ClaudeState {
            session_id: Some("abc123".to_string()),
            activity: ClaudeActivity::Thinking,
            model: Some("claude-3-opus".to_string()),
            tokens_used: Some(1500),
        };

        let msg = ServerMessage::ClaudeStateChanged {
            pane_id,
            state: state.clone(),
        };

        if let ServerMessage::ClaudeStateChanged {
            pane_id: pid,
            state: s,
        } = msg
        {
            assert_eq!(pid, pane_id);
            assert_eq!(s.activity, ClaudeActivity::Thinking);
            assert_eq!(s.tokens_used, Some(1500));
        }
    }

    #[test]
    fn test_server_message_pane_closed() {
        let pane_id = Uuid::new_v4();

        // With exit code
        let msg1 = ServerMessage::PaneClosed {
            pane_id,
            exit_code: Some(0),
        };

        // Without exit code (killed)
        let msg2 = ServerMessage::PaneClosed {
            pane_id,
            exit_code: None,
        };

        assert_ne!(msg1, msg2);
    }

    #[test]
    fn test_server_message_window_closed() {
        let window_id = Uuid::new_v4();
        let msg = ServerMessage::WindowClosed { window_id };

        if let ServerMessage::WindowClosed { window_id: wid } = msg {
            assert_eq!(wid, window_id);
        }
    }

    #[test]
    fn test_server_message_session_ended() {
        let session_id = Uuid::new_v4();
        let msg = ServerMessage::SessionEnded { session_id };

        if let ServerMessage::SessionEnded { session_id: sid } = msg {
            assert_eq!(sid, session_id);
        }
    }

    #[test]
    fn test_server_message_error() {
        let msg = ServerMessage::Error {
            code: ErrorCode::SessionNotFound,
            message: "Session 'test' not found".to_string(),
        };

        if let ServerMessage::Error { code, message } = msg {
            assert_eq!(code, ErrorCode::SessionNotFound);
            assert!(message.contains("test"));
        }
    }

    #[test]
    fn test_server_message_pong() {
        assert_eq!(ServerMessage::Pong.clone(), ServerMessage::Pong);
    }

    #[test]
    fn test_error_code_equality() {
        assert_eq!(ErrorCode::SessionNotFound, ErrorCode::SessionNotFound);
        assert_ne!(ErrorCode::SessionNotFound, ErrorCode::WindowNotFound);
        assert_ne!(ErrorCode::WindowNotFound, ErrorCode::PaneNotFound);
        assert_ne!(ErrorCode::PaneNotFound, ErrorCode::InvalidOperation);
        assert_ne!(ErrorCode::InvalidOperation, ErrorCode::ProtocolMismatch);
        assert_ne!(ErrorCode::ProtocolMismatch, ErrorCode::InternalError);
    }

    #[test]
    fn test_error_code_clone() {
        let code = ErrorCode::InternalError;
        let cloned = code.clone();
        assert_eq!(code, cloned);
    }

    #[test]
    fn test_error_code_debug() {
        let code = ErrorCode::ProtocolMismatch;
        let debug = format!("{:?}", code);
        assert_eq!(debug, "ProtocolMismatch");
    }

    #[test]
    fn test_all_error_codes_covered() {
        // Ensure we have a test that touches all variants
        let codes = [
            ErrorCode::SessionNotFound,
            ErrorCode::WindowNotFound,
            ErrorCode::PaneNotFound,
            ErrorCode::InvalidOperation,
            ErrorCode::ProtocolMismatch,
            ErrorCode::InternalError,
        ];

        assert_eq!(codes.len(), 6);
        for (i, code) in codes.iter().enumerate() {
            // Each code should be unique
            for (j, other) in codes.iter().enumerate() {
                if i == j {
                    assert_eq!(code, other);
                } else {
                    assert_ne!(code, other);
                }
            }
        }
    }
}
