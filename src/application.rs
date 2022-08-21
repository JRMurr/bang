use crate::{config::Config, views::get_command_view};
use cursive::{crossterm, views::NamedView};
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
        let config_dir = &self.config.directory;

        let mut tabs = TabPanel::new()
            .with_bar_alignment(Align::Center)
            .with_bar_placement(cursive_tabs::Placement::VerticalLeft);

        // TODO: need commands to not be dropped until the end of run
        // if i wrap command and try to make it a view no output is shown so
        // this hack is the workaround for now
        let mut commands =
            Vec::with_capacity(self.config.config.commands.len());

        for command in &self.config.config.commands {
            let command = command.run(config_dir)?;
            let name = command.name.clone();
            let view = NamedView::new(name, get_command_view(&command));

            tabs.add_tab(view);
            commands.push(command);
        }

        // TODO: this shows the output of the first command but the last one
        // looks selected?
        tabs.set_active_tab(
            tabs.tab_order().first().expect("need at least one command"),
        )?;

        let mut siv = crossterm();

        siv.load_toml(include_str!("../theme.toml")).unwrap();

        siv.add_fullscreen_layer(tabs);

        siv.add_global_callback('q', |s| s.quit());

        siv.add_global_callback('?', crate::views::set_help_menu);

        siv.set_autorefresh(true);
        siv.run();

        Ok(())
    }
}
