use std::sync::Arc;

use dashmap::DashMap;
use sqlx::postgres;
use tracing::info;

#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
pub struct Shortcut {
    pub id: i64,
    pub guild_id: i64,
    pub trigger: String,
    pub response: String,
}

impl Shortcut {
    pub fn responses(&self) -> Vec<String> {
        self.response.split(",").map(|s| s.to_string()).collect()
    }
}

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

    pub fn remove(&self, guild_id: i64) {
        self.data.remove(&guild_id);
    }
}

pub async fn build(pool: &postgres::PgPool) -> Result<(), sqlx::Error> {
    let res_shortcuts = sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS shortcuts (
            id BIGSERIAL PRIMARY KEY NOT NULL,
            guild_id INT8 NOT NULL,
            trigger TEXT NOT NULL,
            response TEXT NOT NULL,
            CONSTRAINT unique_trigger_per_guild UNIQUE (guild_id, trigger)
        );
        "#,
    )
    .execute(pool)
    .await?;

    match res_shortcuts.rows_affected() {
        0 => info!("Table 'shortcuts' already exists."),
        _ => info!("Created table 'shortcuts'."),
    }

    Ok(())
}

pub async fn get_shortcut(
    pool: &postgres::PgPool,
    guild_id: i64,
    trigger: &str,
) -> Result<Option<Shortcut>, sqlx::Error> {
    let shortcut: Option<Shortcut> = sqlx::query_as::<_, Shortcut>(
        "SELECT id, guild_id, trigger, response FROM shortcuts WHERE guild_id = $1 AND trigger = $2",
    ).bind(guild_id).bind(trigger).fetch_optional(pool).await?;
    Ok(shortcut)
}

pub async fn get_all_shortcuts_for_guild(
    pool: &postgres::PgPool,
    guild_id: i64,
) -> Result<Vec<Shortcut>, sqlx::Error> {
    let shortcuts: Vec<Shortcut> = sqlx::query_as::<_, Shortcut>(
        "SELECT id, guild_id, trigger, response FROM shortcuts WHERE guild_id = $1",
    )
    .bind(guild_id)
    .fetch_all(pool)
    .await?;
    Ok(shortcuts)
}

pub async fn remove_shortcut(
    pool: &postgres::PgPool,
    guild_id: i64,
    trigger: &str,
    cache: &ShortcutCache,
) -> Result<bool, sqlx::Error> {
    let res = sqlx::query("DELETE FROM shortcuts WHERE guild_id = $1 AND trigger = $2")
        .bind(guild_id)
        .bind(trigger)
        .execute(pool)
        .await?;

    if res.rows_affected() > 0 {
        cache.remove(guild_id);
        return Ok(true);
    }

    Ok(false)
}

pub async fn add_shortcut(
    pool: &postgres::PgPool,
    guild_id: i64,
    trigger: &str,
    response: &str,
    cache: &ShortcutCache,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO shortcuts (guild_id, trigger, response) VALUES ($1, $2, $3)")
        .bind(guild_id)
        .bind(trigger)
        .bind(response)
        .execute(pool)
        .await?;
    cache.remove(guild_id);

    Ok(())
}

pub async fn edit_shortcut(
    pool: &postgres::PgPool,
    guild_id: i64,
    trigger: &str,
    new_response: &str,
    cache: &ShortcutCache,
) -> Result<bool, sqlx::Error> {
    // abstract away the addition if it doesn't exist
    {
        if get_shortcut(pool, guild_id, trigger).await?.is_none() {
            add_shortcut(pool, guild_id, trigger, new_response, cache).await?;
            return Ok(true);
        }
    }

    let res =
        sqlx::query("UPDATE shortcuts SET response = $1 WHERE guild_id = $2 AND trigger = $3")
            .bind(new_response)
            .bind(guild_id)
            .bind(trigger)
            .execute(pool)
            .await?;

    if res.rows_affected() > 0 {
        cache.remove(guild_id);
        return Ok(true);
    }

    Ok(false)
}

pub async fn get_guild_ids(pool: &postgres::PgPool) -> Result<Vec<i64>, sqlx::Error> {
    let guild_ids: Vec<(i64,)> = sqlx::query_as("SELECT DISTINCT guild_id FROM shortcuts")
        .fetch_all(pool)
        .await?;

    Ok(guild_ids.into_iter().map(|(id,)| id).collect())
}
