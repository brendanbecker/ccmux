//! Command executor for sideband commands
//!
//! Executes parsed sideband commands by dispatching to the session manager
//! and other system components.

use std::sync::Arc;
use parking_lot::Mutex;
use thiserror::Error;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use super::commands::{ControlAction, NotifyLevel, PaneRef, SidebandCommand, SplitDirection};
use crate::session::SessionManager;

/// Errors that can occur during command execution
#[derive(Debug, Error)]
pub enum ExecuteError {
    #[error("Pane not found: {0}")]
    PaneNotFound(String),

    #[error("Window not found: {0}")]
    WindowNotFound(Uuid),

    #[error("Session not found: {0}")]
    SessionNotFound(Uuid),

    #[error("Invalid pane reference: {0}")]
    InvalidPaneRef(String),

    #[error("Operation not supported: {0}")]
    NotSupported(String),

    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
}

/// Result type for command execution
pub type ExecuteResult<T> = Result<T, ExecuteError>;

/// Executor for sideband commands
///
/// Takes parsed commands and executes them against the session manager.
pub struct CommandExecutor {
    /// Reference to the session manager
    session_manager: Arc<Mutex<SessionManager>>,
}

impl CommandExecutor {
    /// Create a new command executor
    pub fn new(session_manager: Arc<Mutex<SessionManager>>) -> Self {
        Self { session_manager }
    }

    /// Execute a sideband command
    ///
    /// # Arguments
    /// * `command` - The command to execute
    /// * `source_pane` - The UUID of the pane that emitted the command
    pub fn execute(
        &self,
        command: SidebandCommand,
        source_pane: Uuid,
    ) -> ExecuteResult<()> {
        debug!("Executing sideband command: {:?}", command);

        match command {
            SidebandCommand::Spawn {
                direction,
                command,
                cwd,
            } => self.execute_spawn(source_pane, direction, command, cwd),

            SidebandCommand::Focus { pane } => self.execute_focus(source_pane, pane),

            SidebandCommand::Input { pane, text } => self.execute_input(source_pane, pane, text),

            SidebandCommand::Scroll { pane, lines } => {
                self.execute_scroll(source_pane, pane, lines)
            }

            SidebandCommand::Notify {
                title,
                message,
                level,
            } => self.execute_notify(title, message, level),

            SidebandCommand::Control { action, pane } => {
                self.execute_control(source_pane, pane, action)
            }
        }
    }

    /// Execute a batch of commands
    pub fn execute_batch(
        &self,
        commands: Vec<SidebandCommand>,
        source_pane: Uuid,
    ) -> Vec<ExecuteResult<()>> {
        commands
            .into_iter()
            .map(|cmd| self.execute(cmd, source_pane))
            .collect()
    }

    /// Resolve a pane reference to a concrete UUID
    fn resolve_pane_ref(&self, pane_ref: PaneRef, source_pane: Uuid) -> ExecuteResult<Uuid> {
        let manager = self.session_manager.lock();

        match pane_ref {
            PaneRef::Active => Ok(source_pane),
            PaneRef::Id(id) => {
                if manager.find_pane(id).is_some() {
                    Ok(id)
                } else {
                    Err(ExecuteError::PaneNotFound(id.to_string()))
                }
            }
            PaneRef::Index(idx) => {
                // Find the window containing the source pane, then get pane by index
                if let Some((_, window, _)) = manager.find_pane(source_pane) {
                    if let Some(pane) = window.get_pane_by_index(idx) {
                        Ok(pane.id())
                    } else {
                        Err(ExecuteError::PaneNotFound(format!("index {}", idx)))
                    }
                } else {
                    Err(ExecuteError::PaneNotFound(format!(
                        "source pane {} not found",
                        source_pane
                    )))
                }
            }
        }
    }

    /// Execute spawn command - create a new pane
    fn execute_spawn(
        &self,
        source_pane: Uuid,
        direction: SplitDirection,
        command: Option<String>,
        cwd: Option<String>,
    ) -> ExecuteResult<()> {
        info!(
            "Spawn requested: direction={:?}, command={:?}, cwd={:?}",
            direction, command, cwd
        );

        let _manager = self.session_manager.lock();

        // Find the window containing the source pane
        // TODO: Implement actual pane splitting in SessionManager
        // This requires:
        // 1. Finding the window containing source_pane
        // 2. Creating a new pane in that window
        // 3. Spawning a PTY for the new pane
        // 4. Running the command if specified

        warn!(
            "Spawn command not yet implemented - pane: {}, direction: {:?}",
            source_pane, direction
        );

        // For now, log the request but don't fail
        // This allows the command to be stripped from output while
        // the implementation is completed
        Ok(())
    }

