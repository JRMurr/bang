use std::{path::PathBuf, time::Duration};

use crossterm::event::{self, Event, KeyCode};

use crate::{
    actions::Actions, command::CommandManager, config::Config,
    renderer::Renderer,
};

#[derive(Debug)]
pub struct Application {
    config: Config,
}

impl Application {
    pub fn new(config_location: Option<PathBuf>) -> crate::Result<Application> {
        // TODO: switch to thiserror
        let config = Config::read(config_location)?;
        Ok(Self { config })
    }

    pub fn run(&mut self, out: impl std::io::Write) -> crate::Result<()> {
        let mut commands = CommandManager::default();

        let config_dir = &self.config.directory;

        for command in &self.config.config.commands {
            let command = command.run(config_dir)?;
            commands.add_command(command)?;
        }
        commands.select(0);
        commands.poll_commands();

        let mut renderer = Renderer::new(out)?;
        renderer.render(&mut commands)?;

        loop {
            if event::poll(Duration::from_millis(16))? && let Event::Key(key) = event::read()? {
                if let Ok(action) = key.try_into() {
                    match action {
                        Actions::Exit => return Ok(()),
                        Actions::Kill => {
                            let selected = commands.get_selected();
                            selected.kill()?;
                        },
                        Actions::Restart => {
                            let selected = commands.get_selected();
                            selected.restart(config_dir)?;
                        },
                        Actions::Previous => commands.previous(),
                        Actions::Next => commands.next(),
                    }
                }
            }

            commands.poll_commands();
            renderer.render(&mut commands)?;
        }
    }
}
