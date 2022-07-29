pub mod application;
mod command;
mod renderer;

// TODO: make legit error type
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
