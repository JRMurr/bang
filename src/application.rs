use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};

use crate::{
    command::{CommandBuilder, CommandManager},
    renderer::Renderer,
};

pub struct Application {}

impl Application {
    pub fn run(&mut self, out: impl std::io::Write) -> crate::Result<()> {
        let mut renderer = Renderer::new(out)?;
        let mut commands = CommandManager::default();

        let command_strings =
            vec!["ping localhost".to_string(), "ping 1.1.1.1".to_string()];
        for command in command_strings {
            let command = CommandBuilder::new(command).run()?;
            commands.add_command(command)?;
        }

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
