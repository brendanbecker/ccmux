//! ccmux server - Background daemon

use ccmux_utils::Result;

mod claude;
mod config;
mod parser;
mod persistence;
mod pty;
mod reply;
mod session;

pub use reply::{ReplyError, ReplyHandler};

#[tokio::main]
async fn main() -> Result<()> {
    ccmux_utils::init_logging()?;
    tracing::info!("ccmux server starting");

    // TODO: Implement server
    Ok(())
}
