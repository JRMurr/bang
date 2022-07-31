use std::io::Write;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::command::{Command, CommandManager};

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
    .block(Block::default().borders(Borders::ALL))
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
    .highlight_symbol(">>");

    frame.render_stateful_widget(list_output, list_chunk, commands.state());

    draw_command_output(frame, commands.get_selected(), main);
}

fn draw_command_output(
    frame: &mut Frame<CrosstermBackend<impl Write>>,
    command: &mut Command,
    chunk: Rect,
) {
    let (state, list) = command.draw_info();

    let output = List::new(list).block(Block::default().borders(Borders::ALL));
    frame.render_stateful_widget(output, chunk, state);
}
