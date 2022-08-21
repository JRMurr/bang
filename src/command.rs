use cursive::views::TextContent;
use derivative::Derivative;
use log::trace;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    process::Stdio,
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
        content: TextContent,
    ) {
        let mut f = BufReader::new(reader);
        loop {
            let mut buf = String::new();
            match f.read_line(&mut buf).await {
                Ok(data_read) => {
                    if data_read == 0 {
                        // hit eof
                        break;
                    }

                    content.append(buf);
                }
                Err(e) => trace!("an error!: {:?}", e),
            }
        }
    }

    #[instrument]
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

        // TODO: cursive-tabs sad with just empty/whitepsace
        let content = TextContent::new("\nstart\n");
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

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let stdout_content = content.clone();
        let err_content = content.clone();
        // TODO: might be good to switch to tokio async tasks
        tokio::spawn(async move {
            Self::read_io(stdout, stdout_content).await;
        });
        tokio::spawn(async move {
            Self::read_io(stderr, err_content).await;
        });

        Ok(Command::new(name.clone(), content, child, self.to_owned()))
    }
}
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Command {
    pub name: String,
    #[derivative(Debug = "ignore")]
    pub content: TextContent,
    child: Child,

    builder: CommandBuilder,
}

impl Command {
    pub fn new(
        name: String,
        content: TextContent,
        child: Child,
        builder: CommandBuilder,
    ) -> Self {
        Self {
            name,
            child,
            builder,
            content,
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

    // pub fn populate_lines(&mut self) {
    //     let new_lines: Vec<String> = self.receiver.try_iter().collect();
    //     if !new_lines.is_empty() {
    //         // TODO: probably need a leading new line
    //         let new_lines_str = new_lines.join("\n");

    //         self.content.append(new_lines_str);
    //     }
    // }

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
