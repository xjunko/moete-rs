use sqlx::postgres;
use std::sync::Arc;
use tokio::sync::Mutex;

use moete_ext::Currencies;

use super::{Config, EmoteManager};
use crate::{MoeteError, serenity};

#[derive(Debug)]
pub struct State {
    started_at: std::time::Instant,

    pub config: Arc<Config>,
    pub emotes: Arc<Mutex<EmoteManager>>,
    pub pool: Arc<Option<postgres::PgPool>>,
    pub currency: Arc<Mutex<Currencies>>,
}

impl Clone for State {
    fn clone(&self) -> Self {
        Self {
            started_at: self.started_at,
            config: Arc::clone(&self.config),
            emotes: Arc::clone(&self.emotes),
            pool: Arc::clone(&self.pool),
            currency: Arc::clone(&self.currency),
        }
    }
}

impl State {
    pub fn create() -> Self {
        Self {
            started_at: std::time::Instant::now(),

            config: Arc::new(Config::default()),
            emotes: Arc::new(Mutex::new(EmoteManager::new())),
            pool: Arc::new(None),
            currency: Arc::new(Mutex::new(Currencies::new())),
        }
    }

    pub async fn load(&mut self, ctx: &serenity::Context) -> Result<(), MoeteError> {
        self.pool = Arc::new(Some(
            postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect(&self.config.services.database)
                .await?,
        ));

        self.emotes
            .lock()
            .await
            .load(ctx, Arc::clone(&self.config))
            .await;

        let mut currency = self.currency.lock().await;
        currency.load().await?;

        Ok(())
    }

    pub fn uptime(&self) -> std::time::Duration {
        self.started_at.elapsed()
    }
}
