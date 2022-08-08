mod actions;
pub mod application;
pub mod cli;
mod command;
mod config;
mod renderer;
mod ui;

// TODO: make legit error type
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
