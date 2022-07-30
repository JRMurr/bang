use crossbeam::channel::{unbounded, Receiver};
use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    path::Path,
    process::{Child, Command as CommandRunner, Stdio},
    thread,
};
use tui::text::{Span, Spans};

pub struct CommandBuilder<'a> {
    command: String,
    name: Option<String>,
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

// TODO: make builder struct that calls run at the end
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
    pub commands: Vec<Command>,
}

impl CommandManager {
    pub fn add_command(&mut self, command: Command) -> crate::Result<()> {
        self.commands.push(command);

        Ok(())
    }

    pub fn poll_commands(&mut self) {
        self.commands
            .iter_mut()
            .for_each(|command| command.populate_lines());
    }
}
