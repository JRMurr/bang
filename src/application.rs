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

        for command in &self.config.config.commands {
            let command = command.run(&self.config.directory)?;
            commands.add_command(command)?;
        }
        let mut renderer = Renderer::new(out)?;

        commands.select(0);

        commands.poll_commands();

        renderer.render(&mut commands)?;

        loop {
            if event::poll(Duration::from_millis(16))? && let Event::Key(key) = event::read()? {
                if let Ok(action) = key.try_into() {
                    match action {
                        Actions::Exit => return Ok(()),
                        Actions::Kill => todo!(),
                        Actions::Restart => {
                            let selected = commands.get_selected();
                            selected.restart(&self.config.directory)?;
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
