use std::{path::PathBuf, time::Duration};

use crate::{
    actions::Actions, command::CommandManager, config::Config,
    renderer::Renderer,
};
use crossbeam::channel::Sender;
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent};
use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;
use tokio::time;
use tracing::instrument;
/// The main app
#[derive(Debug)]
pub struct Application {
    config: Config,
    // TODO: for now only other state is showing help
    // if more screens are added think of a better way of tracking app state
    in_help: bool,
}

impl Application {
    /// Create the application
    pub fn new(config_location: Option<PathBuf>) -> crate::Result<Application> {
        // TODO: switch to thiserror
        let config = Config::read(config_location)?;
        Ok(Self {
            config,
            in_help: false,
        })
    }

    /// Run the application
    #[instrument]
    pub async fn run(&mut self, out: std::io::Stdout) -> crate::Result<()> {
        let mut interval = time::interval(time::Duration::from_millis(32));
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
        // TODO: look into https://github.com/tokio-rs/console/blob/3bf60bce7b478c189a3145311e06f14cb2fc1e11/tokio-console/src/main.rs#L73
        // seems to not re-draw the frame all the time which will help cpu usage
        // currently cheating by using interval to have a min time each frame
        // will take
        loop {
            if let Ok(key) = receiver.try_recv() {
                if !self.in_help && let Ok(action) = key.try_into() {
                    match action {
                        Actions::Exit => return Ok(()),
                        // TODO: add map selected helper
                        Actions::Kill => {
                            let selected = commands.get_selected();
                            selected.kill().await?;
                        },
                        Actions::Restart => {
                            let selected = commands.get_selected();
                            selected.restart(config_dir)?;
                        },
                        Actions::Previous => commands.previous(),
                        Actions::Next => commands.next(),
                        Actions::Scroll(dir, amount) => {
                            let selected = commands.get_selected();
                            selected.scroll(dir, amount);
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
            interval.tick().await;
        }
    }
}

#[instrument]
fn create_input_thread(sender: Sender<KeyEvent>) {
    tokio::spawn(async move {
        let mut reader = EventStream::new();

        loop {
            // if let Ok(poll) = event::poll(Duration::from_millis(10)) && poll
            // && let Ok(Event::Key(key)) = event::read() &&
            // sender.send(key).is_err() {     break;
            // }
            let mut delay = Delay::new(Duration::from_millis(16)).fuse();
            let mut event = reader.next().fuse();

            select! {
                _ = delay => {},
                maybe_event = event => {
                    match maybe_event {
                        Some(Ok(event)) => {
                            if let Event::Key(key) = event {
                                if sender.send(key).is_err() {
                                    break;
                                }
                            }
                        }
                        Some(Err(e)) => panic!("Error: {:?}\r", e),
                        None => break,
                    }
                }
            }
        }
    });
}
