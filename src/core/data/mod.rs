use super::Config;
use crate::serenity;

pub struct Data {
    pub config: Config,
}

impl Data {
    pub fn create() -> Self {
        let config = Config::load();
        Self { config }
    }

    pub fn load(&mut self, _ctx: &serenity::Context) {}
}
