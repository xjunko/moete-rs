use sqlx::postgres;
use std::sync::Arc;

use super::{Config, EmoteManager};
use crate::{Error, serenity};

#[derive(Debug)]
pub struct State {
    pub config: Arc<Config>,
    pub emotes: EmoteManager,
    pub pool: Arc<Option<postgres::PgPool>>,
}

impl State {
    pub fn create() -> Self {
        Self {
            config: Arc::new(Config::default()),
            emotes: EmoteManager::new(),
            pool: Arc::new(None),
        }
    }

    pub async fn load(&mut self, ctx: &serenity::Context) -> Result<(), Error> {
        self.pool = Arc::new(Some(
            postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect("postgresql://moete:1442@localhost/moete")
                .await?,
        ));

        self.emotes.load(ctx, Arc::clone(&self.config)).await;
        Ok(())
    }
}
