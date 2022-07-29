use bang::application::Application;

fn main() {
    let mut app = Application {};

    if let Err(e) = app.run(std::io::stdout()) {
        // app is now dropped we can print to stderr safely
        eprintln!("sad: {}", e);
    };
}
