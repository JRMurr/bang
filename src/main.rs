use bang::{application::Application, cli::Cli};
use clap::Parser;

fn main() {
    let args = Cli::parse();
    let mut app = Application::new(args.config).expect("Error making app");

    if let Err(e) = app.run(std::io::stdout()) {
        // app is now dropped we can print to stderr safely
        eprintln!("sad: {}", e);
    };
}
