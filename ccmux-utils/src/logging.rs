//! Logging setup

use tracing_subscriber::{fmt, EnvFilter};

use crate::Result;

/// Initialize tracing/logging for the application
pub fn init_logging() -> Result<()> {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(filter)
        .init();

    Ok(())
}
