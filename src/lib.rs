//! [bang](https://github.com/JRMurr/bang) is a parallel command runner

#![warn(missing_docs)]

/// the actions a user can take
mod actions;
/// the running app
pub mod application;
/// cli args
pub mod cli;
/// handlers for running,killing,restarting, and displaying commands
mod command;
/// config options/parsing
mod config;
/// Implement views for commands
mod views;

// TODO: make legit error type
/// The main error type for the app
pub type Error = Box<dyn std::error::Error + Send + Sync>;
/// the main restul type for the app
pub type Result<T> = std::result::Result<T, Error>;
