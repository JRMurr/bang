use crossterm::{
    terminal::{self},
    ExecutableCommand,
};
use std::io::Write;
use tui::{
    backend::{Backend, CrosstermBackend},
    text::Spans,
    widgets::Paragraph,
    Frame, Terminal,
};

pub struct Renderer<W: Write> {
    terminal: Terminal<CrosstermBackend<W>>,
}

impl<W: Write> Renderer<W> {
    pub fn new(mut out: W) -> crate::Result<Renderer<W>> {
        terminal::enable_raw_mode()?;
        out.execute(terminal::EnterAlternateScreen)?;

        Ok(Renderer {
            terminal: Terminal::new(CrosstermBackend::new(out))?,
        })
    }

    pub fn render(&mut self, lines: Vec<Spans>) -> crate::Result<()> {
        self.terminal.draw(|frame| ui(frame, lines))?;
        Ok(())
    }
}

impl<W: Write> Drop for Renderer<W> {
    fn drop(&mut self) {
        self.terminal
            .backend_mut()
            .execute(terminal::LeaveAlternateScreen)
            .expect("Could not execute to stdout");
        terminal::disable_raw_mode()
            .expect("Terminal doesn't support to disable raw mode");
        if std::thread::panicking() {
            eprintln!(
                "bang panicked, to log the error you can redirect stderr to a file, example: bang 2> bang_log",
            );
        }
    }
}

fn ui<B: Backend>(frame: &mut Frame<B>, lines: Vec<Spans>) {
    let output = Paragraph::new(lines);

    frame.render_widget(output, frame.size());
}
