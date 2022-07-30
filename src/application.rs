use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};

use crate::{command::Command, renderer::Renderer};

pub struct Application {}

impl Application {
    pub fn run(&mut self, out: impl std::io::Write) -> crate::Result<()> {
        let mut renderer = Renderer::new(out)?;
        let mut command = Command::new("ping localhost".to_string());
        command.run()?;

        renderer.render(command.spans())?;
        loop {
            if event::poll(Duration::from_millis(16))? && let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }

            command.populate_lines();

            renderer.render(command.spans())?;
        }
    }
}
