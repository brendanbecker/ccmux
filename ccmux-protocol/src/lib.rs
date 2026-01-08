//! ccmux-protocol: Shared IPC definitions for client-server communication

pub mod messages;
pub mod types;
pub mod codec;

pub use messages::{ClientMessage, ServerMessage};
pub use types::{SessionInfo, WindowInfo, PaneInfo, ClaudeState, PaneState};
pub use codec::MessageCodec;
