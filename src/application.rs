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

        let command_strings = vec![
            "ping -i 0.1 localhost".to_string(),
            "ping 1.1.1.1".to_string(),
        ];
        for command in command_strings {
            let command = CommandBuilder::new(command).run()?;
            commands.add_command(command)?;
        }
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
