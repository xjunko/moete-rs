use crate::{serenity, state::Config};

pub struct State {
    pub config: Config,
}

impl State {
    pub fn create() -> Self {
        let config = Config::load();
        Self { config }
    }

    pub fn load(&mut self, _: &serenity::Context) {}
}
