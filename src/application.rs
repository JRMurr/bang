use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};

use crate::{command::CommandManager, renderer::Renderer};

pub struct Application {}

impl Application {
    pub fn run(&mut self, out: impl std::io::Write) -> crate::Result<()> {
        let mut renderer = Renderer::new(out)?;
        let mut commands = CommandManager::default();
        commands.add_command("ping localhost".to_string())?;
        commands.add_command("ping 1.1.1.1".to_string())?;

        commands.poll_commands();

        renderer.render(&commands)?;
        loop {
            if event::poll(Duration::from_millis(16))? && let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }

            commands.poll_commands();

            renderer.render(&commands)?;
        }
    }
}
