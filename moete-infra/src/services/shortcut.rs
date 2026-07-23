use std::sync::Arc;

use dashmap::DashMap;

use crate::repositories;
use crate::shortcut::Shortcut;

#[derive(Default, Debug)]
pub struct ShortcutCache {
    data: DashMap<i64, Arc<Vec<Shortcut>>>,
}

impl ShortcutCache {
    pub fn get(&self, guild_id: i64) -> Option<Arc<Vec<Shortcut>>> {
        self.data.get(&guild_id).map(|v| v.clone())
    }

    pub fn insert(&self, guild_id: i64, shortcuts: Vec<Shortcut>) {
        self.data.insert(guild_id, Arc::new(shortcuts));
    }

    pub fn remove(&self, guild_id: i64) { self.data.remove(&guild_id); }
}

pub async fn add(
    pool: &sqlx::postgres::PgPool,
    guild_id: i64,
    trigger: &str,
    response: &str,
    cache: &ShortcutCache,
) -> Result<(), sqlx::Error> {
    repositories::shortcut::create(pool, guild_id, trigger, response).await?;
    cache.remove(guild_id);
    Ok(())
}

pub async fn remove(
    pool: &sqlx::postgres::PgPool,
    guild_id: i64,
    trigger: &str,
    cache: &ShortcutCache,
) -> Result<bool, sqlx::Error> {
    let affected =
        repositories::shortcut::remove(pool, guild_id, trigger).await?;
    if affected > 0 {
        cache.remove(guild_id);
        return Ok(true);
    }
    Ok(false)
}

pub async fn edit(
    pool: &sqlx::postgres::PgPool,
    guild_id: i64,
    trigger: &str,
    new_response: &str,
    cache: &ShortcutCache,
) -> Result<bool, sqlx::Error> {
    // check if the shortcut exists
    if repositories::shortcut::find(pool, guild_id, trigger).await?.is_some() {
        repositories::shortcut::update(pool, guild_id, trigger, new_response)
            .await?;
        cache.remove(guild_id);
        Ok(true)
    } else {
        repositories::shortcut::create(pool, guild_id, trigger, new_response)
            .await?;
        cache.remove(guild_id);
        Ok(true)
    }
}

pub async fn get(
    pool: &sqlx::postgres::PgPool,
    guild_id: i64,
    trigger: &str,
) -> Result<Option<Shortcut>, sqlx::Error> {
    repositories::shortcut::find(pool, guild_id, trigger).await
}

pub async fn get_all(
    pool: &sqlx::postgres::PgPool,
    guild_id: i64,
) -> Result<Vec<Shortcut>, sqlx::Error> {
    repositories::shortcut::find_all(pool, guild_id).await
}