    /// Execute focus command - focus a specific pane
    fn execute_focus(&self, source_pane: Uuid, pane: PaneRef) -> ExecuteResult<()> {
        let target_pane = self.resolve_pane_ref(pane, source_pane)?;

        info!("Focus requested for pane: {}", target_pane);

        // TODO: Implement pane focusing
        // This requires adding focus tracking to the session/window system
        // and notifying clients of focus changes

        warn!("Focus command not yet fully implemented - pane: {}", target_pane);

        Ok(())
    }

    /// Execute input command - send input to a pane
    fn execute_input(
        &self,
        source_pane: Uuid,
        pane: PaneRef,
        text: String,
    ) -> ExecuteResult<()> {
        let target_pane = self.resolve_pane_ref(pane, source_pane)?;

        info!("Input requested for pane {}: {:?}", target_pane, text);

        // TODO: Implement input sending
        // This requires:
        // 1. Getting the PTY handle for the target pane
        // 2. Writing the text to the PTY's stdin
        // PtyManager needs a method like: send_input(pane_id, &[u8])

        warn!(
            "Input command not yet fully implemented - pane: {}, text_len: {}",
            target_pane,
            text.len()
        );

        Ok(())
    }

    /// Execute scroll command - scroll pane content
    fn execute_scroll(
        &self,
        source_pane: Uuid,
        pane: Option<PaneRef>,
        lines: i32,
    ) -> ExecuteResult<()> {
        let target_pane = match pane {
            Some(p) => self.resolve_pane_ref(p, source_pane)?,
            None => source_pane,
        };

        info!("Scroll requested for pane {}: {} lines", target_pane, lines);

        // TODO: Implement scrolling
        // This requires viewport tracking in the pane
        // Negative lines = scroll up (show older content)
        // Positive lines = scroll down (show newer content)

        warn!(
            "Scroll command not yet fully implemented - pane: {}, lines: {}",
            target_pane, lines
        );

        Ok(())
    }

    /// Execute notify command - show a notification
    fn execute_notify(
        &self,
        title: Option<String>,
        message: String,
        level: NotifyLevel,
    ) -> ExecuteResult<()> {
        // Notifications are logged and will be broadcast to clients
        let level_str = match level {
            NotifyLevel::Info => "INFO",
            NotifyLevel::Warning => "WARN",
            NotifyLevel::Error => "ERROR",
        };

        match level {
            NotifyLevel::Info => {
                info!(
                    "Notification [{}]: {} - {}",
                    level_str,
                    title.as_deref().unwrap_or(""),
                    message
                );
            }
            NotifyLevel::Warning => {
                warn!(
                    "Notification [{}]: {} - {}",
                    level_str,
                    title.as_deref().unwrap_or(""),
                    message
                );
            }
            NotifyLevel::Error => {
                error!(
                    "Notification [{}]: {} - {}",
                    level_str,
                    title.as_deref().unwrap_or(""),
                    message
                );
            }
        }

        // TODO: Broadcast notification to connected clients
        // This requires adding a notification channel to the server

        Ok(())
    }

    /// Execute control command - pane control actions
    fn execute_control(
        &self,
        source_pane: Uuid,
        pane: PaneRef,
        action: ControlAction,
    ) -> ExecuteResult<()> {
        let target_pane = self.resolve_pane_ref(pane, source_pane)?;

        match action {
            ControlAction::Close => {
                info!("Close requested for pane: {}", target_pane);
                // TODO: Implement pane closing
                // This requires:
                // 1. Killing the PTY process
                // 2. Removing the pane from the window
                // 3. Notifying clients
                warn!("Close command not yet implemented - pane: {}", target_pane);
            }

            ControlAction::Resize { cols, rows } => {
                info!(
                    "Resize requested for pane {}: {}x{}",
                    target_pane, cols, rows
                );

                let mut manager = self.session_manager.lock();
                if let Some(pane) = manager.find_pane_mut(target_pane) {
                    pane.resize(cols, rows);
                    info!("Pane {} resized to {}x{}", target_pane, cols, rows);
                } else {
                    return Err(ExecuteError::PaneNotFound(target_pane.to_string()));
                }
            }

            ControlAction::Pin => {
                info!("Pin viewport requested for pane: {}", target_pane);
                // TODO: Implement viewport pinning
                // This disables auto-scroll for the pane
                warn!("Pin command not yet implemented - pane: {}", target_pane);
            }

            ControlAction::Unpin => {
                info!("Unpin viewport requested for pane: {}", target_pane);
                // TODO: Implement viewport unpinning
                // This re-enables auto-scroll for the pane
                warn!("Unpin command not yet implemented - pane: {}", target_pane);
            }
        }

        Ok(())
    }
}

