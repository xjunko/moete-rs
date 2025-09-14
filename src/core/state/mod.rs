use super::{Config, EmoteManager};
use crate::{Error, serenity};

pub struct State {
    pub config: Config,
    pub emotes: EmoteManager,
}

impl State {
    pub fn create() -> Self {
        Self {
            config: Config::load(),
            emotes: EmoteManager::new(),
        }
    }

    pub async fn load(&mut self, ctx: &serenity::Context) -> Result<(), Error> {
        self.emotes.load(ctx).await;
        Ok(())
    }
}
