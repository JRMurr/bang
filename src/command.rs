use crossbeam::channel::{unbounded, Receiver};
use std::{
    io::{BufRead, BufReader},
    process::{Command as CommandRunner, Stdio},
    thread,
};
use tui::text::{Span, Spans};

pub struct Command {
    command: String,
    receiver: Option<Receiver<String>>,
    lines: Vec<String>,
}

impl Command {
    pub fn new(command: String) -> Self {
        Self {
            command,
            receiver: None,
            lines: Vec::new(),
        }
    }

    pub fn run(&mut self) -> crate::Result<()> {
        let command = shell_words::split(&self.command)?;

        // TODO: add errors for parsing
        let program = command.first().expect("Command should not be empty");
        let args = &command[1..];
        let child = CommandRunner::new(program)
            .args(args)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap_or_else(|_| panic!("failed to run {}", self.command));
        let (sender, receiver) = unbounded::<String>();

        self.receiver = Some(receiver);

        thread::spawn(move || {
            let mut f = BufReader::new(child.stdout.unwrap());
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
        Ok(())
    }

    pub fn populate_lines(&mut self) {
        let receiver =
            self.receiver.as_ref().expect("command has not been run");

        // TODO: handle error of disconnected https://docs.rs/crossbeam/0.8.2/crossbeam/channel/enum.TryRecvError.html

        if let Ok(line) = receiver.try_recv() {
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
