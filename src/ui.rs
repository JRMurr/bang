use std::io::Write;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::command::CommandManager;

pub fn draw(
    frame: &mut Frame<CrosstermBackend<impl Write>>,
    commands: &mut CommandManager,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [Constraint::Percentage(10), Constraint::Percentage(90)].as_ref(),
        )
        .split(frame.size());

    let list_chunk = chunks[0];
    let main = chunks[1];

    let list_output = List::new(
        commands
            .iter()
            .map(|c| ListItem::new(c.name.clone()))
            .collect::<Vec<_>>(),
    )
    .block(Block::default().title("List").borders(Borders::ALL))
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
    .highlight_symbol(">>");

    frame.render_stateful_widget(list_output, list_chunk, commands.state());

    let output = Paragraph::new(commands.get_selected().spans()).block(
        Block::default()
            // .title(commands.commands[0].name.clone())
            .borders(Borders::ALL),
    );
    frame.render_widget(output, main);

    // for (command, chunk) in commands.commands.iter().zip(chunks.iter()) {
    //     let output = Paragraph::new(command.spans()).block(
    //         Block::default()
    //             .title(command.name.clone())
    //             .borders(Borders::ALL),
    //     );
    //     frame.render_widget(output, *chunk);
    // }
}
