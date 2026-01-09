//! Client-server connection management
//!
//! Provides Unix socket connection to the ccmux server with
//! automatic message framing and async dispatch.

mod client;
mod handler;

pub use client::Connection;

// These are exported for future use when the full client is implemented
#[allow(unused_imports)]
pub use client::ConnectionState;
#[allow(unused_imports)]
pub use handler::{CallbackHandler, MessageHandler, MessageSender};
