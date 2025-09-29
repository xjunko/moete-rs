use sqlx::postgres;
use std::sync::Arc;
use tokio::sync::Mutex;

use moete_ext::Currencies;

use super::{Config, EmoteManager};
use crate::{MoeteError, serenity};

#[derive(Debug)]
pub struct State {
    pub config: Arc<Config>,
    pub emotes: EmoteManager,
    pub pool: Arc<Option<postgres::PgPool>>,
    pub currency: Mutex<Currencies>,
}

impl State {
    pub fn create() -> Self {
        Self {
            config: Arc::new(Config::default()),
            emotes: EmoteManager::new(),
            pool: Arc::new(None),
            currency: Mutex::new(Currencies::new()),
        }
    }

    pub async fn load(&mut self, ctx: &serenity::Context) -> Result<(), MoeteError> {
        self.pool = Arc::new(Some(
            postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect(&self.config.services.database)
                .await?,
        ));
        self.emotes.load(ctx, Arc::clone(&self.config)).await;

        let mut currency = self.currency.lock().await;
        currency.load().await?;

        Ok(())
    }
}
