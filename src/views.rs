use cursive::{
    view::{ScrollStrategy, SizeConstraint},
    views::{Dialog, ResizedView, ScrollView, TextView},
    Cursive,
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

pub fn set_help_menu(s: &mut Cursive) {
    let dialog = Dialog::around(TextView::new("Some\tHelp"))
        .button("Exit help", |s| {
            s.pop_layer();
        })
        .title("Help");
    s.add_layer(dialog);
}
