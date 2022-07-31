use crossbeam::channel::{unbounded, Receiver};
use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Child, Command as CommandRunner, Stdio},
    slice::Iter,
    thread,
};
use tui::widgets::{ListItem, ListState};

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandBuilder {
    command: String,
    name: Option<String>,
    running_dir: Option<PathBuf>,
}

fn expand_tilde<P: AsRef<Path>>(path_user_input: P) -> Option<PathBuf> {
    let p = path_user_input.as_ref();
    if !p.starts_with("~") {
        return Some(p.to_path_buf());
    }
    if p == Path::new("~") {
        return dirs::home_dir();
    }
    dirs::home_dir().map(|mut h| {
        if h == Path::new("/") {
            // Corner case: `h` root directory;
            // don't prepend extra `/`, just drop the tilde.
            p.strip_prefix("~").unwrap().to_path_buf()
        } else {
            h.push(p.strip_prefix("~/").unwrap());
            h
        }
    })
}

impl CommandBuilder {
    pub fn run(self) -> crate::Result<Command> {
        let command = shell_words::split(&self.command)?;

        // TODO: add errors for parsing
        let program = command.first().expect("Command should not be empty");
        let args = &command[1..];

        let mut binding = CommandRunner::new(program);
        let mut command_runner = binding.args(args).stdout(Stdio::piped());

        if let Some(dir) = self.running_dir {
            let dir = expand_tilde(dir).unwrap();
            let dir = std::fs::canonicalize(dir)?;
            command_runner = command_runner.current_dir(dbg!(dir));
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
    pub state: ListState,
    _child: Child,
}

impl Command {
    pub fn new(name: String, receiver: Receiver<String>, child: Child) -> Self {
        Self {
            name,
            receiver,
            _child: child,
            lines: Vec::new(),
            state: ListState::default(),
        }
    }

    pub fn populate_lines(&mut self) {
        // TODO: handle error of disconnected https://docs.rs/crossbeam/0.8.2/crossbeam/channel/enum.TryRecvError.html

        if let Ok(line) = self.receiver.try_recv() {
            self.lines.push(line);
            self.state.select(Some(self.lines.len() - 1));
        }
    }

    pub fn draw_info(&mut self) -> (&mut ListState, Vec<ListItem>) {
        let lines = self
            .lines
            .iter()
            .map(|line| ListItem::new(line.clone()))
            .collect();

        (&mut self.state, lines)
    }
}

impl Drop for Command {
    fn drop(&mut self) {
        self._child.kill().expect("sad")
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

    pub fn get_selected(&mut self) -> &mut Command {
        // TODO: throw errors here
        let selected = self
            .state
            .selected()
            .expect("a command must be selected at all times");

        self.commands
            .get_mut(selected)
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
