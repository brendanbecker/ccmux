//! ccmux client - Terminal UI for ccmux

use ccmux_utils::Result;

mod ui;
mod input;
mod connection;

#[tokio::main]
async fn main() -> Result<()> {
    ccmux_utils::init_logging()?;
    tracing::info!("ccmux client starting");

    // TODO: Implement client
    Ok(())
}
