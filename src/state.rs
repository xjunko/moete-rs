use super::config;
use super::serenity;

pub struct State {
    config: config::Config,
}

impl State {
    pub fn load(_: &serenity::Context) -> Self {
        let config = config::Config::load();

        Self { config: config }
    }
}