impl std::fmt::Debug for CommandExecutor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandExecutor")
            .field("session_manager", &"Arc<Mutex<SessionManager>>")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_executor() -> (CommandExecutor, Arc<Mutex<SessionManager>>) {
        let manager = Arc::new(Mutex::new(SessionManager::new()));
        let executor = CommandExecutor::new(Arc::clone(&manager));
        (executor, manager)
    }

    fn setup_test_pane(manager: &Arc<Mutex<SessionManager>>) -> Uuid {
        let mut mgr = manager.lock();
        let session = mgr.create_session("test").unwrap();
        let session_id = session.id();

        let session = mgr.get_session_mut(session_id).unwrap();
        let window = session.create_window(None);
        let window_id = window.id();

        let window = session.get_window_mut(window_id).unwrap();
        let pane = window.create_pane();
        pane.id()
    }

    #[test]
    fn test_executor_creation() {
        let (executor, _) = create_test_executor();
        let debug_str = format!("{:?}", executor);
        assert!(debug_str.contains("CommandExecutor"));
    }

    #[test]
    fn test_execute_spawn() {
        let (executor, manager) = create_test_executor();
        let pane_id = setup_test_pane(&manager);

        let cmd = SidebandCommand::Spawn {
            direction: SplitDirection::Vertical,
            command: Some("echo hello".to_string()),
            cwd: None,
        };

        // Should not error (just logs warning for now)
        let result = executor.execute(cmd, pane_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_focus() {
        let (executor, manager) = create_test_executor();
        let pane_id = setup_test_pane(&manager);

        let cmd = SidebandCommand::Focus {
            pane: PaneRef::Active,
        };

        let result = executor.execute(cmd, pane_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_focus_by_index() {
        let (executor, manager) = create_test_executor();
        let pane_id = setup_test_pane(&manager);

        let cmd = SidebandCommand::Focus {
            pane: PaneRef::Index(0),
        };

        let result = executor.execute(cmd, pane_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_focus_invalid_index() {
        let (executor, manager) = create_test_executor();
        let pane_id = setup_test_pane(&manager);

        let cmd = SidebandCommand::Focus {
            pane: PaneRef::Index(999),
        };

        let result = executor.execute(cmd, pane_id);
        assert!(matches!(result, Err(ExecuteError::PaneNotFound(_))));
    }

    #[test]
    fn test_execute_focus_invalid_uuid() {
        let (executor, manager) = create_test_executor();
        let pane_id = setup_test_pane(&manager);

        let cmd = SidebandCommand::Focus {
            pane: PaneRef::Id(Uuid::new_v4()), // Random UUID not in manager
        };

        let result = executor.execute(cmd, pane_id);
        assert!(matches!(result, Err(ExecuteError::PaneNotFound(_))));
    }

    #[test]
    fn test_execute_input() {
        let (executor, manager) = create_test_executor();
        let pane_id = setup_test_pane(&manager);

        let cmd = SidebandCommand::Input {
            pane: PaneRef::Active,
            text: "ls -la\n".to_string(),
        };

        let result = executor.execute(cmd, pane_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_scroll() {
        let (executor, manager) = create_test_executor();
        let pane_id = setup_test_pane(&manager);

        let cmd = SidebandCommand::Scroll {
            pane: None,
            lines: -20,
        };

        let result = executor.execute(cmd, pane_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_notify_info() {
        let (executor, _) = create_test_executor();

        let cmd = SidebandCommand::Notify {
            title: Some("Test".to_string()),
            message: "Test message".to_string(),
            level: NotifyLevel::Info,
        };

        let result = executor.execute(cmd, Uuid::new_v4());
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_notify_warning() {
        let (executor, _) = create_test_executor();

        let cmd = SidebandCommand::Notify {
            title: None,
            message: "Warning message".to_string(),
            level: NotifyLevel::Warning,
        };

        let result = executor.execute(cmd, Uuid::new_v4());
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_notify_error() {
        let (executor, _) = create_test_executor();

        let cmd = SidebandCommand::Notify {
            title: Some("Error".to_string()),
            message: "Error message".to_string(),
            level: NotifyLevel::Error,
        };

        let result = executor.execute(cmd, Uuid::new_v4());
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_control_close() {
        let (executor, manager) = create_test_executor();
        let pane_id = setup_test_pane(&manager);

        let cmd = SidebandCommand::Control {
            action: ControlAction::Close,
            pane: PaneRef::Active,
        };

        let result = executor.execute(cmd, pane_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_control_resize() {
        let (executor, manager) = create_test_executor();
        let pane_id = setup_test_pane(&manager);

        let cmd = SidebandCommand::Control {
            action: ControlAction::Resize { cols: 120, rows: 40 },
            pane: PaneRef::Active,
        };

        let result = executor.execute(cmd, pane_id);
        assert!(result.is_ok());

        // Verify resize was applied
        let mgr = manager.lock();
        let (_, _, pane) = mgr.find_pane(pane_id).unwrap();
        assert_eq!(pane.dimensions(), (120, 40));
    }

    #[test]
    fn test_execute_control_pin() {
        let (executor, manager) = create_test_executor();
        let pane_id = setup_test_pane(&manager);

        let cmd = SidebandCommand::Control {
            action: ControlAction::Pin,
            pane: PaneRef::Active,
        };

        let result = executor.execute(cmd, pane_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_control_unpin() {
        let (executor, manager) = create_test_executor();
        let pane_id = setup_test_pane(&manager);

        let cmd = SidebandCommand::Control {
            action: ControlAction::Unpin,
            pane: PaneRef::Active,
        };

        let result = executor.execute(cmd, pane_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_batch() {
        let (executor, manager) = create_test_executor();
        let pane_id = setup_test_pane(&manager);

        let commands = vec![
            SidebandCommand::Focus {
                pane: PaneRef::Active,
            },
            SidebandCommand::Notify {
                title: None,
                message: "test".to_string(),
                level: NotifyLevel::Info,
            },
            SidebandCommand::Scroll {
                pane: None,
                lines: -5,
            },
        ];

        let results = executor.execute_batch(commands, pane_id);
        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|r| r.is_ok()));
    }

    #[test]
    fn test_execute_batch_with_error() {
        let (executor, manager) = create_test_executor();
        let pane_id = setup_test_pane(&manager);

        let commands = vec![
            SidebandCommand::Focus {
                pane: PaneRef::Active,
            },
            SidebandCommand::Focus {
                pane: PaneRef::Id(Uuid::new_v4()), // Invalid - will fail
            },
            SidebandCommand::Notify {
                title: None,
                message: "test".to_string(),
                level: NotifyLevel::Info,
            },
        ];

        let results = executor.execute_batch(commands, pane_id);
        assert_eq!(results.len(), 3);
        assert!(results[0].is_ok());
        assert!(results[1].is_err());
        assert!(results[2].is_ok());
    }

    #[test]
    fn test_resolve_pane_ref_active() {
        let (executor, manager) = create_test_executor();
        let pane_id = setup_test_pane(&manager);

        let result = executor.resolve_pane_ref(PaneRef::Active, pane_id);
        assert_eq!(result.unwrap(), pane_id);
    }

    #[test]
    fn test_resolve_pane_ref_by_id() {
        let (executor, manager) = create_test_executor();
        let pane_id = setup_test_pane(&manager);

        let result = executor.resolve_pane_ref(PaneRef::Id(pane_id), pane_id);
        assert_eq!(result.unwrap(), pane_id);
    }

    #[test]
    fn test_resolve_pane_ref_by_index() {
        let (executor, manager) = create_test_executor();
        let pane_id = setup_test_pane(&manager);

        let result = executor.resolve_pane_ref(PaneRef::Index(0), pane_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_error_display() {
        let err = ExecuteError::PaneNotFound("test".to_string());
        assert!(err.to_string().contains("Pane not found"));

        let err = ExecuteError::NotSupported("feature".to_string());
        assert!(err.to_string().contains("not supported"));
    }
}
