//! ccmux server - Background daemon

use ccmux_utils::Result;

mod session;
mod pty;
mod parser;
mod claude;
mod persistence;
mod config;

#[tokio::main]
async fn main() -> Result<()> {
    ccmux_utils::init_logging()?;
    tracing::info!("ccmux server starting");

    // TODO: Implement server
    Ok(())
}
