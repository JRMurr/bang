use cursive::{crossterm, view::Nameable, views::TextView};
use cursive_tabs::{Align, TabPanel};
use std::path::PathBuf;
use tracing::instrument;

use crate::{command::CommandManager, config::Config};

/// The main app
#[derive(Debug)]
pub struct Application {
    config: Config,
}

impl Application {
    /// Create the application
    pub fn new(config_location: Option<PathBuf>) -> crate::Result<Application> {
        // TODO: switch to thiserror
        let config = Config::read(config_location)?;
        Ok(Self { config })
    }

    /// Run the application
    #[instrument]
    pub async fn run(&mut self, out: std::io::Stdout) -> crate::Result<()> {
        let mut commands = CommandManager::default();

        let config_dir = &self.config.directory;

        for command in &self.config.config.commands {
            let command = command.run(config_dir)?;
            commands.add_command(command)?;
        }
        // commands.select(0);
        commands.poll_commands();

        let tabs = TabPanel::new()
            .with_tab(TextView::new("First").with_name("First"))
            .with_tab(TextView::new("Second").with_name("Second"))
            .with_bar_alignment(Align::Center)
            .with_bar_placement(cursive_tabs::Placement::VerticalLeft);
        let mut siv = crossterm();

        siv.load_toml(include_str!("../theme.toml")).unwrap();

        siv.add_layer(tabs);

        siv.add_global_callback('q', |s| s.quit());

        siv.run();

        Ok(())
    }
}
