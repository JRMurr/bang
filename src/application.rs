use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};

use crate::{command::CommandManager, config::Config, renderer::Renderer};

pub struct Application {}

impl Application {
    pub fn run(&mut self, out: impl std::io::Write) -> crate::Result<()> {
        let mut commands = CommandManager::default();

        let config = Config::read(None)?;

        for command in config.commands {
            let command = command.run()?;
            commands.add_command(command)?;
        }
        let mut renderer = Renderer::new(out)?;

        commands.select(0);

        commands.poll_commands();

        renderer.render(&mut commands)?;

        loop {
            if event::poll(Duration::from_millis(16))? && let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()), // exit on q
                    KeyCode::Up => commands.previous(),
                    KeyCode::Down => commands.next(),
                    _ => {}
                }
            }

            commands.poll_commands();

            renderer.render(&mut commands)?;
        }
    }
}
