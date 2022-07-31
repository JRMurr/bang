use serde::{Deserialize, Serialize};

use crate::command::CommandBuilder;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub commands: Vec<CommandBuilder>,
}

impl Config {
    pub fn read() -> anyhow::Result<Self> {
        let data = std::fs::read_to_string("./config.toml")
            .expect("Unable to read file");
        Ok(toml::from_str(data.as_str())?)
    }
}
