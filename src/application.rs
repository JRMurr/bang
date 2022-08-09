use std::{path::PathBuf, time::Duration};

use crossbeam::channel::Sender;
use crossterm::event::{self, Event, KeyCode, KeyEvent};

use crate::{
    actions::Actions, command::CommandManager, config::Config,
    renderer::Renderer,
};

#[derive(Debug)]
pub struct Application {
    config: Config,
    // TODO: for now only other state is showing help
    // if more screens are added think of a better way of tracking app state
    in_help: bool,
}

impl Application {
    pub fn new(config_location: Option<PathBuf>) -> crate::Result<Application> {
        // TODO: switch to thiserror
        let config = Config::read(config_location)?;
        Ok(Self {
            config,
            in_help: false,
        })
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
        renderer.render(&mut commands, false)?;
        let (sender, receiver) = crossbeam::channel::bounded(10);
        create_input_thread(sender);
        loop {
            if let Ok(key) = receiver.try_recv() {
                if !self.in_help && let Ok(action) = key.try_into() {
                    match action {
                        Actions::Exit => return Ok(()),
                        // TODO: add map selected helper
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
                        Actions::Scroll(dir) => {
                            let selected = commands.get_selected();
                            selected.scroll(dir);
                        }
                        Actions::Help => {
                            self.in_help = true;
                        }
                    }
                }
                if self.in_help && KeyCode::Esc == key.code {
                    self.in_help = false;
                }
            }

            commands.poll_commands();
            renderer.render(&mut commands, self.in_help)?;
        }
    }
}

fn create_input_thread(
    sender: Sender<KeyEvent>,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || loop {
        if let Ok(poll) = event::poll(Duration::from_millis(10)) && poll && let Ok(Event::Key(key)) = event::read() && sender.send(key).is_err() {
            break;
        }
    })
}
