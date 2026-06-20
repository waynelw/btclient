pub mod commands;
pub mod domain;
pub mod engine;
pub mod persistence;
pub mod runtime;
pub mod scheduler;
pub mod service;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
