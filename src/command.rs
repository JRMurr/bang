use crossbeam::channel::Sender;
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    thread,
};

pub fn run_command(sender: Sender<String>) {
    let child = Command::new("ping")
        .arg("google.com")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start ping process");

    // println!("Started process: {}", child.id());

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
}
