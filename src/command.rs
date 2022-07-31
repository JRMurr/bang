use crossbeam::channel::{unbounded, Receiver};
use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, BufReader},
    path::Path,
    process::{Child, Command as CommandRunner, Stdio},
    slice::Iter,
    thread,
};
use tui::{
    text::{Span, Spans},
    widgets::ListState,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandBuilder<'a> {
    command: String,
    name: Option<String>,
    #[serde(borrow)]
    running_dir: Option<&'a Path>,
}

impl<'a> CommandBuilder<'a> {
    pub fn new(command: String) -> Self {
        Self {
            command,
            name: None,
            running_dir: None,
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn running_dir(mut self, path: &'a Path) -> Self {
        self.running_dir = Some(path);
        self
    }

    pub fn run(self) -> crate::Result<Command> {
        let command = shell_words::split(&self.command)?;

        // TODO: add errors for parsing
        let program = command.first().expect("Command should not be empty");
        let args = &command[1..];

        let mut binding = CommandRunner::new(program);
        let mut command_runner = binding.args(args).stdout(Stdio::piped());

        if let Some(dir) = self.running_dir {
            command_runner = command_runner.current_dir(dir);
        }

        let mut child = command_runner
            .spawn()
            .unwrap_or_else(|_| panic!("failed to run {}", self.command));

        let (sender, receiver) = unbounded::<String>();
        let stdout = child.stdout.take().unwrap();
        thread::spawn(move || {
            let mut f = BufReader::new(stdout);
            loop {
                let mut buf = String::new();
                match f.read_line(&mut buf) {
                    Ok(_) => {
                        sender.send(buf).unwrap();
                    }
                    Err(e) => println!("an error!: {:?}", e),
                }
            }
        });

        let name = self.name.unwrap_or(self.command);

        Ok(Command::new(name, receiver, child))
    }
}

#[derive(Debug)]
pub struct Command {
    pub name: String,
    receiver: Receiver<String>,
    lines: Vec<String>,
    child: Child,
}

impl Command {
    pub fn new(name: String, receiver: Receiver<String>, child: Child) -> Self {
        Self {
            name,
            receiver,
            child,
            lines: Vec::new(),
        }
    }

    pub fn populate_lines(&mut self) {
        // TODO: handle error of disconnected https://docs.rs/crossbeam/0.8.2/crossbeam/channel/enum.TryRecvError.html

        if let Ok(line) = self.receiver.try_recv() {
            self.lines.push(line);
        }
    }

    pub fn spans(&self) -> Vec<Spans> {
        self.lines
            .iter()
            .map(|line| Spans::from(vec![Span::raw(line)]))
            .collect()
    }
}

#[derive(Debug, Default)]
pub struct CommandManager {
    commands: Vec<Command>,
    state: ListState,
}

impl CommandManager {
    pub fn add_command(&mut self, command: Command) -> crate::Result<()> {
        self.commands.push(command);

        Ok(())
    }

    pub fn get(&self, idx: usize) -> Option<&Command> {
        self.commands.get(idx)
    }

    pub fn poll_commands(&mut self) {
        self.commands
            .iter_mut()
            .for_each(|command| command.populate_lines());
    }

    pub fn iter(&self) -> Iter<Command> {
        self.commands.iter()
    }

    pub fn select(&mut self, idx: usize) {
        self.state.select(Some(idx));
    }

    pub fn state(&mut self) -> &mut ListState {
        &mut self.state
    }

    pub fn get_selected(&self) -> &Command {
        // TODO: throw errors here
        let selected = self
            .state
            .selected()
            .expect("a command must be selected at all times");

        self.commands
            .get(selected)
            .expect("selected command must be in list")
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.commands.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    // Select the previous item. This will not be reflected until the widget is
    // drawn in the `Terminal::draw` callback using
    // `Frame::render_stateful_widget`.
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.commands.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
