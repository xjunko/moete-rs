use std::sync::Arc;

use moete_ext::Currencies;
use moete_infra::services::shortcut::ShortcutCache;
use sqlx::postgres;
use tokio::sync::Mutex;
use tracing::error;

use super::{
    Config,
    EmoteManager,
};
use crate::{
    MoeteError,
    serenity,
};

#[derive(Debug)]
pub struct State {
    started_at: std::time::Instant,

    pub config: Arc<Config>,
    pub emotes: Arc<Mutex<EmoteManager>>,
    pub database: Option<Arc<postgres::PgPool>>,
    pub currency: Arc<Mutex<Currencies>>,

    pub shortcut_cache: Arc<ShortcutCache>,
}

impl Clone for State {
    fn clone(&self) -> Self {
        Self {
            started_at: self.started_at,
            config: Arc::clone(&self.config),
            emotes: Arc::clone(&self.emotes),
            database: self.database.clone(),
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
            database: None,
            currency: Arc::new(Mutex::new(Currencies::new())),
            shortcut_cache: Arc::new(ShortcutCache::default()),
        }
    }

    pub async fn load(
        &mut self,
        ctx: &serenity::Context,
    ) -> Result<(), MoeteError> {
        self.database = match postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&self.config.services.database)
            .await
        {
            Ok(db) => Some(Arc::new(db)),
            Err(err) => {
                error!(
                    "Failed to connect to database: {}, continuing without database.",
                    err
                );
                None
            },
        };

        self.emotes.lock().await.load(ctx, Arc::clone(&self.config)).await;

        Ok(())
    }

    pub fn uptime(&self) -> std::time::Duration { self.started_at.elapsed() }
}
