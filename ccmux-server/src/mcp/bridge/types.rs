//! Shared types for MCP bridge

/// Heartbeat interval in milliseconds
pub const HEARTBEAT_INTERVAL_MS: u64 = 1000;

/// Heartbeat timeout in milliseconds (detect loss within 2-3 seconds)
pub const HEARTBEAT_TIMEOUT_MS: u64 = 2000;

/// Reconnection delays in milliseconds (exponential backoff)
pub const RECONNECT_DELAYS_MS: &[u64] = &[100, 200, 400, 800, 1600];

/// Maximum number of reconnection attempts
pub const MAX_RECONNECT_ATTEMPTS: u8 = 5;

/// BUG-037 FIX: Timeout for waiting for a daemon response (in seconds)
/// This prevents tool calls from hanging indefinitely if the daemon
/// doesn't send the expected response. Claude Code has its own timeout
/// that triggers AbortError, so we set this slightly lower to provide
/// a more informative error message.
pub const DAEMON_RESPONSE_TIMEOUT_SECS: u64 = 25;

/// Connection state for daemon communication
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    /// Connected and healthy
    Connected,
    /// Connection lost, attempting recovery
    Reconnecting { attempt: u8 },
    /// Disconnected, recovery failed or not yet attempted
    Disconnected,
}
