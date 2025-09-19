use std::sync::Arc;

use super::{Config, EmoteManager};
use crate::{Error, serenity};

pub struct State {
    pub config: Arc<Config>,
    pub emotes: EmoteManager,
}

impl State {
    pub fn create() -> Self {
        Self {
            config: Arc::new(Config::default()),
            emotes: EmoteManager::new(),
        }
    }

    pub async fn load(&mut self, ctx: &serenity::Context) -> Result<(), Error> {
        self.emotes.load(ctx).await;
        Ok(())
    }
}
