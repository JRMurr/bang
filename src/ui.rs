use std::io::Write;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::command::CommandManager;

pub fn draw(
    frame: &mut Frame<CrosstermBackend<impl Write>>,
    commands: &CommandManager,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Percentage(50)].as_ref())
        .split(frame.size());

    for (command, chunk) in commands.commands.iter().zip(chunks.iter()) {
        let output = Paragraph::new(command.spans()).block(
            Block::default()
                .title(command.name.clone())
                .borders(Borders::ALL),
        );
        frame.render_widget(output, *chunk);
    }
}
