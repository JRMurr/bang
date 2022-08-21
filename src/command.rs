use crossbeam::channel::{bounded, Receiver, Sender};
use log::trace;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    process::Stdio,
    slice::Iter,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncRead, BufReader},
    process::{Child, Command as CommandRunner},
};
use tracing::instrument;

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    async fn read_io<R: AsyncRead + std::marker::Unpin>(
        reader: R,
        sender: Sender<String>,
    ) {
        let mut f = BufReader::new(reader);
        loop {
            let mut buf = String::new();
            match f.read_line(&mut buf).await {
                Ok(data_read) => {
                    if let Err(_e) = sender.send(buf) {
                        // disconnected. Right now only happens on exit so
                        // probably fine to ignore
                        // dbg!(e);
                        break;
                    }
                    if data_read == 0 {
                        // hit eof
                        break;
                    }
                }
                Err(e) => trace!("an error!: {:?}", e),
            }
        }
    }

    pub fn run(&self, config_dir: &PathBuf) -> crate::Result<Command> {
        let command = shell_words::split(&self.command)?;

        // TODO: add errors for parsing
        let program = command.first().expect("Command should not be empty");
        let args = &command[1..];
        // TODO: should we not always be realative to the config file?
        let running_dir = match &self.running_dir {
            Some(dir) => {
                let dir = expand_tilde(dir).unwrap();
                let running_dir = std::fs::canonicalize(dir)?;
                // join the path with the config dir so relative paths make
                // sense
                Path::new(config_dir).join(running_dir)
            }
            None => Path::new(config_dir).to_path_buf(),
        };

        let mut binding = CommandRunner::new(program);
        let mut child = binding
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .current_dir(std::fs::canonicalize(running_dir)?)
            .kill_on_drop(true)
            .spawn()
            .unwrap_or_else(|_| panic!("failed to run {}", self.command));

        let name = self.name.as_ref().unwrap_or(&self.command);

        let (sender, receiver) = bounded::<String>(100);
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let err_sender = sender.clone();
        // TODO: might be good to switch to tokio async tasks
        tokio::spawn(async move {
            Self::read_io(stdout, sender).await;
        });
        tokio::spawn(async move {
            Self::read_io(stderr, err_sender).await;
        });

        Ok(Command::new(name.clone(), receiver, child, self.to_owned()))
    }
}

#[derive(Debug)]
pub struct Command {
    pub name: String,
    receiver: Receiver<String>,
    lines: Vec<String>,
    // pub state: ListState,
    child: Child,

    builder: CommandBuilder,
}

impl Command {
    pub fn new(
        name: String,
        receiver: Receiver<String>,
        child: Child,
        builder: CommandBuilder,
    ) -> Self {
        Self {
            name,
            receiver,
            child,
            builder,
            lines: Vec::with_capacity(100),
            // state: ListState::default(),
        }
    }

    #[allow(dead_code)]
    pub fn restart(&mut self, config_dir: &PathBuf) -> crate::Result<()> {
        let new = self.builder.run(config_dir)?;
        let old = std::mem::replace(self, new);
        // TODO: not sure if this is needed
        std::mem::drop(old);
        Ok(())
    }

    pub fn populate_lines(&mut self) {
        let mut new_lines: Vec<String> = self.receiver.try_iter().collect();
        // let cursor_auto_scroll = self
        //     .state
        //     .selected()
        //     .map_or(true, |curr| curr >= self.lines.len() - 1);
        if !new_lines.is_empty() {
            self.lines.append(&mut new_lines);
            // if cursor_auto_scroll {
            //     self.state.select(Some(self.lines.len() - 1));
            // }
        }
    }

    // pub fn scroll(&mut self, dir: ScrollDirection, amount: usize) {
    //     if let Some(curr) = self.state.selected() {
    //         let new_pos = match dir {
    //             ScrollDirection::Up => curr.saturating_sub(amount),
    //             ScrollDirection::Down => {
    //                 std::cmp::min(curr + amount, self.lines.len())
    //             }
    //         };
    //         // TODO: this only selects the previous line so they need to go
    //         // all the way to the top for the screen to scroll
    //         // might need to fork the list wigit to update the logic
    //         self.state.select(Some(new_pos));
    //     }
    // }

    // pub fn draw_info(&mut self) -> (&mut ListState, Vec<ListItem>) {
    //     let lines = self
    //         .lines
    //         .iter()
    //         .map(|line| ListItem::new(line.clone()))
    //         .collect();

    //     (&mut self.state, lines)
    // }

    #[allow(dead_code)]
    pub async fn kill(&mut self) -> crate::Result<()> {
        if let Err(e) = self.child.kill().await {
            // InvalidInput when child already killed
            if e.kind() != std::io::ErrorKind::InvalidInput {
                return Err(Box::new(e));
            }
        }
        self.child.wait().await?;
        Ok(())
    }
}

// impl Drop for Command {
//     fn drop(&mut self) {
//         let _ = self.kill();
//     }
// }

#[derive(Debug, Default)]
pub struct CommandManager {
    commands: Vec<Command>,
    // state: ListState,
}

impl CommandManager {
    pub fn add_command(&mut self, command: Command) -> crate::Result<()> {
        self.commands.push(command);

        Ok(())
    }

    #[instrument]
    pub fn poll_commands(&mut self) {
        self.commands
            .iter_mut()
            .for_each(|command| command.populate_lines());
    }

    #[allow(dead_code)]
    pub fn iter(&self) -> Iter<Command> {
        self.commands.iter()
    }

    // pub fn select(&mut self, idx: usize) {
    //     self.state.select(Some(idx));
    // }

    // pub fn state(&mut self) -> &mut ListState {
    //     &mut self.state
    // }

    // pub fn get_selected(&mut self) -> &mut Command {
    //     // TODO: throw errors here
    //     let selected = self
    //         .state
    //         .selected()
    //         .expect("a command must be selected at all times");

    //     self.commands
    //         .get_mut(selected)
    //         .expect("selected command must be in list")
    // }

    // pub fn next(&mut self) {
    //     let i = match self.state.selected() {
    //         Some(i) => {
    //             if i >= self.commands.len() - 1 {
    //                 0
    //             } else {
    //                 i + 1
    //             }
    //         }
    //         None => 0,
    //     };
    //     self.state.select(Some(i));
    // }

    // pub fn previous(&mut self) {
    //     let i = match self.state.selected() {
    //         Some(i) => {
    //             if i == 0 {
    //                 self.commands.len() - 1
    //             } else {
    //                 i - 1
    //             }
    //         }
    //         None => 0,
    //     };
    //     self.state.select(Some(i));
    // }
}
