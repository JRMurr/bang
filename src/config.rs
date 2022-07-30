use serde::{Deserialize, Serialize};

use crate::command::CommandBuilder;

#[derive(Serialize, Deserialize, Debug)]
struct Config<'a> {
    #[serde(borrow)]
    commands: Vec<CommandBuilder<'a>>,
}
