use bang::{application::Application, cli::Cli};
use clap::Parser;

async fn run() -> bang::Result<()> {
    // TODO: probably feature flag this
    fern::Dispatch::new()
        .format(|out, message, record| {
            let offset = time::OffsetDateTime::now_utc();

            out.finish(format_args!(
                "{}[{}][{}] {}",
                offset
                    .format(&time::macros::format_description!(
                        // The weird "[[[" is because we need to escape a bracket ("[[") to show one "[".
                        // See https://time-rs.github.io/book/api/format-description.html
                        "[[[year]-[month]-[day]][[[hour]:[minute]:[second][subsecond digits:9]]"
                    ))
                    .unwrap(),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(fern::log_file("bang.log")?).apply()?;
    let args = Cli::parse();
    let mut app = Application::new(args.config)?;

    app.run(std::io::stdout()).await
}

#[tokio::main]
async fn main() {
    console_subscriber::init();
    if let Err(e) = run().await {
        eprintln!("sad: {}", e);
    };
}
