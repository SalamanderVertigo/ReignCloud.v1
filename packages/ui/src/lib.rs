//! This crate contains all shared UI for the workspace.

mod hero;
pub use hero::Hero;

mod navbar;
pub use navbar::Navbar;

mod echo;
pub use echo::Echo;

mod auth_test;
pub use auth_test::AuthTest;

mod use_websocket;
pub use use_websocket::use_websocket;
