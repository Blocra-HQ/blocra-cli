//! blocra: signed, verifiable traction reports for on-chain projects.
//!
//! The library behind the `blocra` command: fetch BlocRA-signed reports
//! ([`api`]), verify them ([`verify`]) and render or parse their Markdown form
//! ([`markdown`]).

pub mod api;
pub mod config;
pub mod defaults;
pub mod js;
pub mod markdown;
pub mod strkey;
pub mod verify;
