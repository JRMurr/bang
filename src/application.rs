use std::time::Duration;

use crossbeam::channel::unbounded;
use crossterm::event::{self, Event, KeyCode};

use crate::{command::run_command, renderer::Renderer};

pub struct Application {}

impl Application {
    pub fn run(&mut self, out: impl std::io::Write) -> crate::Result<()> {
        let mut renderer = Renderer::new(out)?;
        let mut lines = Vec::new();

        renderer.render(&lines)?;
        let (tx, rx) = unbounded::<String>();
        run_command(tx);
        loop {
            if event::poll(Duration::from_millis(16))? && let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
            // TODO: handle error of disconnected https://docs.rs/crossbeam/0.8.2/crossbeam/channel/enum.TryRecvError.html
            if let Ok(line) = rx.try_recv() {
                lines.push(line);
            }

            renderer.render(&lines)?;
        }
    }
}
