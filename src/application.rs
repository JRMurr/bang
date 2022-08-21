use crate::{command::CommandManager, config::Config};
use cursive::{
    crossterm,
    view::Nameable,
    views::{ScrollView, TextView},
};
use cursive_tabs::{Align, TabPanel};
use std::path::PathBuf;
use tracing::instrument;

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

        let mut tabs = TabPanel::new()
            .with_bar_alignment(Align::Center)
            .with_bar_placement(cursive_tabs::Placement::VerticalLeft);

        for command in commands.iter() {
            let name = command.name.to_string();
            let content = command.content.clone();
            let text = TextView::new_with_content(content);

            tabs.add_tab(ScrollView::new(text).with_name(name))
        }

        let mut siv = crossterm();

        siv.load_toml(include_str!("../theme.toml")).unwrap();

        siv.add_layer(tabs);

        siv.add_global_callback('q', |s| s.quit());

        siv.set_autorefresh(true);

        siv.run();

        Ok(())
    }
}
