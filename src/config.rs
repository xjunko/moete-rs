use std::path::Path;

use super::Error;
use serde::Deserialize;
use serde::Serialize;

use log::error;

static CONFIG_PATH: &str = "config.toml";

#[derive(Serialize, Deserialize)]
pub struct Config {
    discord: Discord,
}

#[derive(Serialize, Deserialize)]
pub struct Discord {
    name: String,
    status: Vec<String>,
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
            panic!("Fill out the config with the correct values");
        }

        let conf_content = std::fs::read_to_string(conf_path).expect("Failed to read config.toml");
        let config: Config = toml::from_str(&conf_content).unwrap();
        config
    }

    fn create_default() -> Self {
        Self {
            discord: Discord {
                name: String::from("Moete"),
                status: vec!["hello world!".into()],
            },
        }
    }
}
