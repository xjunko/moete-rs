use std::sync::Arc;

use moete_ext::Currencies;
use sqlx::postgres;
use tokio::sync::Mutex;
use tracing::error;

use super::{Config, EmoteManager};
use crate::{MoeteError, serenity};

#[derive(Debug)]
pub struct State {
    started_at: std::time::Instant,

    pub config: Arc<Config>,
    pub emotes: Arc<Mutex<EmoteManager>>,
    pub pool: Arc<Option<postgres::PgPool>>,
    pub currency: Arc<Mutex<Currencies>>,

    pub shortcut_cache: Arc<moete_database::shortcut::ShortcutCache>,
}

impl Clone for State {
    fn clone(&self) -> Self {
        Self {
            started_at: self.started_at,
            config: Arc::clone(&self.config),
            emotes: Arc::clone(&self.emotes),
            pool: Arc::clone(&self.pool),
            currency: Arc::clone(&self.currency),
            shortcut_cache: Arc::clone(&self.shortcut_cache),
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
            shortcut_cache: Arc::new(moete_database::shortcut::ShortcutCache::default()),
        }
    }

    pub async fn load(&mut self, ctx: &serenity::Context) -> Result<(), MoeteError> {
        self.pool = Arc::new({
            let pool_res = postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect(&self.config.services.database)
                .await
                .ok();

            if pool_res.is_none() {
                error!("Failed to connect to database, continuing without database.");
                None
            } else {
                pool_res
            }
        });

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
