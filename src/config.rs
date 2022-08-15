use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::command::CommandBuilder;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub commands: Vec<CommandBuilder>,
}

#[derive(Debug)]

pub struct Config {
    pub config: ConfigFile,
    pub directory: PathBuf,
}

impl Config {
    pub fn read(config_location: Option<PathBuf>) -> anyhow::Result<Self> {
        let config_location = match config_location {
            Some(c) => c,
            None => {
                // TODO: maybe we `config_dir` on non macos
                let mut home_path = dirs::home_dir().unwrap();
                home_path.push("./.config/bang/bang.toml");
                home_path
            }
        };

        Self::create_or_get_config(config_location)
    }

    pub fn create_or_get_config(config_path: PathBuf) -> anyhow::Result<Self> {
        if let Ok(config_string) = std::fs::read_to_string(&config_path) {
            let config_file: ConfigFile =
                toml::from_str(config_string.as_str())?;

            let mut dir = std::fs::canonicalize(&config_path)?;
            dir.pop(); // just get the directory
            Ok(Self {
                config: config_file,
                directory: dir,
            })
        } else {
            anyhow::bail!("config not found at {}", config_path.display());
            // // Config file DNE...
            // if let Some(parent_path) = path.parent() {
            //     fs::create_dir_all(parent_path)?;
            // }
            // // fs::File::create(path)?.write_all(CONFIG_TOP_HEAD.as_bytes())?
            // ; fs::File::create(path)?.write_all(CONFIG_TEXT.
            // as_bytes())?; Ok(Config::default())
        }
    }
}
