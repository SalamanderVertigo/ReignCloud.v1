//! ReignCloud API — Vertical Slice Architecture
//!
//! Server-only dependencies (sqlx, jwt, bcrypt) are gated behind
//! `cfg(not(target_arch = "wasm32"))` so the crate compiles for WASM too.

use dioxus::prelude::*;

pub mod auth;
#[cfg(not(target_arch = "wasm32"))]
pub mod db;
pub mod features;

// Re-export feature endpoints so consumers can reference them directly.
pub use features::users::login::login;
pub use features::users::register::register;
pub use features::messages::create::create_message;
pub use features::messages::list::list_messages;
pub use features::messages::update::update_message;
pub use features::messages::delete::delete_message;

/// Echo the user input on the server (kept for testing).
#[post("/api/echo")]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
