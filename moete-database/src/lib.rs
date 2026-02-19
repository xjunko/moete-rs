use std::sync::Arc;

use sqlx::postgres;

pub use self::models::*;

mod models;

#[derive(Debug, Clone)]
pub struct Database(Arc<postgres::PgPool>);

impl Database {
    pub async fn connect(url: &str) -> Result<Self, sqlx::Error> {
        let pool = postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await?;

        Ok(Database(Arc::new(pool)))
    }

    // Operations
    pub async fn build(&self) -> Result<(), sqlx::Error> {
        models::build(self.0.as_ref()).await?;
        Ok(())
    }

    // Counter API
    pub async fn get_counters(
        &self,
        word: &str,
    ) -> Result<Vec<models::counter::Counter>, sqlx::Error> {
        models::counter::get_counters(self.0.as_ref(), word).await
    }

    pub async fn increment_counter(&self, user_id: i64, word: &str) -> Result<(), sqlx::Error> {
        models::counter::increment_word_for_user_id(self.0.as_ref(), user_id, word).await
    }

    // Markov API
    pub async fn add_message(&self, user_id: i64, content: &str) -> Result<(), sqlx::Error> {
        models::markov::add_message(self.0.as_ref(), user_id, content).await
    }

    pub async fn get_user(
        &self,
        user_id: i64,
    ) -> Result<Option<models::markov::MarkovUser>, sqlx::Error> {
        models::markov::get_user(self.0.as_ref(), user_id).await
    }

    pub async fn get_user_count(&self, user_id: i64) -> Result<Option<i64>, sqlx::Error> {
        models::markov::get_user_count(self.0.as_ref(), user_id).await
    }

    // Shortcut API
    pub async fn add_shortcut(
        &self,
        guild_id: i64,
        trigger: &str,
        response: &str,
        cache: Arc<models::shortcut::ShortcutCache>,
    ) -> Result<(), sqlx::Error> {
        models::shortcut::add_shortcut(self.0.as_ref(), guild_id, trigger, response, &cache).await
    }

    pub async fn remove_shortcut(
        &self,
        guild_id: i64,
        trigger: &str,
        cache: Arc<models::shortcut::ShortcutCache>,
    ) -> Result<bool, sqlx::Error> {
        models::shortcut::remove_shortcut(self.0.as_ref(), guild_id, trigger, &cache).await
    }

    pub async fn edit_shortcut(
        &self,
        guild_id: i64,
        trigger: &str,
        new_response: &str,
        cache: Arc<models::shortcut::ShortcutCache>,
    ) -> Result<bool, sqlx::Error> {
        models::shortcut::edit_shortcut(self.0.as_ref(), guild_id, trigger, new_response, &cache)
            .await
    }

    pub async fn get_shortcut(
        &self,
        guild_id: i64,
        name: &str,
    ) -> Result<Option<models::shortcut::Shortcut>, sqlx::Error> {
        models::shortcut::get_shortcut(self.0.as_ref(), guild_id, name).await
    }

    pub async fn get_all_shortcuts(
        &self,
        guild_id: i64,
    ) -> Result<Vec<models::shortcut::Shortcut>, sqlx::Error> {
        models::shortcut::get_all_shortcuts_for_guild(self.0.as_ref(), guild_id).await
    }
}
