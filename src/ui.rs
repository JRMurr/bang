use std::{io::Write, vec};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Row, Table},
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

    let side_bar = chunks[0];
    let main = chunks[1];

    draw_side_bar(frame, commands, side_bar);

    draw_command_output(frame, commands.get_selected(), main);
}

pub fn draw_help_menu(frame: &mut Frame<CrosstermBackend<impl Write>>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(15),
                Constraint::Percentage(70),
                Constraint::Percentage(15),
            ]
            .as_ref(),
        )
        .margin(2)
        .split(frame.size());

    let help_menu_style = Style::default().fg(Color::White);

    let rows = vec![
        ("Select Previous Command", "<Up Arrow>"),
        ("Select next Command", "<Down Arrow>"),
        ("Restart Command", "r"),
        ("Kill Command", "k"),
        ("Quit bang", "q"),
    ];
    let rows = rows.iter().map(|(action, binding)| {
        Row::new(vec![action.to_owned(), binding.to_owned()])
            .style(help_menu_style)
    });

    let help_menu = Table::new(rows)
        .header(Row::new(vec!["Action", "Key"]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(help_menu_style)
                .title(Span::styled(
                    "Help (press <Esc> to go back)",
                    help_menu_style,
                ))
                .border_style(help_menu_style),
        )
        .style(help_menu_style)
        .widths(&[Constraint::Percentage(90), Constraint::Percentage(10)]);
    frame.render_widget(help_menu, chunks[1]);
}

fn draw_side_bar(
    frame: &mut Frame<CrosstermBackend<impl Write>>,
    commands: &mut CommandManager,
    chunk: Rect,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(chunk);

    let help_box_output = Paragraph::new(Text::from("Type ?"))
        .block(Block::default().title("Help").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    frame.render_widget(help_box_output, chunks[0]);

    draw_command_list(frame, commands, chunks[1]);
}

fn draw_command_list(
    frame: &mut Frame<CrosstermBackend<impl Write>>,
    commands: &mut CommandManager,
    chunk: Rect,
) {
    let list_output = List::new(
        commands
            .iter()
            .map(|c| ListItem::new(c.name.clone()))
            .collect::<Vec<_>>(),
    )
    .block(Block::default().borders(Borders::ALL).title("Commands"))
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
    .highlight_symbol(">>");

    frame.render_stateful_widget(list_output, chunk, commands.state());
}

fn draw_command_output(
    frame: &mut Frame<CrosstermBackend<impl Write>>,
    command: &mut Command,
    chunk: Rect,
) {
    let (state, list) = command.draw_info();

    let output = List::new(list)
        .block(Block::default().borders(Borders::ALL).title("Output"));
    frame.render_stateful_widget(output, chunk, state);
}
