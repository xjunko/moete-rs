use std::path::Path;

use crate::Error;
use log::info;
use serde::Deserialize;
use serde::Serialize;

use log::error;

static CONFIG_PATH: &str = "config.toml";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub discord: Discord,
}

#[derive(Serialize, Deserialize)]
pub struct Discord {
    pub name: String,
    pub token: String,
    pub status: Vec<String>,
    pub debug: bool,
}

impl Config {
    pub fn save(&self) -> Result<(), Error> {
        let conf_path = Path::new(CONFIG_PATH);
        std::fs::write(conf_path, toml::to_string(self).unwrap())?;
        Ok(())
    }

    pub fn load() -> Self {
        let conf_path = Path::new(CONFIG_PATH);

        if !conf_path.exists() {
            let default_config = Config::create_default();
            if let Err(reason) = default_config.save() {
                error!("Failed to save file: {reason:?}");
            }
            info!("Fill out the config with the correct values");
            std::process::exit(0);
        }

        let conf_content = std::fs::read_to_string(conf_path).expect("Failed to read config.toml");
        let config: Config = toml::from_str(&conf_content).unwrap();
        config
    }

    fn create_default() -> Self {
        Self {
            discord: Discord {
                name: String::from("Moete"),
                token: String::from("DISCORD_TOKEN"),
                status: vec!["hello world!".into()],
                debug: false,
            },
        }
    }
}
