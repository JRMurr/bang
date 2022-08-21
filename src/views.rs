use cursive::{
    view::{ScrollStrategy, SizeConstraint},
    views::{ResizedView, ScrollView, TextView},
};

use crate::command::Command;

type CommandView = ResizedView<ScrollView<TextView>>;

pub fn get_command_view(command: &Command) -> CommandView {
    let content = command.content.clone();
    let text = TextView::new_with_content(content);

    let scroll_view =
        ScrollView::new(text).scroll_strategy(ScrollStrategy::StickToBottom);

    ResizedView::new(SizeConstraint::Full, SizeConstraint::Full, scroll_view)
}

// pub struct Test {
//     command: Command,
//     view: CommandView,
// }

// impl Test {
//     pub fn new(command: Command) -> Self {
//         let view = get_command_view(&command);
//         Self { command, view }
//     }

//     pub fn name(&self) -> String {
//         self.command.name.clone()
//     }
// }

// impl View for Test {
//     fn draw(&self, printer: &cursive::Printer) {
//         self.view.draw(printer);
//     }
// }