//! ccmux-utils: Common utilities shared across crates

pub mod error;
pub mod logging;
pub mod paths;

pub use error::{CcmuxError, Result};
pub use logging::init_logging;
pub use paths::{socket_path, config_dir, state_dir};
